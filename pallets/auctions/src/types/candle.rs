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

use super::*;

///
/// Implementation of Candle auction
///
impl<T: Config> NftAuction<T::AccountId, T::AuctionId, BalanceOf<T>, Auction<T>, Bid<T>> for CandleAuction<T> {
	///
	/// Creates a Candle Auction
	///
	#[require_transactional]
	fn create(&self, sender: T::AccountId, auction: Auction<T>) -> DispatchResult {
		Pallet::<T>::validate_create_permissions(sender.clone(), &self.common_data)?;
		self.validate_data(sender.clone())?;
		Pallet::<T>::handle_create(sender, auction, &self.common_data)?;

		Ok(())
	}

	///
	/// Updates a Candle Auction
	///
	#[require_transactional]
	fn update(self, sender: T::AccountId, auction_id: T::AuctionId) -> DispatchResult {
		self.validate_data(sender.clone())?;

		<Auctions<T>>::try_mutate(auction_id, |maybe_auction| -> DispatchResult {
			let auction_result = maybe_auction.as_mut().ok_or(Error::<T>::AuctionDoesNotExist)?;

			if let Auction::Candle(existing_auction) = auction_result {
				Pallet::<T>::validate_update_destroy_permissions(sender, &existing_auction.common_data)?;
				Pallet::<T>::validate_update(&existing_auction.common_data, &self.common_data)?;

				*existing_auction = self.clone();

				Ok(())
			} else {
				Err(Error::<T>::NoChangeOfAuctionType.into())
			}
		})
	}

	///
	/// Destroys a Candle Auction
	///
	#[require_transactional]
	fn destroy(self, sender: T::AccountId, id: T::AuctionId) -> DispatchResult {
		Pallet::<T>::validate_update_destroy_permissions(sender, &self.common_data)?;
		Pallet::<T>::handle_destroy(id)?;
		Pallet::<T>::unfreeze_nft(&self.common_data)?;

		Ok(())
	}

	///
	/// Places a bid on an CandleAuction
	///
	#[require_transactional]
	fn bid(&mut self, auction_id: T::AuctionId, bidder: T::AccountId, bid: &Bid<T>) -> DispatchResult {
		let closing_period_range = Pallet::<T>::determine_candle_closing_range(bid, self)?;

		<HighestBiddersByAuctionClosingRange<T>>::mutate(
			&auction_id,
			&closing_period_range,
			|highest_bidder| -> DispatchResult {
				*highest_bidder = Some(bidder.clone());

				Ok(())
			},
		)?;

		let past_bids_amount = <ReservedAmounts<T>>::get(bidder.clone(), auction_id);
		let amount_to_reserve = bid.amount.saturating_sub(past_bids_amount);

		// Reserve bid amount by transferring it to the subaccount of the auction
		Pallet::<T>::reserve_bid_amount(auction_id, bidder.clone(), amount_to_reserve)?;

		self.common_data.last_bid = Some((bidder.clone(), bid.amount));

		// Set next minimal bid
		Pallet::<T>::set_next_bid_min(&mut self.common_data, bid.amount)?;

		// Avoid auction sniping
		Pallet::<T>::avoid_auction_sniping(&mut self.common_data)?;

		Ok(())
	}

	///
	/// Closes a Candle auction
	///
	#[require_transactional]
	fn close(&mut self, auction_id: T::AuctionId) -> Result<bool, DispatchError> {
		let mut destroy_auction_data = false;
		let mut auction_winner: Option<T::AccountId> = None;

		Pallet::<T>::unfreeze_nft(&self.common_data)?;

		self.common_data.closed = true;

		if Pallet::<T>::is_auction_won(&self.common_data) {
			let winning_closing_range =
				Pallet::<T>::choose_random_block_from_range(One::one(), T::CandleDefaultClosingRangesCount::get())?;

			self.specific_data.winning_closing_range = Some(winning_closing_range);

			let mut maybe_winner: Option<T::AccountId> = None;
			let mut i = winning_closing_range;
			while i >= One::one() {
				let bidder = <HighestBiddersByAuctionClosingRange<T>>::get(&auction_id, i);

				if let Some(highest_bidder) = bidder {
					maybe_winner = Some(highest_bidder);
					break;
				}

				i = i.saturating_sub(One::one());
			}

			match maybe_winner {
				Some(winner) => {
					let dest = T::Lookup::unlookup(winner.clone());
					let source = T::Origin::from(frame_system::RawOrigin::Signed(self.common_data.owner.clone()));
					pallet_nft::Pallet::<T>::transfer(
						source,
						self.common_data.token.0,
						self.common_data.token.1,
						dest,
					)?;

					self.specific_data.winner = Some(winner.clone());

					let auction_account = Pallet::<T>::get_auction_subaccount_id(auction_id)
						.ok_or(Error::<T>::CannotGenerateAuctionAccount)?;

					let auction_balance =
						<<T as crate::Config>::Currency as Currency<T::AccountId>>::free_balance(&auction_account);
					let reserved_amount = <ReservedAmounts<T>>::get(&winner, &auction_id);

					ensure!(
						reserved_amount > Zero::zero(),
						Error::<T>::NoReservedAmountAvailableToClaim
					);

					<<T as crate::Config>::Currency as Currency<T::AccountId>>::transfer(
						&auction_account,
						&self.common_data.owner,
						reserved_amount,
						ExistenceRequirement::AllowDeath,
					)?;

					<ReservedAmounts<T>>::remove(&winner, &auction_id);

					// Only one bidder: Auction and related data can be pruned
					if reserved_amount == auction_balance {
						destroy_auction_data = true;
					}

					auction_winner = Some(winner.clone());
				}
				None => return Err(Error::<T>::ErrorDeterminingAuctionWinner.into()),
			}
		} else {
			destroy_auction_data = true;
		}

		Pallet::<T>::deposit_event(Event::AuctionClosed {
			auction_id: auction_id,
			auction_winner: auction_winner,
		});

		Ok(destroy_auction_data)
	}

	///
	/// Claims reserved amounts which were bid on a Candle auction
	///
	#[require_transactional]
	fn claim(
		&self,
		auction_id: T::AuctionId,
		bidder: T::AccountId,
		amount: BalanceOf<T>,
	) -> Result<bool, DispatchError> {
		let mut destroy_auction_data = false;

		Pallet::<T>::validate_claim(&self.common_data)?;
		Pallet::<T>::unreserve_bid_amount(auction_id, bidder.clone(), amount, bidder)?;

		let auction_account =
			Pallet::<T>::get_auction_subaccount_id(auction_id).ok_or(Error::<T>::CannotGenerateAuctionAccount)?;

		let auction_balance =
			<<T as crate::Config>::Currency as Currency<T::AccountId>>::free_balance(&auction_account);

		if auction_balance.is_zero() {
			destroy_auction_data = true;
		}

		Ok(destroy_auction_data)
	}

	///
	/// Validates common and specific auction data
	///
	fn validate_data(&self, sender: T::AccountId) -> DispatchResult {
		Pallet::<T>::validate_common_data(sender, &self.common_data)?;

		let default_duration = self
			.common_data
			.start
			.checked_add(&T::BlockNumber::from(T::CandleDefaultDuration::get()))
			.ok_or(Error::<T>::Overflow)?;

		ensure!(
			self.common_data.end == default_duration,
			Error::<T>::CandleAuctionMustHaveDefaultDuration
		);

		ensure!(
			self.common_data.reserve_price.is_none(),
			Error::<T>::CandleAuctionDoesNotSupportReservePrice
		);

		let closing_period_duration = self
			.common_data
			.end
			.checked_sub(&T::BlockNumber::from(T::CandleDefaultClosingPeriodDuration::get()))
			.ok_or(Error::<T>::Overflow)?;

		ensure!(
			self.specific_data.closing_start == closing_period_duration,
			Error::<T>::CandleAuctionMustHaveDefaultClosingPeriodDuration
		);

		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	fn choose_random_block_from_range(from: u32, to: u32) -> Result<u32, DispatchError> {
		ensure!(from < to, Error::<T>::InvalidTimeConfiguration);
		let mut random_number = Self::generate_random_number(0u32);

		let difference = to.checked_sub(from).ok_or(Error::<T>::Overflow)?;

		// Best effort attempt to remove bias from modulus operator.
		for i in 1..10 {
			if random_number < u32::MAX.saturating_sub(u32::MAX % difference) {
				break;
			}

			random_number = Self::generate_random_number(i);
		}

		// Caution: Remainder (%) operator only safe with unsigned
		let result = from
			.checked_add(random_number % difference)
			.ok_or(Error::<T>::Overflow)?;

		Ok(result)
	}

	fn generate_random_number(seed: u32) -> u32 {
		let (random_seed, _) = T::Randomness::random(&(T::PalletId::get(), seed).encode());
		let random_number = <u32>::decode(&mut random_seed.as_ref()).unwrap_or_default();
		random_number
	}

	fn determine_candle_closing_range(bid: &Bid<T>, auction: &CandleAuction<T>) -> Result<u32, Error<T>> {
		let block_number: u32 = bid.block_number.try_into().map_err(|_| Error::<T>::Overflow)?;
		let closing_start: u32 = auction
			.specific_data
			.closing_start
			.try_into()
			.map_err(|_| Error::<T>::Overflow)?;
		let end: u32 = auction.common_data.end.try_into().map_err(|_| Error::<T>::Overflow)?;

		if block_number < closing_start {
			Ok(One::one())
		} else if (closing_start..end).contains(&block_number) {
			let auction_duration = end.checked_sub(closing_start).ok_or(Error::<T>::Overflow)?;
			let block_spread = block_number.checked_sub(closing_start).ok_or(Error::<T>::Overflow)?;
			let multiplied_block_spread = block_spread.checked_mul(100).ok_or(Error::<T>::Overflow)?;

			let closing_range = multiplied_block_spread
				.checked_div(auction_duration)
				.ok_or(Error::<T>::Overflow)?;

			Ok(if closing_range.is_zero() {
				One::one()
			} else {
				closing_range
			})
		} else {
			Ok(T::CandleDefaultClosingRangesCount::get())
		}
	}
}
