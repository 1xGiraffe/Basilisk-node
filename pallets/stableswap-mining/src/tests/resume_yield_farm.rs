// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

#[test]
fn resume_yield_farm_should_work() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(ALICE, BSX, 1_000 * ONE),
			(ALICE, DAI, 1_000 * ONE),
			(GC, BSX, 2_000_000 * ONE),
		])
		.with_registered_asset("BSX".as_bytes().to_vec(), BSX)
		.with_registered_asset("DAI".as_bytes().to_vec(), DAI)
		.with_pool(
			ALICE,
			PoolInfo::<AssetId> {
				assets: vec![BSX, DAI].try_into().unwrap(),
				amplification: 100,
				trade_fee: Permill::from_percent(0),
				withdraw_fee: Permill::from_percent(0),
			},
			InitialLiquidity {
				account: ALICE,
				assets: vec![
					AssetLiquidity {
						asset_id: BSX,
						amount: 100 * ONE,
					},
					AssetLiquidity {
						asset_id: DAI,
						amount: 100 * ONE,
					},
				],
			},
		)
		.with_global_farm(
			100 * ONE,
			1_000_000,
			1_000,
			BSX,
			BSX,
			GC,
			Perquintill::from_float(0.2),
			1_000,
			One::one(),
		)
		.with_yield_farm(GC, GC_FARM, FixedU128::one(), None, 3, vec![BSX, DAI])
		.build()
		.execute_with(|| {
			let owner = GC;
			let global_farm_id = GC_FARM;
			let yield_farm_id = 2;
			let pool_id = get_pool_id_at(0);
			let new_multiplier = FixedU128::from_float(1.23);

			//Stop yield farm before test
			assert_ok!(StableswapMining::stop_yield_farm(
				Origin::signed(owner),
				global_farm_id,
				pool_id
			));

			assert_ok!(StableswapMining::resume_yield_farm(
				Origin::signed(owner),
				global_farm_id,
				yield_farm_id,
				pool_id,
				new_multiplier
			));

			assert_last_event!(crate::Event::YieldFarmResumed {
				who: owner,
				global_farm_id,
				yield_farm_id,
				pool_id,
				multiplier: new_multiplier
			}
			.into())
		});
}

#[test]
fn resume_yield_farm_should_fail_when_stableswap_pool_doesnt_exists() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(ALICE, BSX, 1_000 * ONE),
			(ALICE, DAI, 1_000 * ONE),
			(GC, BSX, 2_000_000 * ONE),
		])
		.with_registered_asset("BSX".as_bytes().to_vec(), BSX)
		.with_registered_asset("DAI".as_bytes().to_vec(), DAI)
		.with_pool(
			ALICE,
			PoolInfo::<AssetId> {
				assets: vec![BSX, DAI].try_into().unwrap(),
				amplification: 100,
				trade_fee: Permill::from_percent(0),
				withdraw_fee: Permill::from_percent(0),
			},
			InitialLiquidity {
				account: ALICE,
				assets: vec![
					AssetLiquidity {
						asset_id: BSX,
						amount: 100 * ONE,
					},
					AssetLiquidity {
						asset_id: DAI,
						amount: 100 * ONE,
					},
				],
			},
		)
		.with_global_farm(
			100 * ONE,
			1_000_000,
			1_000,
			BSX,
			BSX,
			GC,
			Perquintill::from_float(0.2),
			1_000,
			One::one(),
		)
		.with_yield_farm(GC, GC_FARM, FixedU128::one(), None, 3, vec![BSX, DAI])
		.build()
		.execute_with(|| {
			let owner = GC;
			let global_farm_id = GC_FARM;
			let yield_farm_id = 2;
			let pool_id = get_pool_id_at(0);
			let new_multiplier = FixedU128::from_float(1.23);

			//Stop yield farm before test
			assert_ok!(StableswapMining::stop_yield_farm(
				Origin::signed(owner),
				global_farm_id,
				pool_id
			));

			//Destroy stableswap pool - stableswap pallet doesn't allow poll destruction
			pallet_stableswap::Pools::<Test>::remove(pool_id);
			pretty_assertions::assert_eq!(pallet_stableswap::Pools::<Test>::get(pool_id).is_none(), true);

			assert_noop!(
				StableswapMining::resume_yield_farm(
					Origin::signed(owner),
					global_farm_id,
					yield_farm_id,
					pool_id,
					new_multiplier
				),
				Error::<Test>::StableswapPoolNotFound
			);
		});
}
