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


//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-08-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --steps=50
// --repeat=20
// --template=scripts/pallet-weight-template.hbs
// --pallet
// pallet_collective
// --output=./weights/pallet_collective

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_collective` using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for BasiliskWeight<T> {
	/// Storage: `TechnicalCommittee::Members` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:20 w:20)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 20]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (672 ±0) + p * (310 ±0)`
		//  Estimated: `4236 + m * (398 ±4) + p * (2602 ±2)`
		// Minimum execution time: 9_127_000 picoseconds.
		Weight::from_parts(9_437_000, 4236)
			// Standard Error: 91_387
			.saturating_add(Weight::from_parts(2_713_745, 0).saturating_mul(m.into()))
			// Standard Error: 46_214
			.saturating_add(Weight::from_parts(4_238_011, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 398).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 2602).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8 + m * (32 ±0)`
		//  Estimated: `1492 + m * (32 ±0)`
		// Minimum execution time: 15_056_000 picoseconds.
		Weight::from_parts(15_457_369, 1492)
			// Standard Error: 28
			.saturating_add(Weight::from_parts(1_409, 0).saturating_mul(b.into()))
			// Standard Error: 3_043
			.saturating_add(Weight::from_parts(20_390, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:0)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8 + m * (32 ±0)`
		//  Estimated: `3472 + m * (32 ±0)`
		// Minimum execution time: 19_161_000 picoseconds.
		Weight::from_parts(19_361_751, 3472)
			// Standard Error: 31
			.saturating_add(Weight::from_parts(1_403, 0).saturating_mul(b.into()))
			// Standard Error: 3_377
			.saturating_add(Weight::from_parts(37_926, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalCount` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 20]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (32 ±0) + p * (55 ±0)`
		//  Estimated: `3426 + m * (38 ±0) + p * (54 ±0)`
		// Minimum execution time: 25_350_000 picoseconds.
		Weight::from_parts(26_705_209, 3426)
			// Standard Error: 78
			.saturating_add(Weight::from_parts(2_729, 0).saturating_mul(b.into()))
			// Standard Error: 9_306
			.saturating_add(Weight::from_parts(52_223, 0).saturating_mul(m.into()))
			// Standard Error: 4_099
			.saturating_add(Weight::from_parts(398_301, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 38).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 54).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 10]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `552 + m * (64 ±0)`
		//  Estimated: `4017 + m * (64 ±0)`
		// Minimum execution time: 23_770_000 picoseconds.
		Weight::from_parts(24_095_059, 4017)
			// Standard Error: 3_210
			.saturating_add(Weight::from_parts(51_920, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `97 + m * (64 ±0) + p * (55 ±0)`
		//  Estimated: `3562 + m * (66 ±0) + p * (55 ±0)`
		// Minimum execution time: 28_048_000 picoseconds.
		Weight::from_parts(28_749_435, 3562)
			// Standard Error: 6_379
			.saturating_add(Weight::from_parts(67_494, 0).saturating_mul(m.into()))
			// Standard Error: 2_149
			.saturating_add(Weight::from_parts(319_084, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41 + b * (1 ±0) + m * (64 ±0) + p * (78 ±0)`
		//  Estimated: `3635 + b * (1 ±0) + m * (55 ±1) + p * (74 ±0)`
		// Minimum execution time: 41_823_000 picoseconds.
		Weight::from_parts(43_780_760, 3635)
			// Standard Error: 116
			.saturating_add(Weight::from_parts(2_347, 0).saturating_mul(b.into()))
			// Standard Error: 6_076
			.saturating_add(Weight::from_parts(455_860, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 74).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117 + m * (64 ±0) + p * (55 ±0)`
		//  Estimated: `3582 + m * (66 ±0) + p * (55 ±0)`
		// Minimum execution time: 30_213_000 picoseconds.
		Weight::from_parts(31_273_020, 3582)
			// Standard Error: 6_950
			.saturating_add(Weight::from_parts(49_519, 0).saturating_mul(m.into()))
			// Standard Error: 2_341
			.saturating_add(Weight::from_parts(321_432, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `61 + b * (1 ±0) + m * (64 ±0) + p * (78 ±0)`
		//  Estimated: `3655 + b * (1 ±0) + m * (55 ±1) + p * (74 ±0)`
		// Minimum execution time: 44_990_000 picoseconds.
		Weight::from_parts(46_996_882, 3655)
			// Standard Error: 111
			.saturating_add(Weight::from_parts(2_287, 0).saturating_mul(b.into()))
			// Standard Error: 5_804
			.saturating_add(Weight::from_parts(456_028, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 74).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 20]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168 + p * (32 ±0)`
		//  Estimated: `1653 + p * (32 ±0)`
		// Minimum execution time: 17_330_000 picoseconds.
		Weight::from_parts(18_191_303, 1653)
			// Standard Error: 1_667
			.saturating_add(Weight::from_parts(275_395, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}