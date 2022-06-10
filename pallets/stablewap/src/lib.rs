// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Stableswap pallet (v1)
//!
//! Curve/stableswap AMM implementation.
//!
//! Version v1 - supports only 2 assets pool.
//!
//! ### Terminology
//!
//! * **LP** - liquidity provider
//! * **Share Token** - a token representing share asset of specific pool. Each pool has its own share token.
//! * **Amplification** - curve AMM pool amplification parameter
//!
//! ## Assumptions
//!
//! Only 2 assets pool are possible to create in V1.
//!
//! A pool can be created only by allowed `CreatePoolOrigin`.
//!
//! LP must add liquidity of both pool assets. in V1 it is not allowed single token LPing.
//!
//! LP specifies an amount of liquidity to be added of one selected asset, the required amount of second pool asset is calculated
//! in a way that the ratio does not change.
//!
//! LP is given certain amount of shares by minting a pool's share token.
//!
//! When LP decides to withdraw liquidity, it receives both assets. Single token withdrawal is not supported.
//!
//! ## Interface
//!
//! ### Dispatchable functions
//!
//! * `create_pool`
//! * `add_liquidity`
//! * `remove_liquidity`
//! * `sell`
//! * `buy`
//!

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{DispatchResult, Get};
use frame_support::transactional;
use sp_runtime::Permill;
use sp_std::vec::Vec;

pub use pallet::*;

mod math;
pub mod traits;
pub mod types;
pub mod weights;

use crate::types::Balance;
use weights::WeightInfo;

#[cfg(test)]
pub(crate) mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarks;

/// Stableswap share token and account id identifier.
/// Used as identifier to create share token unique names and account ids.
const POOL_IDENTIFIER: &[u8] = b"sts";

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::math::{
		calculate_add_liquidity_shares, calculate_asset_b_reserve, calculate_in_given_out, calculate_out_given_in,
		calculate_remove_liquidity_amounts,
	};
	use crate::traits::ShareAccountIdFor;
	use crate::types::{AssetAmounts, Balance, PoolAssets, PoolId, PoolInfo};
	use codec::HasCompact;
	use core::ops::RangeInclusive;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use hydradx_traits::{Registry, ShareTokenRegistry};
	use orml_traits::MultiCurrency;
	use sp_runtime::traits::Zero;
	use sp_runtime::ArithmeticError;
	use sp_runtime::Permill;

	#[pallet::pallet]
	#[pallet::generate_store(pub(crate) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Identifier for the class of asset.
		type AssetId: Member
			+ Parameter
			+ PartialOrd
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		/// Multi currency mechanism
		type Currency: MultiCurrency<Self::AccountId, CurrencyId = Self::AssetId, Balance = Balance>;

		/// Account ID constructor
		type ShareAccountId: ShareAccountIdFor<PoolAssets<Self::AssetId>, AccountId = Self::AccountId>;

		/// Asset registry mechanism
		type AssetRegistry: ShareTokenRegistry<Self::AssetId, Vec<u8>, Balance, DispatchError>;

		/// The origin which can create a new pool
		type CreatePoolOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

		/// Precision used in Newton's method to solve math equations iteratively.
		#[pallet::constant]
		type Precision: Get<Balance>;

		/// Minimum pool liquidity
		#[pallet::constant]
		type MinPoolLiquidity: Get<Balance>;

		/// Minimum trading amount
		#[pallet::constant]
		type MinTradingLimit: Get<Balance>;

		/// Amplification inclusive range. Pool's amp can be selected from the range only.
		#[pallet::constant]
		type AmplificationRange: Get<RangeInclusive<u16>>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Existing pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, PoolId<T::AssetId>, PoolInfo<T::AssetId>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A pool was created.
		PoolCreated {
			id: PoolId<T::AssetId>,
			assets: (T::AssetId, T::AssetId),
			initial_liquidity: (Balance, Balance),
			amplification: u16,
		},
		/// Liquidity of an asset was added to a pool.
		LiquidityAdded {
			id: PoolId<T::AssetId>,
			who: T::AccountId,
			assets: (T::AssetId, T::AssetId),
			amounts: (Balance, Balance),
		},
		/// Liquidity removed.
		LiquidityRemoved {
			id: PoolId<T::AssetId>,
			who: T::AccountId,
			shares: Balance,
			amounts: (Balance, Balance),
		},
		/// Sell trade executed.
		SellExecuted {
			who: T::AccountId,
			asset_in: T::AssetId,
			asset_out: T::AssetId,
			amount_in: Balance,
			amount_out: Balance,
			fee: Balance,
		},
		/// Buy trade executed.
		BuyExecuted {
			who: T::AccountId,
			asset_in: T::AssetId,
			asset_out: T::AssetId,
			amount_in: Balance,
			amount_out: Balance,
			fee: Balance,
		},
	}

	#[pallet::error]
	#[cfg_attr(test, derive(PartialEq))]
	pub enum Error<T> {
		/// Creating a pool with same assets is not allowed.
		SameAssets,

		/// A pool with given assets does not exist.
		PoolNotFound,

		/// A pool with given assets already exists.
		PoolExists,

		/// Asset is not in the pool.
		AssetNotInPool,

		/// One or more assets are not registered in AssetRegistry
		AssetNotRegistered,

		/// Invalid asset amount provided. Amount must be greater than zero.
		InvalidAssetAmount,

		/// Balance of an asset is nto sufficient to perform a trade.
		InsufficientBalance,

		/// Balance of an share asset is nto sufficient to withdraw liquidity.
		InsufficientShares,

		/// Liquidity has not reached the required minimum.
		InsufficientLiquidity,

		/// Insufficient liquidity left in the pool after withdrawal.
		InsufficientLiquidityRemaining,

		/// Amount is less than min trading limit.
		InsufficientTradingAmount,

		/// Minimum limit has not been reached during trade.
		BuyLimitNotReached,

		/// Maximum limit has been exceeded during trade.
		SellLimitExceeded,

		/// Initial liquidity of asset must be > 0.
		InvalidInitialLiquidity,

		/// Account balance is too low.
		BalanceTooLow,

		/// Amplification is outside specified range.
		InvalidAmplification,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a 2-asset pool with initial liquidity for both assets.
		///
		/// Both assets must be correctly registered in `T::AssetRegistry`
		///
		/// Initial liquidity must be > 0.
		///
		/// Origin is given corresponding amount of shares.
		///
		/// Parameters:
		/// - `origin`: Must be T::CreatePoolOrigin
		/// - `assets`: Asset ids tuple
		/// - `initial_liquidity`: Corresponding initial liquidity of `assets`
		/// - `amplification`: Pool amplification
		/// - `fee`: trade fee to be applied in sell/buy trades
		///
		/// Emits `PoolCreated` event when successful.
		#[pallet::weight(<T as Config>::WeightInfo::create_pool())]
		#[transactional]
		pub fn create_pool(
			origin: OriginFor<T>,
			assets: (T::AssetId, T::AssetId),
			initial_liquidity: (Balance, Balance),
			amplification: u16,
			fee: Permill,
		) -> DispatchResult {
			let who = T::CreatePoolOrigin::ensure_origin(origin)?;

			let pool_assets: PoolAssets<T::AssetId> = assets.into();

			ensure!(pool_assets.is_valid(), Error::<T>::SameAssets);

			ensure!(
				T::AmplificationRange::get().contains(&amplification),
				Error::<T>::InvalidAmplification
			);

			ensure!(T::AssetRegistry::exists(pool_assets.0), Error::<T>::AssetNotRegistered);
			ensure!(T::AssetRegistry::exists(pool_assets.1), Error::<T>::AssetNotRegistered);

			let reserves: AssetAmounts<Balance> = initial_liquidity.into();

			ensure!(reserves.is_valid(), Error::<T>::InvalidInitialLiquidity);

			ensure!(
				T::Currency::free_balance(assets.0, &who) >= initial_liquidity.0,
				Error::<T>::BalanceTooLow,
			);
			ensure!(
				T::Currency::free_balance(assets.1, &who) >= initial_liquidity.1,
				Error::<T>::BalanceTooLow,
			);

			let share_asset_ident = T::ShareAccountId::name(&pool_assets, Some(POOL_IDENTIFIER));

			let share_asset = T::AssetRegistry::get_or_create_shared_asset(
				share_asset_ident,
				(&pool_assets).into(),
				Balance::zero(),
			)?;

			let pool_id = PoolId(share_asset);

			let pool = Pools::<T>::try_mutate(&pool_id, |maybe_pool| -> Result<PoolInfo<T::AssetId>, DispatchError> {
				ensure!(maybe_pool.is_none(), Error::<T>::PoolExists);

				let pool = PoolInfo {
					assets: pool_assets.clone(),
					amplification,
					fee,
				};

				*maybe_pool = Some(pool.clone());

				Ok(pool)
			})?;

			let share_amount = calculate_add_liquidity_shares(
				&AssetAmounts::default(),
				&reserves,
				T::Precision::get(),
				amplification.into(),
				Balance::zero(),
			)
			.ok_or(ArithmeticError::Overflow)?;

			ensure!(
				share_amount >= T::MinPoolLiquidity::get(),
				Error::<T>::InsufficientLiquidity
			);

			let pool_account = T::ShareAccountId::from_assets(&pool.assets, Some(POOL_IDENTIFIER));

			T::Currency::transfer(assets.0, &who, &pool_account, initial_liquidity.0)?;
			T::Currency::transfer(assets.1, &who, &pool_account, initial_liquidity.1)?;

			T::Currency::deposit(pool_id.0, &who, share_amount)?;

			Self::deposit_event(Event::PoolCreated {
				id: pool_id,
				assets,
				initial_liquidity,
				amplification,
			});

			Ok(())
		}

		/// Add liquidity to selected 2-asset pool.
		///
		/// LP must provide liquidity of both assets by specifying amount of asset a.
		///
		/// Amount of asset b is calculated so the ratio does not change:
		///
		/// new_reserve_b = (reserve_a + amount) * reserve_b / reserve_a
		/// amount_b = new_reserve_b - reserve_b
		///
		/// LP must have sufficient amount of asset a/b - amount_a and amount_b.
		///
		/// Origin is given corresponding amount of shares.
		///
		/// Parameters:
		/// - `origin`: liquidity provider
		/// - `pool_id`: Pool Id
		/// - `asset`: Asset id
		/// - `amount`: liquidity amount of `asset` to be added to the pool.
		///
		/// Emits `LiquidityAdded` event when successful.
		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		#[transactional]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			pool_id: PoolId<T::AssetId>,
			asset: T::AssetId,
			amount: Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				amount >= T::MinTradingLimit::get(),
				Error::<T>::InsufficientTradingAmount
			);

			// Ensure that who has enough balance of given asset
			ensure!(
				T::Currency::free_balance(asset, &who) >= amount,
				Error::<T>::InsufficientBalance
			);

			let pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;

			ensure!(pool.contains_asset(asset), Error::<T>::AssetNotInPool);

			let pool_account = T::ShareAccountId::from_assets(&pool.assets, Some(POOL_IDENTIFIER));

			// Work out which asset is asset B ( second asset which has to provided by LP )
			// Update initial reserves in correct order ( asset a is given as params, asset b is second asset)
			let (assets, initial_reserves) = {
				let initial = (
					T::Currency::free_balance(pool.assets.0, &pool_account),
					T::Currency::free_balance(pool.assets.1, &pool_account),
				);
				if asset == pool.assets.0 {
					((asset, pool.assets.1), AssetAmounts(initial.0, initial.1))
				} else {
					((asset, pool.assets.0), AssetAmounts(initial.1, initial.0))
				}
			};

			let updated_a_reserve = initial_reserves
				.0
				.checked_add(amount)
				.ok_or(ArithmeticError::Overflow)?;

			// Calculate new reserve of asset b and work out the difference to get required amount of second asset which has to be provided by LP too.
			let asset_b_reserve = calculate_asset_b_reserve(initial_reserves.0, initial_reserves.1, updated_a_reserve)
				.ok_or(ArithmeticError::Overflow)?;

			let asset_b_amount = asset_b_reserve
				.checked_sub(initial_reserves.1)
				.ok_or(ArithmeticError::Underflow)?;

			ensure!(
				T::Currency::free_balance(assets.1, &who) >= asset_b_amount,
				Error::<T>::InsufficientBalance
			);

			let new_reserves = AssetAmounts(updated_a_reserve, asset_b_reserve);

			let share_issuance = T::Currency::total_issuance(pool_id.0);

			let share_amount = calculate_add_liquidity_shares(
				&initial_reserves,
				&new_reserves,
				T::Precision::get(),
				pool.amplification.into(),
				share_issuance,
			)
			.ok_or(ArithmeticError::Overflow)?;

			T::Currency::deposit(pool_id.0, &who, share_amount)?;

			T::Currency::transfer(assets.0, &who, &pool_account, amount)?;
			T::Currency::transfer(assets.1, &who, &pool_account, asset_b_amount)?;

			Self::deposit_event(Event::LiquidityAdded {
				id: pool_id,
				who,
				assets,
				amounts: (amount, asset_b_amount),
			});

			Ok(())
		}

		/// Remove liquidity from selected 2-asset pool in the form of burning shares.
		///
		/// LP receives certain amount of both assets.
		///
		/// Partial withdrawal is allowed.
		///
		/// Parameters:
		/// - `origin`: origin of the caller
		/// - `pool_id`: Pool Id
		/// - `amount`: Amount of shares to burn
		///
		/// Emits `LiquidityRemoved` event when successful.
		#[pallet::weight(<T as Config>::WeightInfo::remove_liquidity())]
		#[transactional]
		pub fn remove_liquidity(origin: OriginFor<T>, pool_id: PoolId<T::AssetId>, amount: Balance) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(amount > Balance::zero(), Error::<T>::InvalidAssetAmount);

			ensure!(
				T::Currency::free_balance(pool_id.0, &who) >= amount,
				Error::<T>::InsufficientShares
			);

			let pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let pool_account = T::ShareAccountId::from_assets(&pool.assets, Some(POOL_IDENTIFIER));

			let initial_reserves = AssetAmounts(
				T::Currency::free_balance(pool.assets.0, &pool_account),
				T::Currency::free_balance(pool.assets.1, &pool_account),
			);

			let share_issuance = T::Currency::total_issuance(pool_id.0);

			ensure!(
				share_issuance.saturating_sub(amount) >= T::MinPoolLiquidity::get(),
				Error::<T>::InsufficientLiquidityRemaining
			);

			let amounts = calculate_remove_liquidity_amounts(&initial_reserves, amount, share_issuance)
				.ok_or(ArithmeticError::Overflow)?;

			// burn `amount` of shares
			T::Currency::withdraw(pool_id.0, &who, amount)?;

			// Assets are ordered by id in pool.assets. So amounts provided corresponds.
			T::Currency::transfer(pool.assets.0, &pool_account, &who, amounts.0)?;
			T::Currency::transfer(pool.assets.1, &pool_account, &who, amounts.1)?;

			Self::deposit_event(Event::LiquidityRemoved {
				id: pool_id,
				who,
				shares: amount,
				amounts: (amounts.0, amounts.1),
			});

			Ok(())
		}

		/// Execute a swap of `asset_in` for `asset_out`.
		///
		/// Parameters:
		/// - `origin`:
		/// - `pool_id`: Id of a pool
		/// - `asset_in`: ID of asset sold to the pool
		/// - `asset_out`: ID of asset bought from the pool
		/// - `amount`: Amount of asset sold
		/// - `min_buy_amount`: Minimum amount required to receive
		///
		/// Emits `SellExecuted` event when successful.
		///
		#[pallet::weight(<T as Config>::WeightInfo::sell())]
		#[transactional]
		pub fn sell(
			origin: OriginFor<T>,
			pool_id: PoolId<T::AssetId>,
			asset_in: T::AssetId,
			asset_out: T::AssetId,
			amount_in: Balance,
			min_buy_amount: Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				amount_in >= T::MinTradingLimit::get(),
				Error::<T>::InsufficientTradingAmount
			);

			ensure!(
				T::Currency::free_balance(asset_in, &who) >= amount_in,
				Error::<T>::InsufficientBalance
			);

			let pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;

			ensure!(pool.assets.contains(asset_in), Error::<T>::AssetNotInPool);
			ensure!(pool.assets.contains(asset_out), Error::<T>::AssetNotInPool);

			let pool_account = T::ShareAccountId::from_assets(&pool.assets, Some(POOL_IDENTIFIER));

			let reserve_in = T::Currency::free_balance(asset_in, &pool_account);
			let reserve_out = T::Currency::free_balance(asset_out, &pool_account);

			let amount_out = calculate_out_given_in(
				reserve_in,
				reserve_out,
				amount_in,
				T::Precision::get(),
				pool.amplification.into(),
			)
			.ok_or(ArithmeticError::Overflow)?;

			let fee_amount = Self::calculate_fee_amount(amount_out, pool.fee, Rounding::Down);

			let amount_out = amount_out.checked_sub(fee_amount).ok_or(ArithmeticError::Underflow)?;

			ensure!(amount_out >= min_buy_amount, Error::<T>::BuyLimitNotReached);

			T::Currency::transfer(asset_in, &who, &pool_account, amount_in)?;
			T::Currency::transfer(asset_out, &pool_account, &who, amount_out)?;

			Self::deposit_event(Event::SellExecuted {
				who,
				asset_in,
				asset_out,
				amount_in,
				amount_out,
				fee: fee_amount,
			});

			Ok(())
		}

		/// Execute a swap of `asset_out` for `asset_in`.
		///
		/// Parameters:
		/// - `origin`:
		/// - `pool_id`: Id of a pool
		/// - `asset_out`: ID of asset bought from the pool
		/// - `asset_in`: ID of asset sold to the pool
		/// - `amount`: Amount of asset sold
		/// - `max_sell_amount`: Maximum amount allowedto be sold
		///
		/// Emits `buyExecuted` event when successful.
		///
		#[pallet::weight(<T as Config>::WeightInfo::buy())]
		#[transactional]
		pub fn buy(
			origin: OriginFor<T>,
			pool_id: PoolId<T::AssetId>,
			asset_out: T::AssetId,
			asset_in: T::AssetId,
			amount_out: Balance,
			max_sell_amount: Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				amount_out >= T::MinTradingLimit::get(),
				Error::<T>::InsufficientTradingAmount
			);

			let pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;

			ensure!(pool.assets.contains(asset_in), Error::<T>::AssetNotInPool);
			ensure!(pool.assets.contains(asset_out), Error::<T>::AssetNotInPool);

			let pool_account = T::ShareAccountId::from_assets(&pool.assets, Some(POOL_IDENTIFIER));

			let reserve_in = T::Currency::free_balance(asset_in, &pool_account);
			let reserve_out = T::Currency::free_balance(asset_out, &pool_account);

			ensure!(reserve_out > amount_out, Error::<T>::InsufficientLiquidity);

			let amount_in = calculate_in_given_out(
				reserve_in,
				reserve_out,
				amount_out,
				T::Precision::get(),
				pool.amplification.into(),
			)
			.ok_or(ArithmeticError::Overflow)?;

			let fee_amount = Self::calculate_fee_amount(amount_in, pool.fee, Rounding::Up);

			let amount_in = amount_in.checked_add(fee_amount).ok_or(ArithmeticError::Overflow)?;

			ensure!(amount_in <= max_sell_amount, Error::<T>::SellLimitExceeded);

			ensure!(
				T::Currency::free_balance(asset_in, &who) >= amount_in,
				Error::<T>::InsufficientBalance
			);

			T::Currency::transfer(asset_in, &who, &pool_account, amount_in)?;
			T::Currency::transfer(asset_out, &pool_account, &who, amount_out)?;

			Self::deposit_event(Event::BuyExecuted {
				who,
				asset_in,
				asset_out,
				amount_in,
				amount_out,
				fee: fee_amount,
			});

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
}

/// Rounding method to use
enum Rounding {
	Down,
	Up,
}

impl<T: Config> Pallet<T> {
	fn calculate_fee_amount(amount: Balance, fee: Permill, rounding: Rounding) -> Balance {
		match rounding {
			Rounding::Down => fee.mul_floor(amount),
			Rounding::Up => fee.mul_ceil(amount),
		}
	}
}
