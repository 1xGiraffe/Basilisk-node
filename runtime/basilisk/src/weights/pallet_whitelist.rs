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


//! Autogenerated weights for `pallet_whitelist`
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
// pallet_whitelist
// --output=./weights/pallet_whitelist.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_whitelist`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_whitelist` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_whitelist::WeightInfo for BasiliskWeight<T> {
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn whitelist_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3556`
		// Minimum execution time: 21_172_000 picoseconds.
		Weight::from_parts(21_673_000, 3556)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn remove_whitelisted_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88`
		//  Estimated: `3556`
		// Minimum execution time: 19_816_000 picoseconds.
		Weight::from_parts(20_204_000, 3556)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:1 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `Measured`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 4194294]`.
	fn dispatch_whitelisted_call(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168 + n * (1 ±0)`
		//  Estimated: `3631 + n * (1 ±0)`
		// Minimum execution time: 37_346_000 picoseconds.
		Weight::from_parts(37_589_000, 3631)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_180, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
	}
	/// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
	/// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88`
		//  Estimated: `3556`
		// Minimum execution time: 26_118_000 picoseconds.
		Weight::from_parts(26_907_975, 3556)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_320, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}