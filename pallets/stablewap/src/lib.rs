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

//! # Stableswap pallet
//!
//! TBD
//!
//!
//! Questions:
//! 1. who can create pools
//! 2. pool owner needed to know ?
//! 3. creation fee?
//! 4. fees = trade fee and admin fee?!
//! 5.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::DispatchResult;
use frame_support::transactional;

mod math;
mod traits;
mod types;
pub mod weights;

pub use pallet::*;
use weights::WeightInfo;

const POOL_IDENTIFIER: &str = "sts";

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::traits::ShareAccountIdFor;
	use crate::types::{order_assets_amounts, Balance, FixedBalance, PoolAssets, PoolInfo};
	use codec::HasCompact;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use hydradx_traits::{Registry, ShareTokenRegistry};
	use math::calculate_add_liquidity_changes;
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

		type ShareAccountId: ShareAccountIdFor<PoolAssets<Self::AssetId>, AccountId = Self::AccountId>;

		type AssetRegistry: ShareTokenRegistry<Self::AssetId, Vec<u8>, Balance, DispatchError>;

		/// The origin which can create a new pool
		type CreatePoolOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: weights::WeightInfo;
	}

	/// Existing pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> =
		StorageMap<_, Blake2_128Concat, PoolAssets<T::AssetId>, PoolInfo<T::AssetId, Balance, FixedBalance>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A pool was created.
		PoolCreated {
			assets: PoolAssets<T::AssetId>,
			amplification: FixedBalance,
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

		/// One or more assets are not registered in AssetRegistry
		AssetNotRegistered,

		/// Invalid asset amounts provided. Amounts must be greater than zero.
		InvalidAssetAmounts,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::create_pool())]
		#[transactional]
		pub fn create_pool(
			origin: OriginFor<T>,
			assets: (T::AssetId, T::AssetId),
			amplification: FixedBalance,
			fee: Permill,
		) -> DispatchResult {
			T::CreatePoolOrigin::ensure_origin(origin)?;

			let pool_assets: PoolAssets<T::AssetId> = assets.into();

			ensure!(pool_assets.is_valid(), Error::<T>::SameAssets);

			for asset in (&pool_assets).into_iter() {
				ensure!(T::AssetRegistry::exists(asset), Error::<T>::AssetNotRegistered);
			}

			Pools::<T>::try_mutate(&pool_assets, |maybe_pool| -> DispatchResult {
				ensure!(maybe_pool.is_none(), Error::<T>::PoolExists);

				let share_asset_ident = T::ShareAccountId::name(&pool_assets, Some(POOL_IDENTIFIER));

				let share_asset = T::AssetRegistry::get_or_create_shared_asset(
					share_asset_ident,
					(&pool_assets).into(),
					Balance::zero(),
				)?;

				*maybe_pool = Some(PoolInfo {
					share_asset,
					amplification,
					balances: Default::default(),
					fee,
				});

				Self::deposit_event(Event::PoolCreated {
					assets: pool_assets.clone(),
					amplification,
				});

				Ok(())
			})
		}

		#[pallet::weight(<T as Config>::WeightInfo::create_pool())]
		#[transactional]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			assets: (T::AssetId, T::AssetId),
			amounts: (Balance, Balance),
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (pool_assets, assets_amounts) = order_assets_amounts(assets, amounts);

			ensure!(assets_amounts.valid(), Error::<T>::InvalidAssetAmounts);

			// TODO: check if balances of who are sufficient

			Pools::<T>::try_mutate(&pool_assets, |maybe_pool| -> DispatchResult {
				let pool = maybe_pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

				let delta_changes =
					calculate_add_liquidity_changes(pool, &assets_amounts).ok_or(ArithmeticError::Overflow)?;

				let pool_account = T::ShareAccountId::from_assets(&pool_assets, Some(POOL_IDENTIFIER));

				pool.add_amounts(&delta_changes.delta_amounts)
					.ok_or(ArithmeticError::Overflow)?;

				T::Currency::deposit(pool.share_asset, &who, delta_changes.share_amount)?;

				for (asset, amount) in (&pool_assets)
					.into_iter()
					.zip((&delta_changes.delta_amounts).into_iter())
				{
					T::Currency::transfer(asset, &who, &pool_account, amount)?;
				}

				Ok(())
			})
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
}
