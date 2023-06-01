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

//! Autogenerated weights for pallet_lbp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --pallet=pallet-lbp
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// lbp.rs
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

use pallet_lbp::weights::WeightInfo;

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	// Storage: LBP PoolData (r:1 w:1)
	// Proof Skipped: LBP PoolData (max_values: None, max_size: None, mode: Measured)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:1)
	// Proof Skipped: LBP FeeCollectorWithAsset (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn create_pool() -> Weight {
		// Minimum execution time: 69_643 nanoseconds.
		Weight::from_ref_time(70_487_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: LBP PoolData (r:1 w:1)
	// Proof Skipped: LBP PoolData (max_values: None, max_size: None, mode: Measured)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:2)
	// Proof Skipped: LBP FeeCollectorWithAsset (max_values: None, max_size: None, mode: Measured)
	fn update_pool_data() -> Weight {
		// Minimum execution time: 17_606 nanoseconds.
		Weight::from_ref_time(17_866_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof Skipped: LBP PoolData (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 54_416 nanoseconds.
		Weight::from_ref_time(54_978_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: LBP PoolData (r:1 w:1)
	// Proof Skipped: LBP PoolData (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:0)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:0 w:1)
	// Proof Skipped: LBP FeeCollectorWithAsset (max_values: None, max_size: None, mode: Measured)
	fn remove_liquidity() -> Weight {
		// Minimum execution time: 68_270 nanoseconds.
		Weight::from_ref_time(69_701_000 as u64)
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: LBP PoolData (r:1 w:0)
	// Proof Skipped: LBP PoolData (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn sell() -> Weight {
		// Minimum execution time: 91_282 nanoseconds.
		Weight::from_ref_time(94_560_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof Skipped: LBP PoolData (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn buy() -> Weight {
		// Minimum execution time: 91_220 nanoseconds.
		Weight::from_ref_time(92_250_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
}
