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


//! Autogenerated weights for `orml_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-08-28, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime
// target/release/wbuild/basilisk-runtime/basilisk_runtime.compact.compressed.wasm
// --extrinsic
//
// --heap-pages=4096
// --steps=20
// --repeat=50
// --template=scripts/pallet-weight-template.hbs
// --pallet
// orml_vesting
// --output=./weights/orml_vesting.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `orml_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `orml_vesting` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_vesting::WeightInfo for BasiliskWeight<T> {
	/// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	/// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(469), added: 2944, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn vested_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `6196`
		// Minimum execution time: 96_297_000 picoseconds.
		Weight::from_parts(97_330_000, 6196)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	/// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(469), added: 2944, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 15]`.
	fn claim(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `479 + i * (18 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 50_567_000 picoseconds.
		Weight::from_parts(51_975_340, 4764)
			// Standard Error: 3_365
			.saturating_add(Weight::from_parts(145_031, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Vesting::VestingSchedules` (r:0 w:1)
	/// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(469), added: 2944, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 15]`.
	fn update_vesting_schedules(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `4764`
		// Minimum execution time: 43_735_000 picoseconds.
		Weight::from_parts(45_065_971, 4764)
			// Standard Error: 2_536
			.saturating_add(Weight::from_parts(102_492, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}