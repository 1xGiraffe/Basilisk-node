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


//! Autogenerated weights for `pallet_tips`
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
// --pallet=pallet-tips
// --extrinsic=*
// --template=scripts/pallet-weight-template.hbs
// --output=./weights/pallet_tips.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_tips` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for BasiliskWeight<T> {
	/// Storage: `Tips::Reasons` (r:1 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[0, 1024]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3468`
		// Minimum execution time: 30_814_000 picoseconds.
		Weight::from_parts(31_696_561, 3468)
			// Standard Error: 24
			.saturating_add(Weight::from_parts(1_531, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn retract_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `220`
		//  Estimated: `3685`
		// Minimum execution time: 29_320_000 picoseconds.
		Weight::from_parts(29_715_000, 3685)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:1 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:0 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[0, 1024]`.
	/// The range of component `t` is `[1, 7]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238 + t * (64 ±0)`
		//  Estimated: `3703 + t * (64 ±0)`
		// Minimum execution time: 20_957_000 picoseconds.
		Weight::from_parts(21_144_909, 3703)
			// Standard Error: 14
			.saturating_add(Weight::from_parts(1_578, 0).saturating_mul(r.into()))
			// Standard Error: 2_221
			.saturating_add(Weight::from_parts(35_536, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(t.into()))
	}
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 7]`.
	fn tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459 + t * (112 ±0)`
		//  Estimated: `3924 + t * (112 ±0)`
		// Minimum execution time: 17_252_000 picoseconds.
		Weight::from_parts(17_558_205, 3924)
			// Standard Error: 3_253
			.saturating_add(Weight::from_parts(139_110, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 112).saturating_mul(t.into()))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 7]`.
	fn close_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459 + t * (112 ±0)`
		//  Estimated: `3956 + t * (107 ±0)`
		// Minimum execution time: 62_447_000 picoseconds.
		Weight::from_parts(63_625_920, 3956)
			// Standard Error: 7_882
			.saturating_add(Weight::from_parts(97_662, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 107).saturating_mul(t.into()))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 7]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3733`
		// Minimum execution time: 15_408_000 picoseconds.
		Weight::from_parts(15_836_324, 3733)
			// Standard Error: 1_630
			.saturating_add(Weight::from_parts(5_081, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}