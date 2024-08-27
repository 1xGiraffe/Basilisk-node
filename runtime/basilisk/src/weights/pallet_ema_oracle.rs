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


//! Autogenerated weights for `pallet_ema_oracle`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-27, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `RR`, CPU: `<UNKNOWN>`
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
// pallet_ema_oracle
// --output=./weights/pallet_ema_oracle.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_ema_oracle`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_ema_oracle` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ema_oracle::WeightInfo for BasiliskWeight<T> {
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:1)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(481), added: 976, mode: `MaxEncodedLen`)
	fn add_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472`
		//  Estimated: `1966`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 1966)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:1)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(481), added: 976, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(4441), added: 4936, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:0 w:5)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn remove_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `488`
		//  Estimated: `5926`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 5926)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:0)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(4441), added: 4936, mode: `MaxEncodedLen`)
	fn on_finalize_no_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `5926`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(1_000_000, 5926)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(4441), added: 4936, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:145 w:145)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 29]`.
	fn on_finalize_multiple_tokens(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + b * (933 ±0)`
		//  Estimated: `5926 + b * (13260 ±0)`
		// Minimum execution time: 55_000_000 picoseconds.
		Weight::from_parts(4_299_179, 5926)
			// Standard Error: 15_256
			.saturating_add(Weight::from_parts(50_164_476, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 13260).saturating_mul(b.into()))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(4441), added: 4936, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 29]`.
	fn on_trade_multiple_tokens(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78 + b * (148 ±0)`
		//  Estimated: `5926`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_296_787, 5926)
			// Standard Error: 2_058
			.saturating_add(Weight::from_parts(260_728, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(4441), added: 4936, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 29]`.
	fn on_liquidity_changed_multiple_tokens(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78 + b * (148 ±0)`
		//  Estimated: `5926`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_362_529, 5926)
			// Standard Error: 2_119
			.saturating_add(Weight::from_parts(258_853, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482`
		//  Estimated: `6294`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 6294)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
}