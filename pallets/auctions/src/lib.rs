#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

// Used for encoding/decoding into scale
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{
		tokens::nonfungibles::Inspect, Currency, ExistenceRequirement, Get, LockIdentifier, LockableCurrency,
		WithdrawReasons,
	},
	Parameter,
};
use frame_system::{ensure_signed, RawOrigin};
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, Bounded, CheckedAdd, CheckedSub, MaybeSerializeDeserialize, Member, One, StaticLookup,
		Zero,
	},
	Permill,
};
use sp_std::result;
pub use traits::*;
use weights::WeightInfo;

mod benchmarking;
pub mod traits;
pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Identifier for the currency lock on accounts
const AUCTION_LOCK_ID: LockIdentifier = *b"_auction";

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_nft::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The balance type for bidding
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize;

		/// The auction ID type
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

		// This type is needed to convert from Currency to Balance
		type CurrencyBalance: From<Self::Balance>
			+ Into<<<Self as crate::Config>::Currency as Currency<<Self as frame_system::Config>::AccountId>>::Balance>;

		/// Limit of auction name length
		type AuctionsStringLimit: Get<u32>;

		/// Increase end time to avoid sniping
		type BidAddBlocks: Get<u32>;

		/// Next bid step in percent
		type BidStepPerc: Get<u32>;

		/// Minimum auction duration
		type MinAuctionDuration: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn auctions)]
	/// Stores on-going and future auctions. Closed auction are removed.
	pub type Auctions<T: Config> = StorageMap<_, Twox64Concat, T::AuctionId, Auction<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auctions_index)]
	/// Track the next auction ID.
	pub type NextAuctionId<T: Config> = StorageValue<_, T::AuctionId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auction_end_time)]
	/// Index auctions by end time.
	pub type AuctionEndTime<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::BlockNumber, Twox64Concat, T::AuctionId, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auction_owner_by_id)]
	/// Auction owner by ID
	pub type AuctionOwnerById<T: Config> = StorageMap<_, Twox64Concat, T::AuctionId, T::AccountId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Auction created
		AuctionCreated(T::AccountId, T::AuctionId),
		/// A bid is placed
		Bid(T::AuctionId, T::AccountId, BalanceOf<T>),
		/// Auction ended
		AuctionConcluded(T::AuctionId),
		/// Auction removed
		AuctionRemoved(T::AuctionId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Auction does not exist
		AuctionNotExist,
		/// Auction has not started yet
		AuctionNotStarted,
		/// Auction has already started
		AuctionAlreadyStarted,
		/// Bid amount is invalid
		InvalidBidPrice,
		/// Auction count has reached it's limit
		NoAvailableAuctionId,
		/// Auction has already started
		AuctionStartTimeAlreadyPassed,
		/// Invalid auction time configuration
		InvalidTimeConfiguration,
		/// No permissions to update/destroy auction
		NotAuctionOwner,
		/// No permission to handle token
		NotATokenOwner,
		/// Auction has already ended
		AuctionAlreadyConcluded,
		/// Bid overflow
		BidOverflow,
		/// Can't bid on own auction
		BidOnOwnAuction,
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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::create_auction())]
		pub fn create(origin: OriginFor<T>, auction: Auction<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			match &auction {
				Auction::English(data) => {
					Self::validate_general_data(&data.general_data)?;
					Self::validate_create(&data.general_data)?;
					Self::handle_create(sender, &auction, &data.general_data)?;
				}
			}

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::update_auction())]
		pub fn update(origin: OriginFor<T>, id: T::AuctionId, updated_auction: Auction<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			match &updated_auction {
				Auction::English(data) => {
					Self::validate_general_data(&data.general_data)?;
					Self::handle_update(sender, id, updated_auction.clone(), &data.general_data)?;
				}
			}

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::destroy_auction())]
		pub fn destroy(origin: OriginFor<T>, id: T::AuctionId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let auction = <Auctions<T>>::get(id).ok_or(Error::<T>::AuctionNotExist)?;

			match &auction {
				Auction::English(data) => {
					Self::validate_update(sender, &data.general_data)?;
					Self::handle_destroy(id, &data.general_data)?;
				}
			}

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::bid_value())]
		pub fn bid(origin: OriginFor<T>, auction_id: T::AuctionId, value: BalanceOf<T>) -> DispatchResult {
			let bidder = ensure_signed(origin)?;
			let auction = <Auctions<T>>::get(auction_id).ok_or(Error::<T>::AuctionNotExist)?;

			match &auction {
				Auction::English(data) => {
					Self::validate_bid(&bidder, &data.general_data, value)?;
					EnglishAuction::<T>::bid(bidder.clone(), auction_id, value)?;
				}
			}

			Self::deposit_event(Event::Bid(auction_id, bidder, value));

			Ok(())
		}
	}

	// TODO implement a different auction closing mechanism (not in hooks)
	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_finalize(now: T::BlockNumber) {
			Self::close_auctions(now);
		}
	}
}

impl<T: Config> Pallet<T> {
	fn validate_general_data(general_data: &GeneralAuctionData<T>) -> DispatchResult {
		let current_block_number = frame_system::Pallet::<T>::block_number();
		ensure!(
			general_data.start >= current_block_number,
			Error::<T>::AuctionStartTimeAlreadyPassed
		);
		ensure!(
			general_data.start >= Zero::zero()
				&& general_data.end > Zero::zero()
				&& general_data.end > general_data.start + T::MinAuctionDuration::get().into(),
			Error::<T>::InvalidTimeConfiguration
		);
		ensure!(!general_data.name.is_empty(), Error::<T>::EmptyAuctionName);
		let token_owner = pallet_uniques::Pallet::<T>::owner(general_data.token.0, general_data.token.1);
		ensure!(
			token_owner == Some(general_data.owner.clone()),
			Error::<T>::NotATokenOwner
		);
		Ok(())
	}

	fn validate_create(general_data: &GeneralAuctionData<T>) -> DispatchResult {
		let is_transferrable = pallet_uniques::Pallet::<T>::can_transfer(&general_data.token.0, &general_data.token.1);
		ensure!(is_transferrable, Error::<T>::TokenFrozen);
		Ok(())
	}

	fn handle_create(
		sender: <T>::AccountId,
		auction: &Auction<T>,
		general_data: &GeneralAuctionData<T>,
	) -> DispatchResult {
		let auction_id = <NextAuctionId<T>>::try_mutate(|next_id| -> result::Result<<T>::AuctionId, DispatchError> {
			let current_id = *next_id;
			*next_id = next_id
				.checked_add(&One::one())
				.ok_or(Error::<T>::NoAvailableAuctionId)?;
			Ok(current_id)
		})?;

		<Auctions<T>>::insert(auction_id, auction.clone());
		<AuctionOwnerById<T>>::insert(auction_id, &sender);
		<AuctionEndTime<T>>::insert(general_data.end, auction_id, ());

		pallet_uniques::Pallet::<T>::freeze(
			RawOrigin::Signed(sender.clone()).into(),
			general_data.token.0,
			general_data.token.1,
		)?;

		Self::deposit_event(Event::AuctionCreated(sender, auction_id));

		Ok(())
	}

	fn validate_update(sender: <T>::AccountId, general_data: &GeneralAuctionData<T>) -> DispatchResult {
		ensure!(general_data.owner == sender, Error::<T>::NotAuctionOwner);

		let current_block_number = frame_system::Pallet::<T>::block_number();
		ensure!(
			current_block_number < general_data.start,
			Error::<T>::AuctionAlreadyStarted
		);

		Ok(())
	}

	fn handle_update(
		sender: <T>::AccountId,
		auction_id: T::AuctionId,
		updated_auction: Auction<T>,
		general_data: &GeneralAuctionData<T>,
	) -> DispatchResult {
		<Auctions<T>>::try_mutate(auction_id, |auction_result| -> DispatchResult {
			if let Some(_auction) = auction_result {
				Self::validate_update(sender, &general_data)?;
				*auction_result = Some(updated_auction);
				Ok(())
			} else {
				Err(Error::<T>::AuctionNotExist.into())
			}
		})
	}

	fn handle_destroy(auction_id: T::AuctionId, general_data: &GeneralAuctionData<T>) -> DispatchResult {
		pallet_uniques::Pallet::<T>::thaw(
			RawOrigin::Signed(general_data.owner.clone()).into(),
			general_data.token.0,
			general_data.token.1,
		)?;

		<AuctionOwnerById<T>>::remove(auction_id);
		<Auctions<T>>::remove(auction_id);

		Self::deposit_event(Event::AuctionRemoved(auction_id));

		Ok(())
	}

	fn validate_bid(
		bidder: &<T>::AccountId,
		general_auction_data: &GeneralAuctionData<T>,
		value: BalanceOf<T>,
	) -> DispatchResult {
		let block_number = <frame_system::Pallet<T>>::block_number();
		ensure!(bidder != &general_auction_data.owner, Error::<T>::BidOnOwnAuction);
		ensure!(block_number > general_auction_data.start, Error::<T>::AuctionNotStarted);
		ensure!(
			block_number < general_auction_data.end,
			Error::<T>::AuctionAlreadyConcluded
		);
		ensure!(value >= general_auction_data.next_bid_min, Error::<T>::InvalidBidPrice);

		if let Some(ref current_bid) = general_auction_data.last_bid {
			ensure!(value > current_bid.1, Error::<T>::InvalidBidPrice);
		} else {
			ensure!(!value.is_zero(), Error::<T>::InvalidBidPrice);
		}

		Ok(())
	}

	fn close_auctions(current_block: T::BlockNumber) -> DispatchResult {
		for (auction_id, _) in <AuctionEndTime<T>>::drain_prefix(&current_block) {
			let auction = <Auctions<T>>::get(auction_id).ok_or(Error::<T>::AuctionNotExist)?;

			match &auction {
				Auction::English(data) => {
					EnglishAuction::<T>::close(data)?;
				}
			}
		}

		Ok(())
	}
}

impl<T: Config> NftAuction<T::AccountId, T::AuctionId, BalanceOf<T>, Auction<T>> for EnglishAuction<T> {
	fn bid(bidder: T::AccountId, auction_id: T::AuctionId, value: BalanceOf<T>) -> DispatchResult {
		<Auctions<T>>::try_mutate(auction_id, |maybe_auction| -> DispatchResult {
			let auction = maybe_auction.as_mut().ok_or(Error::<T>::AuctionNotExist)?;
			let Auction::English(data) = auction;
			// Lock / Unlock funds
			if let Some(ref current_bid) = data.general_data.last_bid {
				<T as crate::Config>::Currency::remove_lock(AUCTION_LOCK_ID, &current_bid.0);
			}
			<T as crate::Config>::Currency::set_lock(AUCTION_LOCK_ID, &bidder, value, WithdrawReasons::all());

			data.general_data.last_bid = Some((bidder, value));
			// Set next minimal bid
			let bid_step = Permill::from_percent(<T as crate::Config>::BidStepPerc::get()).mul_floor(value);
			data.general_data.next_bid_min = value.checked_add(&bid_step).ok_or(Error::<T>::BidOverflow)?;

			// Avoid auction sniping
			let block_number = <frame_system::Pallet<T>>::block_number();
			let time_left = data
				.general_data
				.end
				.checked_sub(&block_number)
				.ok_or(Error::<T>::TimeUnderflow)?;
			if time_left < <T as crate::Config>::BidAddBlocks::get().into() {
				data.general_data.end = block_number + <T as crate::Config>::BidAddBlocks::get().into();
			}

			Ok(())
		})
	}

	fn close(&self) -> DispatchResult {
		pallet_uniques::Pallet::<T>::thaw(
			RawOrigin::Signed(self.general_data.owner.clone()).into(),
			self.general_data.token.0,
			self.general_data.token.1,
		)?;
		// there is a bid so let's determine a winner and transfer tokens
		if let Some(ref winner) = self.general_data.last_bid {
			let dest = T::Lookup::unlookup(winner.0.clone());
			let source = T::Origin::from(frame_system::RawOrigin::Signed(self.general_data.owner.clone()));
			pallet_nft::Pallet::<T>::transfer(source, self.general_data.token.0, self.general_data.token.1, dest)?;
			<T as crate::Config>::Currency::remove_lock(AUCTION_LOCK_ID, &winner.0);
			<<T as crate::Config>::Currency as Currency<T::AccountId>>::transfer(
				&winner.0,
				&self.general_data.owner,
				winner.1,
				ExistenceRequirement::KeepAlive,
			)?;
		}

		Ok(())
	}
}
