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


//! Autogenerated weights for `pallet_elections_phragmen`
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
// pallet_elections_phragmen
// --output=./weights/pallet_elections_phragmen.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_elections_phragmen` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for BasiliskWeight<T> {
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 10]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + v * (85 ±0)`
		//  Estimated: `4764 + v * (87 ±0)`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(25_332_904, 4764)
			// Standard Error: 6_260
			.saturating_add(Weight::from_parts(167_145, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 87).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 10]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + v * (80 ±0)`
		//  Estimated: `4764 + v * (86 ±0)`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(34_936_209, 4764)
			// Standard Error: 7_613
			.saturating_add(Weight::from_parts(233_434, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 86).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 10]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273 + v * (80 ±0)`
		//  Estimated: `4764 + v * (86 ±0)`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(34_972_420, 4764)
			// Standard Error: 6_712
			.saturating_add(Weight::from_parts(221_908, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 86).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `602`
		//  Estimated: `4764`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 4764)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 100]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1217 + c * (48 ±0)`
		//  Estimated: `2703 + c * (48 ±0)`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_250_310, 2703)
			// Standard Error: 509
			.saturating_add(Weight::from_parts(33_998, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 100]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264 + c * (48 ±0)`
		//  Estimated: `1744 + c * (48 ±0)`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_093_706, 1744)
			// Standard Error: 620
			.saturating_add(Weight::from_parts(18_184, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1268`
		//  Estimated: `2753`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 2753)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `828`
		//  Estimated: `2313`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(21_000_000, 2313)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 500_000_000_000 picoseconds.
		Weight::from_parts(500_000_000_000, 0)
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1268`
		//  Estimated: `3593`
		// Minimum execution time: 46_000_000 picoseconds.
		Weight::from_parts(47_000_000, 3593)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Elections::Voting` (r:385 w:384)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:384 w:384)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:384 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:384 w:384)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[384, 768]`.
	/// The range of component `d` is `[0, 384]`.
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + d * (628 ±0) + v * (56 ±0)`
		//  Estimated: `78676 + d * (3774 ±2) + v * (10 ±0)`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(496_259_094, 78676)
			// Standard Error: 196_415
			.saturating_add(Weight::from_parts(46_455_799, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(d.into())))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:769 w:0)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:84 w:84)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Elections::ElectionRounds` (r:1 w:1)
	/// Proof: `Elections::ElectionRounds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 100]`.
	/// The range of component `v` is `[1, 768]`.
	/// The range of component `e` is `[768, 7680]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (4 ±0) + e * (29 ±0) + v * (414 ±0)`
		//  Estimated: `278441 + c * (2346 ±4) + e * (13 ±0) + v * (2557 ±3)`
		// Minimum execution time: 2_499_000_000 picoseconds.
		Weight::from_parts(2_506_000_000, 278441)
			// Standard Error: 213_760
			.saturating_add(Weight::from_parts(10_087_217, 0).saturating_mul(v.into()))
			// Standard Error: 22_298
			.saturating_add(Weight::from_parts(620_378, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(39_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2346).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 13).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2557).saturating_mul(v.into()))
	}
}