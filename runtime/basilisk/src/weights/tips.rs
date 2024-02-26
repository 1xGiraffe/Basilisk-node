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
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-26, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-tips
// --output=weights-1.1.0/tips.rs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;
use pallet_tips::WeightInfo;

/// Weights for `pallet_tips` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	/// Storage: `Tips::Reasons` (r:1 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[0, 1024]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3468`
		// Minimum execution time: 39_914_000 picoseconds.
		Weight::from_parts(40_759_457, 3468)
			// Standard Error: 51
			.saturating_add(Weight::from_parts(1_534, 0).saturating_mul(r.into()))
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
		// Minimum execution time: 37_195_000 picoseconds.
		Weight::from_parts(37_632_000, 3685)
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
		// Minimum execution time: 26_974_000 picoseconds.
		Weight::from_parts(26_937_982, 3703)
			// Standard Error: 37
			.saturating_add(Weight::from_parts(1_790, 0).saturating_mul(r.into()))
			// Standard Error: 6_024
			.saturating_add(Weight::from_parts(97_556, 0).saturating_mul(t.into()))
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
		// Minimum execution time: 21_124_000 picoseconds.
		Weight::from_parts(21_367_368, 3924)
			// Standard Error: 5_283
			.saturating_add(Weight::from_parts(197_604, 0).saturating_mul(t.into()))
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
		//  Estimated: `3956 + t * (108 ±0)`
		// Minimum execution time: 80_021_000 picoseconds.
		Weight::from_parts(81_286_488, 3956)
			// Standard Error: 19_506
			.saturating_add(Weight::from_parts(63_273, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 108).saturating_mul(t.into()))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 7]`.
	fn slash_tip(_t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3733`
		// Minimum execution time: 20_188_000 picoseconds.
		Weight::from_parts(20_632_387, 3733)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}