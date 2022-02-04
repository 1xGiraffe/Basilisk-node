// This file is part of Basilisk-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-03, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/basilisk
// benchmark
// --chain=dev
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet_democracy
// --output=democracy.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_democracy::weights::WeightInfo;

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {	fn propose() -> Weight {
		(60_700_000 as Weight)			.saturating_add(T::DbWeight::get().reads(3 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}	fn second(s: u32, ) -> Weight {
		(34_526_000 as Weight)			// Standard Error: 2_000
			.saturating_add((170_000 as Weight).saturating_mul(s as Weight))			.saturating_add(T::DbWeight::get().reads(1 as Weight))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn vote_new(r: u32, ) -> Weight {
		(36_917_000 as Weight)			// Standard Error: 2_000
			.saturating_add((202_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(3 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}	fn vote_existing(r: u32, ) -> Weight {
		(37_413_000 as Weight)			// Standard Error: 2_000
			.saturating_add((192_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(3 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}	fn emergency_cancel() -> Weight {
		(22_325_000 as Weight)			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}	fn blacklist(p: u32, ) -> Weight {
		(56_444_000 as Weight)			// Standard Error: 52_000
			.saturating_add((753_000 as Weight).saturating_mul(p as Weight))			.saturating_add(T::DbWeight::get().reads(4 as Weight))			.saturating_add(T::DbWeight::get().writes(5 as Weight))	}	fn external_propose(v: u32, ) -> Weight {
		(11_348_000 as Weight)			// Standard Error: 1_000
			.saturating_add((76_000 as Weight).saturating_mul(v as Weight))			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn external_propose_majority() -> Weight {
		(2_641_000 as Weight)			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn external_propose_default() -> Weight {
		(2_617_000 as Weight)			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn fast_track() -> Weight {
		(24_448_000 as Weight)			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}	fn veto_external(v: u32, ) -> Weight {
		(24_754_000 as Weight)			// Standard Error: 1_000
			.saturating_add((98_000 as Weight).saturating_mul(v as Weight))			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}	fn cancel_proposal(p: u32, ) -> Weight {
		(57_781_000 as Weight)			// Standard Error: 8_000
			.saturating_add((411_000 as Weight).saturating_mul(p as Weight))			.saturating_add(T::DbWeight::get().reads(4 as Weight))			.saturating_add(T::DbWeight::get().writes(4 as Weight))	}	fn cancel_referendum() -> Weight {
		(14_687_000 as Weight)			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn cancel_queued(r: u32, ) -> Weight {
		(26_100_000 as Weight)			// Standard Error: 3_000
			.saturating_add((1_471_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}	fn on_initialize_base(r: u32, ) -> Weight {
		(4_859_000 as Weight)			// Standard Error: 16_000
			.saturating_add((4_231_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		(12_029_000 as Weight)			// Standard Error: 15_000
			.saturating_add((4_186_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(5 as Weight))			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn delegate(r: u32, ) -> Weight {
		(49_410_000 as Weight)			// Standard Error: 19_000
			.saturating_add((5_682_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(4 as Weight))			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))			.saturating_add(T::DbWeight::get().writes(4 as Weight))			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))	}	fn undelegate(r: u32, ) -> Weight {
		(24_580_000 as Weight)			// Standard Error: 20_000
			.saturating_add((5_621_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))			.saturating_add(T::DbWeight::get().writes(2 as Weight))			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))	}	fn clear_public_proposals() -> Weight {
		(2_827_000 as Weight)			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn note_preimage(b: u32, ) -> Weight {
		(29_421_000 as Weight)			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))			.saturating_add(T::DbWeight::get().reads(1 as Weight))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn note_imminent_preimage(b: u32, ) -> Weight {
		(24_980_000 as Weight)			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))			.saturating_add(T::DbWeight::get().reads(1 as Weight))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn reap_preimage(b: u32, ) -> Weight {
		(31_000_000 as Weight)			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))			.saturating_add(T::DbWeight::get().reads(1 as Weight))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn unlock_remove(r: u32, ) -> Weight {
		(32_637_000 as Weight)			// Standard Error: 2_000
			.saturating_add((103_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(3 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}	fn unlock_set(r: u32, ) -> Weight {
		(31_896_000 as Weight)			// Standard Error: 8_000
			.saturating_add((139_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(3 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}	fn remove_vote(r: u32, ) -> Weight {
		(16_623_000 as Weight)			// Standard Error: 5_000
			.saturating_add((171_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}	fn remove_other_vote(r: u32, ) -> Weight {
		(16_688_000 as Weight)			// Standard Error: 6_000
			.saturating_add((163_000 as Weight).saturating_mul(r as Weight))			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}}
