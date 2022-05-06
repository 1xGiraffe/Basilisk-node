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

//! Autogenerated weights for pallet_exchange
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
// --pallet=pallet_exchange
// --output=exchange.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_exchange::weights::WeightInfo;

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	fn known_overhead_for_on_finalize() -> Weight {
		(10_094_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn sell_intention() -> Weight {
		(93_292_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn buy_intention() -> Weight {
		(95_420_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_finalize(t: u32) -> Weight {
		(0 as Weight) // Standard Error: 59_000
			.saturating_add((66_175_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn on_finalize_buys_no_matches(t: u32) -> Weight {
		(0 as Weight) // Standard Error: 67_000
			.saturating_add((118_616_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(t as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn on_finalize_sells_no_matches(t: u32) -> Weight {
		(6_740_000 as Weight) // Standard Error: 73_000
			.saturating_add((108_168_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(t as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn sell_extrinsic() -> Weight {
		(129_608_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn on_finalize_for_one_sell_extrinsic() -> Weight {
		(165_468_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn buy_extrinsic() -> Weight {
		(129_766_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn on_finalize_for_one_buy_extrinsic() -> Weight {
		(175_645_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}