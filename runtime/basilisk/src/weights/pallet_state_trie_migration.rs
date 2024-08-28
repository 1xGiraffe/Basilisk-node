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


//! Autogenerated weights for `pallet_state_trie_migration`
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
// pallet_state_trie_migration
// --output=./weights/pallet_state_trie_migration.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_state_trie_migration`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_state_trie_migration` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_state_trie_migration::WeightInfo for BasiliskWeight<T> {
	/// Storage: `StateTrieMigration::SignedMigrationMaxLimits` (r:1 w:0)
	/// Proof: `StateTrieMigration::SignedMigrationMaxLimits` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:0)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `StateTrieMigration::MigrationProcess` (r:1 w:1)
	/// Proof: `StateTrieMigration::MigrationProcess` (`max_values`: Some(1), `max_size`: Some(1042), added: 1537, mode: `MaxEncodedLen`)
	fn continue_migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12`
		//  Estimated: `3550`
		// Minimum execution time: 28_166_000 picoseconds.
		Weight::from_parts(29_044_000, 3550)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StateTrieMigration::SignedMigrationMaxLimits` (r:1 w:0)
	/// Proof: `StateTrieMigration::SignedMigrationMaxLimits` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn continue_migrate_wrong_witness() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1493`
		// Minimum execution time: 3_871_000 picoseconds.
		Weight::from_parts(4_034_000, 1493)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Balances::Holds` (r:1 w:0)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn migrate_custom_top_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3550`
		// Minimum execution time: 16_291_000 picoseconds.
		Weight::from_parts(16_845_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x666f6f` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x666f6f` (r:1 w:1)
	fn migrate_custom_top_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9`
		//  Estimated: `3550`
		// Minimum execution time: 90_436_000 picoseconds.
		Weight::from_parts(91_388_000, 3550)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Balances::Holds` (r:1 w:0)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn migrate_custom_child_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3550`
		// Minimum execution time: 16_617_000 picoseconds.
		Weight::from_parts(17_228_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x666f6f` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x666f6f` (r:1 w:1)
	fn migrate_custom_child_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41`
		//  Estimated: `3550`
		// Minimum execution time: 91_494_000 picoseconds.
		Weight::from_parts(92_455_000, 3550)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: UNKNOWN KEY `0x6b6579` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x6b6579` (r:1 w:1)
	/// The range of component `v` is `[1, 4194304]`.
	fn process_top_key(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + v * (1 ±0)`
		//  Estimated: `3448 + v * (1 ±0)`
		// Minimum execution time: 3_314_000 picoseconds.
		Weight::from_parts(3_413_000, 3448)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_039, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(v.into()))
	}
}