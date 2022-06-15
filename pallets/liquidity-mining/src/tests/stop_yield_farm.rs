// This file is part of Basilisk-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use pretty_assertions::assert_eq;
use sp_runtime::FixedPointNumber;
use test_ext::*;
use warehouse_liquidity_mining::GlobalFarmData;
use warehouse_liquidity_mining::YieldFarmData;
use warehouse_liquidity_mining::YieldFarmState;

#[test]
fn stop_yield_farm_should_work() {
	let bsx_tkn1_assets = AssetPair {
		asset_in: BSX,
		asset_out: TKN1,
	};

	//same period
	predefined_test_ext_with_deposits().execute_with(|| {
		let yield_farm_account = WarehouseLM::farm_account_id(BSX_TKN1_YIELD_FARM_ID).unwrap();
		let global_farm_account = WarehouseLM::farm_account_id(GC_FARM).unwrap();
		let yield_farm_bsx_balance = Tokens::free_balance(BSX, &yield_farm_account);
		let global_farm_bsx_balance = Tokens::free_balance(BSX, &global_farm_account);
		let yield_farm = WarehouseLM::yield_farm((BSX_TKN1_AMM, GC_FARM, BSX_TKN1_YIELD_FARM_ID)).unwrap();
		let global_farm = WarehouseLM::global_farm(GC_FARM).unwrap();

		assert_ok!(LiquidityMining::stop_yield_farm(
			Origin::signed(GC),
			GC_FARM,
			bsx_tkn1_assets
		));

		expect_events(vec![mock::Event::LiquidityMining(Event::LiquidityMiningCanceled {
			farm_id: GC_FARM,
			liq_pool_farm_id: BSX_TKN1_YIELD_FARM_ID,
			who: GC,
			asset_pair: bsx_tkn1_assets,
		})]);

		let stake_in_global_farm = yield_farm
			.multiplier
			.checked_mul_int(yield_farm.total_valued_shares)
			.unwrap();

		assert_eq!(
			WarehouseLM::yield_farm((BSX_TKN1_AMM, GC_FARM, BSX_TKN1_YIELD_FARM_ID)).unwrap(),
			YieldFarmData {
				state: YieldFarmState::Stopped,
				multiplier: 0.into(),
				..yield_farm
			}
		);

		assert_eq!(
			WarehouseLM::global_farm(GC_FARM).unwrap(),
			GlobalFarmData {
				total_shares_z: global_farm.total_shares_z.checked_sub(stake_in_global_farm).unwrap(),
				..global_farm
			}
		);

		assert_eq!(Tokens::free_balance(BSX, &yield_farm_account), yield_farm_bsx_balance);
		assert_eq!(Tokens::free_balance(BSX, &global_farm_account), global_farm_bsx_balance);
	});

	//stop yield farm with farm update
	predefined_test_ext_with_deposits().execute_with(|| {
		let yield_farm_account = WarehouseLM::farm_account_id(BSX_TKN1_YIELD_FARM_ID).unwrap();
		let global_farm_account = WarehouseLM::farm_account_id(GC_FARM).unwrap();
		let yield_farm_bsx_balance = Tokens::free_balance(BSX, &yield_farm_account);
		let global_farm_bsx_balance = Tokens::free_balance(BSX, &global_farm_account);
		let yield_farm = WarehouseLM::yield_farm((BSX_TKN1_AMM, GC_FARM, BSX_TKN1_YIELD_FARM_ID)).unwrap();

		let global_farm = WarehouseLM::global_farm(GC_FARM).unwrap();

		set_block_number(10_000);

		assert_ok!(LiquidityMining::stop_yield_farm(
			Origin::signed(GC),
			GC_FARM,
			bsx_tkn1_assets
		));

		expect_events(vec![mock::Event::LiquidityMining(Event::LiquidityMiningCanceled {
			farm_id: GC_FARM,
			liq_pool_farm_id: BSX_TKN1_YIELD_FARM_ID,
			who: GC,
			asset_pair: bsx_tkn1_assets,
		})]);

		assert_eq!(
			WarehouseLM::yield_farm((BSX_TKN1_AMM, GC_FARM, BSX_TKN1_YIELD_FARM_ID)).unwrap(),
			YieldFarmData {
				updated_at: 100,
				accumulated_rpvs: 245,
				accumulated_rpz: 49,
				state: YieldFarmState::Stopped,
				multiplier: 0.into(),
				..yield_farm
			}
		);

		let stake_in_global_farm = yield_farm
			.multiplier
			.checked_mul_int(yield_farm.total_valued_shares)
			.unwrap();

		assert_eq!(
			WarehouseLM::global_farm(GC_FARM).unwrap(),
			GlobalFarmData {
				updated_at: 100,
				accumulated_rpz: 49,
				total_shares_z: global_farm.total_shares_z.checked_sub(stake_in_global_farm).unwrap(),
				accumulated_rewards: 18_206_375,
				paid_accumulated_rewards: 9_589_300,
				..global_farm
			}
		);

		assert_eq!(
			Tokens::free_balance(BSX, &yield_farm_account),
			yield_farm_bsx_balance + 8_424_900 //8_424_900 - yield farm last claim from global farm
		);

		assert_eq!(
			Tokens::free_balance(BSX, &global_farm_account),
			global_farm_bsx_balance - 8_424_900 //8_424_900 - yield farm last claim from global farm
		);
	});
}

#[test]
fn stop_yield_farm_should_not_fail_when_yield_farm_is_invalid() {
	let bsx_dot_assets = AssetPair {
		asset_in: BSX,
		asset_out: DOT,
	};

	predefined_test_ext_with_deposits().execute_with(|| {
		assert_noop!(
			LiquidityMining::stop_yield_farm(Origin::signed(GC), GC_FARM, bsx_dot_assets),
			warehouse_liquidity_mining::Error::<Test>::YieldFarmNotFound
		);
	});
}

#[test]
fn stop_yield_farm_should_fail_when_yield_farm_is_already_stopped() {
	let bsx_tkn1_assets = AssetPair {
		asset_in: BSX,
		asset_out: TKN1,
	};

	predefined_test_ext_with_deposits().execute_with(|| {
		assert_ok!(LiquidityMining::stop_yield_farm(
			Origin::signed(GC),
			GC_FARM,
			bsx_tkn1_assets
		));

		assert_noop!(
			LiquidityMining::stop_yield_farm(Origin::signed(GC), GC_FARM, bsx_tkn1_assets),
			warehouse_liquidity_mining::Error::<Test>::YieldFarmNotFound
		);
	});
}

#[test]
fn stop_yield_farm_not_owner_should_not_work() {
	let bsx_tkn1_assets = AssetPair {
		asset_in: BSX,
		asset_out: TKN1,
	};

	predefined_test_ext_with_deposits().execute_with(|| {
		const NOT_LIQ_POOL_OWNER: u128 = ALICE;

		assert_noop!(
			LiquidityMining::stop_yield_farm(Origin::signed(NOT_LIQ_POOL_OWNER), GC_FARM, bsx_tkn1_assets),
			warehouse_liquidity_mining::Error::<Test>::Forbidden
		);
	});
}