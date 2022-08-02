use frame_support::pallet_prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use scale_info::TypeInfo;
use sp_runtime::Percent;

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Offer<AccountId, Balance, BlockNumber> {
	/// User who made the offer
	pub(super) maker: AccountId,
	/// Offered amount
	pub(super) amount: Balance,
	/// After this block the offer can't be accepted
	pub(super) expires: BlockNumber,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Royalty<AccountId> {
	/// The user account which receives the royalty
	pub author: AccountId,
	/// Royalty precentage
	pub royalty: Percent,
}
