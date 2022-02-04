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

//! Autogenerated weights for pallet_lbp
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
// --pallet=pallet_lbp
// --output=lbp.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_lbp::weights::WeightInfo;

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {	fn create_pool() -> Weight {
		(140_984_000 as Weight)			.saturating_add(T::DbWeight::get().reads(10 as Weight))			.saturating_add(T::DbWeight::get().writes(7 as Weight))	}	fn update_pool_data() -> Weight {
		(34_779_000 as Weight)			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}	fn add_liquidity() -> Weight {
		(103_702_000 as Weight)			.saturating_add(T::DbWeight::get().reads(8 as Weight))			.saturating_add(T::DbWeight::get().writes(4 as Weight))	}	fn remove_liquidity() -> Weight {
		(122_228_000 as Weight)			.saturating_add(T::DbWeight::get().reads(9 as Weight))			.saturating_add(T::DbWeight::get().writes(7 as Weight))	}	fn sell() -> Weight {
		(183_378_000 as Weight)			.saturating_add(T::DbWeight::get().reads(12 as Weight))			.saturating_add(T::DbWeight::get().writes(7 as Weight))	}	fn buy() -> Weight {
		(183_014_000 as Weight)			.saturating_add(T::DbWeight::get().reads(12 as Weight))			.saturating_add(T::DbWeight::get().writes(7 as Weight))	}}
