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

//! Autogenerated weights for pallet_tips
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --pallet=pallet-tips
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// tips.rs
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

use pallet_tips::weights::WeightInfo;

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 1024]`.
	fn report_awesome(r: u32) -> Weight {
		// Minimum execution time: 14_250 nanoseconds.
		Weight::from_ref_time(14_548_580 as u64) // Standard Error: 57
			.saturating_add(Weight::from_ref_time(1_035 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Reasons (r:0 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	fn retract_tip() -> Weight {
		// Minimum execution time: 14_572 nanoseconds.
		Weight::from_ref_time(14_877_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Members (r:1 w:0)
	// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tips Reasons (r:1 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Tips (r:0 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 1024]`.
	/// The range of component `t` is `[1, 7]`.
	fn tip_new(r: u32, t: u32) -> Weight {
		// Minimum execution time: 13_599 nanoseconds.
		Weight::from_ref_time(13_227_866 as u64) // Standard Error: 71
			.saturating_add(Weight::from_ref_time(1_368 as u64).saturating_mul(r as u64))
			// Standard Error: 11_820
			.saturating_add(Weight::from_ref_time(149_869 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Members (r:1 w:0)
	// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 7]`.
	fn tip(t: u32) -> Weight {
		// Minimum execution time: 12_293 nanoseconds.
		Weight::from_ref_time(12_611_583 as u64) // Standard Error: 14_351
			.saturating_add(Weight::from_ref_time(111_793 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	// Storage: Elections Members (r:1 w:0)
	// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tips Reasons (r:0 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 7]`.
	fn close_tip(t: u32) -> Weight {
		// Minimum execution time: 26_852 nanoseconds.
		Weight::from_ref_time(28_122_916 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	// Storage: Tips Reasons (r:0 w:1)
	// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 7]`.
	fn slash_tip(t: u32) -> Weight {
		// Minimum execution time: 10_851 nanoseconds.
		Weight::from_ref_time(11_291_916 as u64) // Standard Error: 6_387
			.saturating_add(Weight::from_ref_time(8_416 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
