// This file is part of Basilisk.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! Autogenerated weights for `pallet_xyk_liquidity_mining`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-28, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --chain=dev
// --steps=20
// --repeat=50
// --wasm-execution=compiled
// --pallet=pallet-xyk-liquidity-mining
// --extrinsic=*
// --template=scripts/pallet-weight-template.hbs
// --output=./weights/pallet_xyk_liquidity_mining.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_xyk_liquidity_mining`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_xyk_liquidity_mining` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xyk_liquidity_mining::WeightInfo for BasiliskWeight<T> {
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::FarmSequencer` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::FarmSequencer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:0 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:0 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	fn create_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `6196`
		// Minimum execution time: 70_662_000 picoseconds.
		Weight::from_parts(71_546_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn update_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `946`
		//  Estimated: `6196`
		// Minimum execution time: 75_179_000 picoseconds.
		Weight::from_parts(75_866_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn terminate_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1069`
		//  Estimated: `6196`
		// Minimum execution time: 76_085_000 picoseconds.
		Weight::from_parts(76_892_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::FarmSequencer` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::FarmSequencer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:0 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	fn create_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1280`
		//  Estimated: `6196`
		// Minimum execution time: 96_624_000 picoseconds.
		Weight::from_parts(97_731_000, 6196)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:0)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn update_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1428`
		//  Estimated: `6196`
		// Minimum execution time: 101_694_000 picoseconds.
		Weight::from_parts(102_478_000, 6196)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn stop_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1262`
		//  Estimated: `6196`
		// Minimum execution time: 95_747_000 picoseconds.
		Weight::from_parts(97_001_000, 6196)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:0)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn terminate_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `938`
		//  Estimated: `6196`
		// Minimum execution time: 80_289_000 picoseconds.
		Weight::from_parts(80_731_000, 6196)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:3 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:4 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:1 w:0)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:0)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::DepositSequencer` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::DepositSequencer` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `NFT::Collections` (r:1 w:0)
	/// Proof: `NFT::Collections` (`max_values`: None, `max_size`: Some(99), added: 2574, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::NextAssetId` (r:1 w:0)
	/// Proof: `AssetRegistry::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::LocationAssets` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `NFT::Items` (r:0 w:1)
	/// Proof: `NFT::Items` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:0 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	fn deposit_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3326`
		//  Estimated: `11402`
		// Minimum execution time: 208_452_000 picoseconds.
		Weight::from_parts(210_505_000, 11402)
			.saturating_add(T::DbWeight::get().reads(22_u64))
			.saturating_add(T::DbWeight::get().writes(13_u64))
	}
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:1 w:0)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:0)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:1 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn redeposit_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2397`
		//  Estimated: `3878`
		// Minimum execution time: 72_220_000 picoseconds.
		Weight::from_parts(73_352_000, 3878)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2164`
		//  Estimated: `8799`
		// Minimum execution time: 141_239_000 picoseconds.
		Weight::from_parts(142_524_000, 8799)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:4 w:4)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:1 w:0)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::NextAssetId` (r:1 w:0)
	/// Proof: `AssetRegistry::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::LocationAssets` (`max_values`: None, `max_size`: Some(622), added: 3097, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `NFT::Items` (r:0 w:1)
	/// Proof: `NFT::Items` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn withdraw_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3018`
		//  Estimated: `11402`
		// Minimum execution time: 303_183_000 picoseconds.
		Weight::from_parts(305_211_000, 11402)
			.saturating_add(T::DbWeight::get().reads(19_u64))
			.saturating_add(T::DbWeight::get().writes(15_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn resume_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1536`
		//  Estimated: `6196`
		// Minimum execution time: 99_110_000 picoseconds.
		Weight::from_parts(99_864_000, 6196)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}