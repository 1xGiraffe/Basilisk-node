//      ---_ ......._-_--.        ,adPPYba, 8b,dPPYba,    ,adPPYba,  88   ,d8
//     (|\ /      / /| \  \       I8[    "" 88P'   `"8a  a8P_____88  88 ,a8"
//     /  /     .'  -=-'   `.      `"Y8ba,  88       88  8PP"""""""  8888[
//    /  /    .'             )    aa    ]8I 88       88  "8b,   ,aa  88`"Yba,
//  _/  /   .'        _.)   /     `"YbbdP"' 88       88   `"Ybbd8"'  88   `Y8a
//  / o   o        _.-' /  .'
//  \          _.-'    / .'*|
//  \______.-'//    .'.' \*|      This file is part of Basilisk-node.
//   \|  \ | //   .'.' _ |*|      Built with <3 for decentralisation.
//    `   \|//  .'.'_ _ _|*|
//     .  .// .'.' | _ _ \*|      Copyright (C) 2021-2022  Intergalactic, Limited (GIB).
//     \`-|\_/ /    \ _ _ \*\     SPDX-License-Identifier: Apache-2.0
//      `/'\__/      \ _ _ \*\    Licensed under the Apache License, Version 2.0 (the "License");
//     /^|            \ _ _ \*    you may not use this file except in compliance with the License.
//    '  `             \ _ _ \    http://www.apache.org/licenses/LICENSE-2.0
//     '  `             \ _ _ \
//
//!
//! # Auctions Pallet
//!
//! ## Overview
//!
//! The Auctions pallet provides extendable auction functionality for NFTs.
//!
//! The pallet implements an NftAuction trait which allows users to extend the pallet by implementing other
//! auction types. All auction types must implement the following instance functions at their interface:
//!
//! - `create`
//!
//! - `update`
//!
//! - `bid`
//!
//! - `close`
//!
//! - `validate_data`
//!
//! ## Storage
//!
//! The pallet implements the following stores:
//!
//! - `Auctions` - holds auctions from different types. Auction types are represented in a struct which holds
//! two other structs with common_data (eg auction name, start, end) and specific_data for the given auction type.
//!
//! - `AuctionOwnerById` - index for auction owner by auction id
//!
//! - `NextAuctionId` - index for next auction id
//!
//! - `ReservedAmounts` - store for bid amounts which are reserved until an auction has closed. This enables claim functionality.
//!   Used by Auction::TopUp and Auction::Candle
//!
//! - `HighestBiddersByAuctionClosingRange` - stores the highest bid per closing range (1-100) of an Auction::Candle
//!
//!
//! ## Dispatchable Functions
//!
//! - `create` - create an auction
//!
//! - `update` - update an auction
//!
//! - `destroy` - destroy an auction
//!
//! - `bid` - place a bid on an auctio
//!
//! - `close` - close an auction after the end time has lapsed. Not done in a hook for better chain performance
//!
//! - `claim` - claim assets from reserved amount, after auction has closed
//!
//! ## Implemented Auction types
//!
//! ### Auction::English
//!
//! Implemented in ./types/english.rs
//!
//! In an English auction, participants place bids in a running auction. Once the auction has reached its end time,
//! the highest bid wins and the NFT is transferred to the winner.
//!
//! The amount is reserved by transferring it to the subaccount of the auction. After the auction closes, the bid amount
//! is either transferred to the NFT owner (winning bid), or becomes claimable (losing bid).
//!
//! The implementation of English auction allows sellers to set a reserve price for the NFT
//! (auction.common_data.reserve_price). The reserve_price acts as a minimum starting bid, preventing bidders
//! from placing bids below the reserve_price.
//!
//! When creating an English auction with a reserve_price, auction.common_data.reserve_price must be equal to
//! auction.common_data.next_bid_min.
//!
//! ### Auction::TopUp
//!
//! Implemented in ./types/topup.rs
//!
//! Top up auctions are traditionally used for charitive purposes. Users place bid which are accumulated. At the end,
//! if the sum of all bids has reached the reserve_price set by the seller, the auction is won. In this case, the
//! aggregated amount of bids is transferred to the beneficiary of the auction (eg NGO), and the NFT is transferred
//! to the last bidder. If the auction is not won, bidders are able to claim back the amounts corresponding to their bids.
//!
//! ### Auction::Candle
//!
//! Implemented in ./types/candle.rs
//!
//! Candle auctions are used to incentivize bidders to bring out their bids early. At auction close, a randomness
//! algorithm choses a winning bid from the closing period.
//!
//! The first implementation uses the default length of Kusama parachain auctions: 99_356 blocks (apprx 6d 21h)
//! with a closing period of 72_000 blocks (apprx 5d).
//!
//! For better runtime performance, the closing period is divided into 100 ranges. When a user places a bid, it is
//! stored as the highest bid for the current period. All bid amounts are transferred to a subaccount held by the
//! auction.
//!
//! Once the auction is closed and the winning closing range is determined by the randomness, the total amount bid
//! by the winning bidder is transferred to the auction owner, and the NFT is transferred to the winner. The reserved
//! amounts bid by other users are available to be claimed.
//!
//! ## Auction sniping
//!
//! To avoid auction sniping, the pallet extends the end time of the auction for any late bids which are placed
//! shortly before auction close.
//!
//! When implementing this pallet, make sure that MinBidAmount > ExistentialDeposit
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

// Used for encoding/decoding into scale
use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure, require_transactional,
	traits::{tokens::nonfungibles::Inspect, Currency, ExistenceRequirement, Get, LockableCurrency, Randomness},
	PalletId, Parameter,
};
use frame_system::{ensure_signed, RawOrigin};

use scale_info::TypeInfo;
use sp_runtime::{
	traits::{
		AccountIdConversion, AtLeast32BitUnsigned, BlockNumberProvider, Bounded, CheckedAdd, CheckedSub,
		MaybeSerializeDeserialize, Member, One, Saturating, StaticLookup, Zero,
	},
	Permill,
};
/// This module contains all implementations for the different auction types
mod types;

use sp_std::convert::TryInto;
use sp_std::result;

pub use traits::*;
pub mod traits;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
pub mod weights;
use weights::WeightInfo;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod mocked_objects;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_nft::Config + TypeInfo {
		/// Event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Balance type (used for bidding)
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize;

		/// AuctionID type
		type AuctionId: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ Bounded
			+ CheckedAdd;

		/// Single type currency (TODO multiple currencies)
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

		/// Weights
		type WeightInfo: WeightInfo;

		/// Type that provides randomness
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		/// The block number provider
		type BlockNumberProvider: BlockNumberProvider<BlockNumber = Self::BlockNumber>;

		/// Limit of auction name length
		#[pallet::constant]
		type AuctionsStringLimit: Get<u32>;

		/// Increase end time to avoid sniping
		#[pallet::constant]
		type BidAddBlocks: Get<u32>;

		/// Next bid step in percent
		#[pallet::constant]
		type BidStepPerc: Get<u32>;

		/// Minimum auction duration
		#[pallet::constant]
		type MinAuctionDuration: Get<u32>;

		/// Minimum bid amount
		#[pallet::constant]
		type BidMinAmount: Get<u32>;

		/// Pallet ID (used for generating a subaccount)
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The default duration of a Candle auction
		#[pallet::constant]
		type CandleDefaultDuration: Get<u32>;

		/// The default duration of the closing period of a Candle auction
		#[pallet::constant]
		type CandleDefaultClosingPeriodDuration: Get<u32>;

		/// The default count of closing ranges of a Candle auction
		#[pallet::constant]
		type CandleDefaultClosingRangesCount: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn auctions)]
	/// Stores on-going and future auctions (closed auctions will be destroyed)
	pub(crate) type Auctions<T: Config> = StorageMap<_, Blake2_128Concat, T::AuctionId, Auction<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auctions_index)]
	/// Stores the next auction ID
	pub(crate) type NextAuctionId<T: Config> = StorageValue<_, T::AuctionId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn reserved_amounts)]
	/// Stores reserved amounts which were bid on a given auction
	pub(crate) type ReservedAmounts<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AuctionId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn highest_bidders_by_auction_closing_range)]
	/// Stores the higest bidder by auction and closing range
	pub(crate) type HighestBiddersByAuctionClosingRange<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::AuctionId, Blake2_128Concat, u32, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auction_owner_by_id)]
	/// Stores auction owner by ID
	pub(crate) type AuctionOwnerById<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AuctionId, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An auction is created
		AuctionCreated {
			auction_id: T::AuctionId,
			auction: Auction<T>,
		},
		/// An auction is updated
		AuctionUpdated {
			auction_id: T::AuctionId,
			auction: Auction<T>,
		},
		/// An auction is destroyed manually by owner
		AuctionDestroyed { auction_id: T::AuctionId },
		/// A bid is placed
		BidPlaced {
			auction_id: T::AuctionId,
			bidder: T::AccountId,
			bid: Bid<T>,
		},
		/// A bid amount was reserved by transferring it to pallet account
		BidAmountReserved {
			auction_id: T::AuctionId,
			bidder: T::AccountId,
			amount: BalanceOf<T>,
		},
		/// A bid amount was unreserved by transferring it to a beneficiary
		BidAmountUnreserved {
			auction_id: T::AuctionId,
			bidder: T::AccountId,
			amount: BalanceOf<T>,
			beneficiary: T::AccountId,
		},
		/// An auction has been closed
		AuctionClosed {
			auction_id: T::AuctionId,
			auction_winner: Option<T::AccountId>,
		},
		/// A reserved amount from a bid has been claimed
		BidAmountClaimed {
			auction_id: T::AuctionId,
			bidder: T::AccountId,
			amount: BalanceOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Auction does not exist
		AuctionDoesNotExist,
		/// Auction has not started yet
		AuctionNotStarted,
		/// Auction has already started
		AuctionAlreadyStarted,
		/// Auction is already closed (auction.common_data.closed is true)
		AuctionClosed,
		/// Auction has reached its ending time (auction.common_data.end is in the past)
		AuctionEndTimeReached,
		/// Auction end time has not been reached (auction.common_data.end is in the future)
		AuctionEndTimeNotReached,
		/// Auction.common_data.closed can only be set via close() extrinsic
		CannotSetAuctionClosed,
		/// Bid amount is invalid
		InvalidBidPrice,
		/// Auction count has reached its limit
		NoAvailableAuctionId,
		/// Auction has already started
		AuctionStartTimeAlreadyPassed,
		/// Invalid auction time configuration
		InvalidTimeConfiguration,
		/// No permissions to update/destroy auction
		NotAuctionOwner,
		/// No permission to handle token
		NotATokenOwner,
		/// Bid overflow
		BidOverflow,
		/// Cannot bid on own auction
		CannotBidOnOwnAuction,
		/// Time underflow
		TimeUnderflow,
		/// Token is frozen from transfers
		TokenFrozen,
		/// Auction name cannot be empty
		EmptyAuctionName,
		/// BoundedVec exceeds limits
		TooLong,
		/// Auction type cannot be changed
		NoChangeOfAuctionType,
		/// next_bid_min is invalid
		InvalidNextBidMin,
		/// TopUp reserved amount is invalid
		InvalidReservedAmount,
		/// TopUp bidder does not have claim to a reserved amount
		NoReservedAmountAvailableToClaim,
		/// Auction is closed and won, the bid funds are transferred to seller
		CannotClaimWonAuction,
		/// Auction should be closed before claims are made
		CloseAuctionBeforeClaimingReservedAmounts,
		/// Candle auction must have default duration
		CandleAuctionMustHaveDefaultDuration,
		/// Candle auction must have default closing period duration
		CandleAuctionMustHaveDefaultClosingPeriodDuration,
		/// Candle auction cannot have a reserve price
		CandleAuctionDoesNotSupportReservePrice,
		/// Math overflow
		Overflow,
		/// Error when determining the auction winner
		ErrorDeterminingAuctionWinner,
		/// Certain attributes of an auction cannot be updated
		CannotChangeForbiddenAttribute,
		/// Substrate cannot generate the AccountId (reserved amounts) based on auctions pallet id and auction id
		CannotGenerateAuctionAccount,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		///
		/// Creates a new auction for a given Auction type
		///
		/// - calls the create() implementation on the given Auction type
		///
		#[pallet::weight(
			<T as Config>::WeightInfo::create_english()
				.max(<T as Config>::WeightInfo::create_topup())
				.max(<T as Config>::WeightInfo::create_candle())
		)]
		pub fn create(origin: OriginFor<T>, auction: Auction<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			match &auction {
				Auction::English(auction_object) => {
					auction_object.create(sender, auction.clone())?;
				}
				Auction::TopUp(auction_object) => {
					auction_object.create(sender, auction.clone())?;
				}
				Auction::Candle(auction_object) => {
					auction_object.create(sender, auction.clone())?;
				}
			}

			Ok(())
		}

		///
		/// Updates an existing auction which has not yet started
		///
		/// - calls the update() implementation on the given Auction type
		/// - deposits AuctionUpdated event
		///
		#[pallet::weight(
			<T as Config>::WeightInfo::update_english()
				.max(<T as Config>::WeightInfo::update_topup())
				.max(<T as Config>::WeightInfo::update_candle())
		)]
		pub fn update(origin: OriginFor<T>, id: T::AuctionId, updated_auction: Auction<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			match updated_auction.clone() {
				Auction::English(auction_object) => {
					auction_object.update(sender, id)?;
				}
				Auction::TopUp(auction_object) => {
					auction_object.update(sender, id)?;
				}
				Auction::Candle(auction_object) => {
					auction_object.update(sender, id)?;
				}
			}

			Pallet::<T>::deposit_event(Event::AuctionUpdated {
				auction_id: id,
				auction: updated_auction,
			});

			Ok(())
		}

		///
		/// Destroys an existing auction which has not yet started
		///
		/// - validates write action
		/// - unfreezes NFT
		/// - calls destroy helper function
		/// - deposits AuctionDestroyed event
		///
		#[pallet::weight(
			<T as Config>::WeightInfo::destroy_english()
				.max(<T as Config>::WeightInfo::destroy_topup())
				.max(<T as Config>::WeightInfo::destroy_candle())
		)]
		pub fn destroy(origin: OriginFor<T>, id: T::AuctionId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let auction = <Auctions<T>>::get(id).ok_or(Error::<T>::AuctionDoesNotExist)?;

			match auction {
				Auction::English(auction_object) => {
					auction_object.destroy(sender, id)?;
				}
				Auction::TopUp(auction_object) => {
					auction_object.destroy(sender, id)?;
				}
				Auction::Candle(auction_object) => {
					auction_object.destroy(sender, id)?;
				}
			}

			Self::deposit_event(Event::AuctionDestroyed { auction_id: id });

			Ok(())
		}

		///
		/// Places a bid on a running auction
		///
		/// - validates bid
		/// - calls the bid() implementation on the given Auction type
		/// - deposits BidPlaced event
		///
		#[pallet::weight(
			<T as Config>::WeightInfo::bid_english()
				.max(<T as Config>::WeightInfo::bid_topup())
				.max(<T as Config>::WeightInfo::bid_candle())
		)]
		pub fn bid(origin: OriginFor<T>, auction_id: T::AuctionId, amount: BalanceOf<T>) -> DispatchResult {
			let bidder = ensure_signed(origin)?;
			let bid = Bid {
				amount,
				block_number: frame_system::Pallet::<T>::block_number(),
			};

			<Auctions<T>>::try_mutate(auction_id, |maybe_auction| -> DispatchResult {
				let auction = maybe_auction.as_mut().ok_or(Error::<T>::AuctionDoesNotExist)?;

				match auction {
					Auction::English(auction_object) => {
						Self::validate_bid(&bidder, &auction_object.common_data, &bid)?;
						auction_object.bid(auction_id, bidder.clone(), &bid)?;
					}
					Auction::TopUp(auction_object) => {
						Self::validate_bid(&bidder, &auction_object.common_data, &bid)?;
						auction_object.bid(auction_id, bidder.clone(), &bid)?;
					}
					Auction::Candle(auction_object) => {
						Self::validate_bid(&bidder, &auction_object.common_data, &bid)?;
						auction_object.bid(auction_id, bidder.clone(), &bid)?;
					}
				}

				Self::deposit_event(Event::BidPlaced {
					auction_id,
					bidder,
					bid,
				});

				Ok(())
			})
		}

		///
		/// Closes an auction
		///
		/// All auctions which have reached their auction end time do not accept any new bids.
		/// However, the transfer of NFT and funds happens once an auction is closed.
		///
		/// All auctions which have reached their auction end time must be closed by calling this exstrinsic.
		///
		/// The reason for not automating this in a hook is better runtime performance (similar to claiming
		/// staking rewards in Substrate).
		///
		/// - validates auction close
		/// - calls the implementation of close() on the given Auction type
		/// - if necessary, calls the helper function for destroying all auction-related data
		/// - deposits AuctionClosed event
		///
		#[pallet::weight(
			<T as Config>::WeightInfo::close_english()
				.max(<T as Config>::WeightInfo::close_topup())
				.max(<T as Config>::WeightInfo::close_candle())
		)]
		pub fn close(_origin: OriginFor<T>, auction_id: T::AuctionId) -> DispatchResult {
			let mut destroy_auction_data = false;

			<Auctions<T>>::try_mutate(auction_id, |maybe_auction| -> DispatchResult {
				let auction = maybe_auction.as_mut().ok_or(Error::<T>::AuctionDoesNotExist)?;

				match auction {
					Auction::English(auction_object) => {
						Self::validate_close(&auction_object.common_data)?;
						destroy_auction_data = auction_object.close(auction_id)?;
					}
					Auction::TopUp(auction_object) => {
						Self::validate_close(&auction_object.common_data)?;
						destroy_auction_data = auction_object.close(auction_id)?;
					}
					Auction::Candle(auction_object) => {
						Self::validate_close(&auction_object.common_data)?;
						destroy_auction_data = auction_object.close(auction_id)?;
					}
				}

				Ok(())
			})?;

			if destroy_auction_data {
				Self::handle_destroy(auction_id)?;
			}

			Ok(())
		}

		///
		/// Claims amounts reserved in an auction
		///
		/// For TopUp and Candle auctions, all bids are transferred to a subaccount held by the auction.
		/// After auction close, the reserved amounts of losing bidders can be claimed back.
		///
		/// - fetches claimable amount
		/// - calls claim() implementation on the Auction type
		/// - if necessary, calls the helper function for destroying all auction-related data
		#[pallet::weight(
			<T as Config>::WeightInfo::claim_topup()
				.max(<T as Config>::WeightInfo::claim_candle())
		)]
		pub fn claim(_origin: OriginFor<T>, bidder: T::AccountId, auction_id: T::AuctionId) -> DispatchResult {
			let claimable_amount = <ReservedAmounts<T>>::get(bidder.clone(), auction_id);
			ensure!(
				claimable_amount > Zero::zero(),
				Error::<T>::NoReservedAmountAvailableToClaim
			);

			let auction = <Auctions<T>>::get(auction_id).ok_or(Error::<T>::AuctionDoesNotExist)?;
			let destroy_auction_data: bool = match auction {
				Auction::English(auction_object) => {
					auction_object.claim(auction_id, bidder.clone(), claimable_amount)?
				}
				Auction::TopUp(auction_object) => auction_object.claim(auction_id, bidder.clone(), claimable_amount)?,
				Auction::Candle(auction_object) => {
					auction_object.claim(auction_id, bidder.clone(), claimable_amount)?
				}
			};

			Self::deposit_event(Event::BidAmountClaimed {
				auction_id,
				bidder,
				amount: claimable_amount,
			});

			if destroy_auction_data {
				Self::handle_destroy(auction_id)?;
			}

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	///
	/// Validates auction.common_data
	///
	/// Called on input data during create and update.
	///
	fn validate_common_data(sender: T::AccountId, common_data: &CommonAuctionData<T>) -> DispatchResult {
		// Sender must be the owner of the auction
		ensure!(sender == common_data.owner.clone(), Error::<T>::NotAuctionOwner);

		let current_block_number = frame_system::Pallet::<T>::block_number();
		ensure!(
			common_data.start >= current_block_number,
			Error::<T>::AuctionStartTimeAlreadyPassed
		);
		ensure!(
			common_data.start >= Zero::zero()
				&& common_data.end > Zero::zero()
				&& common_data.end
					> common_data
						.start
						.checked_add(&T::BlockNumber::from(T::MinAuctionDuration::get()))
						.ok_or(Error::<T>::Overflow)?,
			Error::<T>::InvalidTimeConfiguration
		);
		ensure!(!common_data.name.is_empty(), Error::<T>::EmptyAuctionName);

		// Start bid should always be above the minimum
		ensure!(
			common_data.next_bid_min >= <T as crate::Config>::BidMinAmount::get().into(),
			Error::<T>::InvalidNextBidMin
		);

		ensure!(!&common_data.closed, Error::<T>::CannotSetAuctionClosed);

		Ok(())
	}

	///
	/// Validates whether a user has sufficient permissions to create an auction
	///
	fn validate_create_permissions(sender: T::AccountId, common_data: &CommonAuctionData<T>) -> DispatchResult {
		// Sender must be NFT Owner
		let token_owner = pallet_nft::Pallet::<T>::owner(&common_data.token.0, &common_data.token.1);
		ensure!(token_owner == Some(sender), Error::<T>::NotATokenOwner);

		// TODO switch to pallet_nft
		let can_transfer =
			pallet_uniques::Pallet::<T>::can_transfer(&common_data.token.0.into(), &common_data.token.1.into());
		ensure!(can_transfer, Error::<T>::TokenFrozen);

		Ok(())
	}

	///
	/// Handles auction create
	///
	/// - fetches next auction_id
	/// - freezes NFT
	/// - inserts the Auction object in Auctions store
	/// - inserts a new record in AuctionOwnerById
	/// - emits AuctionCreated event
	///
	fn handle_create(
		sender: <T>::AccountId,
		auction: Auction<T>,
		common_data: &CommonAuctionData<T>,
	) -> DispatchResult {
		let auction_id = <NextAuctionId<T>>::try_mutate(|next_id| -> result::Result<<T>::AuctionId, DispatchError> {
			let current_id = *next_id;
			*next_id = next_id
				.checked_add(&One::one())
				.ok_or(Error::<T>::NoAvailableAuctionId)?;
			Ok(current_id)
		})?;

		pallet_uniques::Pallet::<T>::freeze(
			RawOrigin::Signed(sender.clone()).into(),
			common_data.token.0.into(),
			common_data.token.1.into(),
		)?;

		<Auctions<T>>::insert(auction_id, auction.clone());
		<AuctionOwnerById<T>>::insert(auction_id, &sender);

		Pallet::<T>::deposit_event(Event::AuctionCreated { auction_id, auction });

		Ok(())
	}

	///
	/// Validates whether a user has sufficient permissions to preform update or destroy on
	/// an existing auction
	///
	fn validate_update_destroy_permissions(
		sender: T::AccountId,
		existing_common_data: &CommonAuctionData<T>,
	) -> DispatchResult {
		// Sender must be the owner of the auction
		ensure!(
			sender == existing_common_data.owner.clone(),
			Error::<T>::NotAuctionOwner
		);

		// Auction must not be started
		let current_block_number = frame_system::Pallet::<T>::block_number();
		ensure!(
			existing_common_data.start > current_block_number,
			Error::<T>::AuctionAlreadyStarted
		);

		Ok(())
	}

	///
	/// Validates certain aspects relevant to the update action
	///
	fn validate_update(
		existing_common_data: &CommonAuctionData<T>,
		updated_common_data: &CommonAuctionData<T>,
	) -> DispatchResult {
		ensure!(
			existing_common_data.token == updated_common_data.token,
			Error::<T>::CannotChangeForbiddenAttribute
		);

		ensure!(
			existing_common_data.owner == updated_common_data.owner,
			Error::<T>::CannotChangeForbiddenAttribute
		);

		Ok(())
	}

	///
	/// Handles auction destroy
	///
	/// - unfreezes NFT
	/// - removes record from AuctionOwnerById
	/// - removes record from Auctions
	/// - deposits AuctionDestroyed event
	///
	fn handle_destroy(auction_id: T::AuctionId) -> DispatchResult {
		<AuctionOwnerById<T>>::remove(auction_id);
		<Auctions<T>>::remove(auction_id);
		// TODO: Find a non-iterative way to clear storage; only relevant for candle
		// <HighestBiddersByAuctionClosingRange<T>>::clear_prefix(auction_id, 100, None);

		Ok(())
	}

	///
	/// Reserves the amount of a bid by transferring it from the bidder to the subaccount of auction pallet
	/// Creates an entry in ReservedAmounts store
	/// Emits BidAmountReserved event
	///
	fn reserve_bid_amount(auction_id: T::AuctionId, bidder: T::AccountId, bid_amount: BalanceOf<T>) -> DispatchResult {
		let auction_account =
			Self::get_auction_subaccount_id(auction_id).ok_or(Error::<T>::CannotGenerateAuctionAccount)?;

		// Reserve funds by transferring to the subaccount of the auction
		<<T as crate::Config>::Currency as Currency<T::AccountId>>::transfer(
			&bidder,
			&auction_account,
			bid_amount,
			ExistenceRequirement::KeepAlive,
		)?;

		<ReservedAmounts<T>>::try_mutate(&bidder, auction_id, |locked_amount| -> DispatchResult {
			*locked_amount = locked_amount
				.checked_add(&bid_amount)
				.ok_or(Error::<T>::InvalidReservedAmount)?;

			Ok(())
		})?;

		Pallet::<T>::deposit_event(Event::BidAmountReserved {
			auction_id,
			bidder,
			amount: bid_amount,
		});

		Ok(())
	}

	///
	/// Unreserves the amount of a bid by transferring it from the subaccount of auction pallet to the beneficiary
	/// Removes the entry in ReservedAmounts store
	/// Emits BidAmountUnreserved event
	///
	fn unreserve_bid_amount(
		auction_id: T::AuctionId,
		bidder: T::AccountId,
		bid_amount: BalanceOf<T>,
		beneficiary: T::AccountId,
	) -> DispatchResult {
		let auction_account =
			Self::get_auction_subaccount_id(auction_id).ok_or(Error::<T>::CannotGenerateAuctionAccount)?;

		<<T as crate::Config>::Currency as Currency<T::AccountId>>::transfer(
			&auction_account,
			&beneficiary,
			bid_amount,
			ExistenceRequirement::AllowDeath,
		)?;

		Pallet::<T>::deposit_event(Event::BidAmountUnreserved {
			auction_id,
			bidder: bidder.clone(),
			amount: bid_amount,
			beneficiary: bidder.clone(),
		});

		<ReservedAmounts<T>>::remove(bidder, auction_id);

		Ok(())
	}

	///
	/// Unfreezes NFT (called after auction close)
	///
	fn unfreeze_nft(common_data: &CommonAuctionData<T>) -> DispatchResult {
		pallet_uniques::Pallet::<T>::thaw(
			RawOrigin::Signed(common_data.owner.clone()).into(),
			common_data.token.0.into(),
			common_data.token.1.into(),
		)?;

		Ok(())
	}

	///
	/// Validates certain aspects relevant to the bid action
	///
	fn validate_bid(
		bidder: &<T>::AccountId,
		common_auction_data: &CommonAuctionData<T>,
		bid: &Bid<T>,
	) -> DispatchResult {
		let block_number = T::BlockNumberProvider::current_block_number();
		ensure!(bidder != &common_auction_data.owner, Error::<T>::CannotBidOnOwnAuction);
		ensure!(block_number >= common_auction_data.start, Error::<T>::AuctionNotStarted);
		ensure!(
			block_number < common_auction_data.end,
			Error::<T>::AuctionEndTimeReached
		);
		ensure!(
			bid.amount >= common_auction_data.next_bid_min,
			Error::<T>::InvalidBidPrice
		);

		if let Some(current_bid) = &common_auction_data.last_bid {
			ensure!(bid.amount > current_bid.1, Error::<T>::InvalidBidPrice);
		} else {
			ensure!(!bid.amount.is_zero(), Error::<T>::InvalidBidPrice);
		}

		Ok(())
	}

	///
	/// Validates certain aspects relevant to the close action
	///
	fn validate_close(common_auction_data: &CommonAuctionData<T>) -> DispatchResult {
		ensure!(!common_auction_data.closed, Error::<T>::AuctionClosed);
		ensure!(
			Pallet::is_auction_ended(common_auction_data),
			Error::<T>::AuctionEndTimeNotReached
		);

		Ok(())
	}

	///
	/// Validates certain aspects relevant to the claim action
	///
	fn validate_claim(common_auction_data: &CommonAuctionData<T>) -> DispatchResult {
		ensure!(
			Pallet::<T>::is_auction_ended(common_auction_data),
			Error::<T>::AuctionEndTimeNotReached
		);
		ensure!(
			common_auction_data.closed,
			Error::<T>::CloseAuctionBeforeClaimingReservedAmounts
		);

		Ok(())
	}

	fn set_next_bid_min(common_auction_data: &mut CommonAuctionData<T>, amount: BalanceOf<T>) -> DispatchResult {
		let bid_step = Permill::from_percent(<T as crate::Config>::BidStepPerc::get()).mul_floor(amount);
		common_auction_data.next_bid_min = amount.checked_add(&bid_step).ok_or(Error::<T>::BidOverflow)?;

		Ok(())
	}

	///
	/// Helper function which extends auction end time if necessary to prevent auction sniping
	///
	fn avoid_auction_sniping(common_auction_data: &mut CommonAuctionData<T>) -> DispatchResult {
		let block_number = T::BlockNumberProvider::current_block_number();
		let time_left = common_auction_data
			.end
			.checked_sub(&block_number)
			.ok_or(Error::<T>::TimeUnderflow)?;
		if time_left < <T as crate::Config>::BidAddBlocks::get().into() {
			common_auction_data.end = block_number
				.checked_add(&T::BlockNumber::from(<T as crate::Config>::BidAddBlocks::get()))
				.ok_or(Error::<T>::Overflow)?;
		}

		Ok(())
	}

	///
	/// Generates AccountID of auction subaccount
	///
	fn get_auction_subaccount_id(auction_id: T::AuctionId) -> Option<T::AccountId> {
		T::PalletId::get().try_into_sub_account(auction_id)
	}

	/// A helper function which checks whether an auction ending block has been reached
	fn is_auction_ended(common_auction_data: &CommonAuctionData<T>) -> bool {
		T::BlockNumberProvider::current_block_number() >= common_auction_data.end
	}

	/// A helper function which checks whether an auction is won
	fn is_auction_won(common_auction_data: &CommonAuctionData<T>) -> bool {
		if !Pallet::is_auction_ended(common_auction_data) {
			return false;
		}

		match &common_auction_data.last_bid {
			Some(last_bid) => match common_auction_data.reserve_price {
				Some(reserve_price) => last_bid.1 >= reserve_price,
				None => true,
			},
			None => false,
		}
	}
}
