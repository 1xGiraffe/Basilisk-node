use super::*;

///
/// Implementation of EnglishAuction
///
impl<T: Config> NftAuction<T::AccountId, T::AuctionId, BalanceOf<T>, Auction<T>, Bid<T>> for EnglishAuction<T> {
	#[require_transactional]
	fn create(&self, sender: T::AccountId, auction: &Auction<T>) -> DispatchResult {
		self.validate_data()?;
		Pallet::<T>::validate_create(&self.common_data)?;
		Pallet::<T>::handle_create(sender, auction, &self.common_data)?;

		Ok(())
	}

	#[require_transactional]
	fn update(self, sender: T::AccountId, auction_id: T::AuctionId) -> DispatchResult {
		self.validate_data()?;

		<Auctions<T>>::try_mutate(auction_id, |maybe_auction| -> DispatchResult {
			let auction_result = maybe_auction.as_mut().ok_or(Error::<T>::AuctionDoesNotExist)?;

			if let Auction::English(english_auction) = auction_result {
				Pallet::<T>::validate_update(sender, &english_auction.common_data)?;
				*english_auction = self;

				Ok(())
			} else {
				Err(Error::<T>::NoChangeOfAuctionType.into())
			}
		})
	}

	///
	/// Places a bid on an EnglishAuction
	///
	/// - removes lock on auction.common_data.last_bid
	/// - sets lock on new bid
	/// - updates auction.common_data.last_bid and auction.common_data.next_bid_min
	/// - if necessary, increases auction end time to prevent sniping
	///
	#[require_transactional]
	fn bid(&mut self, _auction_id: T::AuctionId, bidder: T::AccountId, bid: &Bid<T>) -> DispatchResult {
		// Lock / Unlock funds
		if let Some(current_bid) = &self.common_data.last_bid {
			<T as crate::Config>::Currency::remove_lock(AUCTION_LOCK_ID, &current_bid.0);
		}
		<T as crate::Config>::Currency::set_lock(AUCTION_LOCK_ID, &bidder, bid.amount, WithdrawReasons::all());

		self.common_data.last_bid = Some((bidder, bid.amount));
		// Set next minimal bid
		Pallet::<T>::set_next_bid_min(&mut self.common_data, bid.amount)?;

		// Avoid auction sniping
		Pallet::<T>::avoid_auction_sniping(&mut self.common_data)?;

		Ok(())
	}

	///
	/// Closes an EnglishAuction
	///
	/// - removes lock on NFT
	/// - transfers NFT to winning bidder
	/// - removes lock on auction.common_data.last_bid
	/// - transfers the amount of the bid from the account of the bidder to the owner of the auction
	/// - sets auction.common_data.closed to true
	///
	#[require_transactional]
	fn close(&mut self, _auction_id: T::AuctionId) -> Result<bool, DispatchError> {
		Pallet::<T>::unfreeze_nft(&self.common_data)?;

		// there is a bid so let's determine a winner and transfer tokens
		if let Some(winner) = &self.common_data.last_bid {
			let dest = T::Lookup::unlookup(winner.0.clone());
			let source = T::Origin::from(frame_system::RawOrigin::Signed(self.common_data.owner.clone()));
			pallet_nft::Pallet::<T>::transfer(source, self.common_data.token.0, self.common_data.token.1, dest)?;
			<T as crate::Config>::Currency::remove_lock(AUCTION_LOCK_ID, &winner.0);
			<<T as crate::Config>::Currency as Currency<T::AccountId>>::transfer(
				&winner.0,
				&self.common_data.owner,
				winner.1,
				ExistenceRequirement::KeepAlive,
			)?;
		}

		self.common_data.closed = true;

		Ok(true)
	}

	/// English auctions do not implement reserved amounts and there are no claims
	fn claim(
		&self,
		_auction_id: T::AuctionId,
		_bidder: T::AccountId,
		_amount: BalanceOf<T>,
	) -> Result<bool, DispatchError> {
		Err(Error::<T>::ClaimsNotSupportedForThisAuctionType.into())
	}

	///
	/// Validates common and specific auction data
	///
	fn validate_data(&self) -> DispatchResult {
		Pallet::<T>::validate_common_data(&self.common_data)?;

		if let Some(reserve_price) = self.common_data.reserve_price {
			ensure!(
				reserve_price == self.common_data.next_bid_min,
				Error::<T>::InvalidNextBidMin
			);
		} else {
			ensure!(
				self.common_data.next_bid_min == T::BidMinAmount::get().into(),
				Error::<T>::InvalidNextBidMin
			);
		}

		Ok(())
	}
}
