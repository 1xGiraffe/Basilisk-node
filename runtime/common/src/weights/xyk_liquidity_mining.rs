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

//! Autogenerated weights for pallet_xyk_liquidity_mining
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --pallet=pallet-xyk-liquidity-mining
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// xyk_liquidity_mining.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_xyk_liquidity_mining::weights::WeightInfo;

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM FarmSequencer (r:1 w:1)
	// Proof: XYKWarehouseLM FarmSequencer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Duster AccountBlacklist (r:0 w:1)
	// Proof Skipped: Duster AccountBlacklist (max_values: None, max_size: None, mode: Measured)
	// Storage: XYKWarehouseLM GlobalFarm (r:0 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	fn create_global_farm() -> Weight {
		// Minimum execution time: 34_299 nanoseconds.
		Weight::from_ref_time(34_966_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_global_farm() -> Weight {
		// Minimum execution time: 40_121 nanoseconds.
		Weight::from_ref_time(40_766_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Duster AccountBlacklist (r:1 w:1)
	// Proof Skipped: Duster AccountBlacklist (max_values: None, max_size: None, mode: Measured)
	fn terminate_global_farm() -> Weight {
		// Minimum execution time: 36_998 nanoseconds.
		Weight::from_ref_time(37_409_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: XYK ShareToken (r:1 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM ActiveYieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM FarmSequencer (r:1 w:1)
	// Proof: XYKWarehouseLM FarmSequencer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:0 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn create_yield_farm() -> Weight {
		// Minimum execution time: 54_373 nanoseconds.
		Weight::from_ref_time(55_152_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: XYK ShareToken (r:1 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM ActiveYieldFarm (r:1 w:0)
	// Proof: XYKWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_yield_farm() -> Weight {
		// Minimum execution time: 57_805 nanoseconds.
		Weight::from_ref_time(58_410_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: XYKWarehouseLM ActiveYieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn stop_yield_farm() -> Weight {
		// Minimum execution time: 54_056 nanoseconds.
		Weight::from_ref_time(54_719_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: XYKWarehouseLM ActiveYieldFarm (r:1 w:0)
	// Proof: XYKWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn terminate_yield_farm() -> Weight {
		// Minimum execution time: 38_529 nanoseconds.
		Weight::from_ref_time(39_064_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: XYK ShareToken (r:1 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:4 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYK PoolAssets (r:1 w:0)
	// Proof: XYK PoolAssets (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: XYK TotalLiquidity (r:1 w:0)
	// Proof: XYK TotalLiquidity (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM DepositSequencer (r:1 w:1)
	// Proof: XYKWarehouseLM DepositSequencer (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: NFT Collections (r:1 w:0)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: NFT Items (r:0 w:1)
	// Proof: NFT Items (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM Deposit (r:0 w:1)
	// Proof: XYKWarehouseLM Deposit (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	fn deposit_shares() -> Weight {
		// Minimum execution time: 128_570 nanoseconds.
		Weight::from_ref_time(130_027_000 as u64)
			.saturating_add(T::DbWeight::get().reads(22 as u64))
			.saturating_add(T::DbWeight::get().writes(13 as u64))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: XYK ShareToken (r:1 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM Deposit (r:1 w:1)
	// Proof: XYKWarehouseLM Deposit (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: XYK PoolAssets (r:1 w:0)
	// Proof: XYK PoolAssets (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: XYK TotalLiquidity (r:1 w:0)
	// Proof: XYK TotalLiquidity (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:0)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn redeposit_shares() -> Weight {
		// Minimum execution time: 54_917 nanoseconds.
		Weight::from_ref_time(56_085_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM Deposit (r:1 w:1)
	// Proof: XYKWarehouseLM Deposit (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_rewards() -> Weight {
		// Minimum execution time: 69_618 nanoseconds.
		Weight::from_ref_time(70_151_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: XYK ShareToken (r:1 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM Deposit (r:1 w:1)
	// Proof: XYKWarehouseLM Deposit (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:4 w:4)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYK PoolAssets (r:1 w:0)
	// Proof: XYK PoolAssets (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	// Storage: NFT Items (r:0 w:1)
	// Proof: NFT Items (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn withdraw_shares() -> Weight {
		// Minimum execution time: 149_187 nanoseconds.
		Weight::from_ref_time(150_295_000 as u64)
			.saturating_add(T::DbWeight::get().reads(19 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
	// Storage: XYK ShareToken (r:1 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM ActiveYieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM YieldFarm (r:1 w:1)
	// Proof: XYKWarehouseLM YieldFarm (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	// Storage: XYKWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: XYKWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn resume_yield_farm() -> Weight {
		// Minimum execution time: 56_409 nanoseconds.
		Weight::from_ref_time(57_202_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
}
