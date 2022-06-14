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
use crate::mock::{
	asset_pair_to_map_key, set_block_number, BlockNumber, Event as TestEvent, ExtBuilder, LiquidityMining, Origin,
	Test, Tokens, WarehouseLM, ACA, ACA_FARM, ACA_KSM_AMM, ACA_KSM_SHARE_ID, ACCOUNT_WITH_1M, ALICE, AMM_POOLS, BOB,
	BSX, BSX_ACA_AMM, BSX_ACA_SHARE_ID, BSX_DOT_AMM, BSX_DOT_SHARE_ID, BSX_ETH_AMM, BSX_ETH_SHARE_ID, BSX_FARM,
	BSX_HDX_AMM, BSX_HDX_SHARE_ID, BSX_KSM_AMM, BSX_KSM_SHARE_ID, BSX_TKN1_AMM, BSX_TKN1_SHARE_ID, BSX_TKN2_AMM,
	BSX_TKN2_SHARE_ID, CHARLIE, DAVE, DOT, ETH, EVE, GC, GC_FARM, HDX, INITIAL_BALANCE, KSM, KSM_DOT_AMM,
	KSM_DOT_SHARE_ID, KSM_FARM, LIQ_MINING_NFT_CLASS, TKN1, TKN2, TREASURY,
};

use frame_support::{assert_noop, assert_ok};
use primitives::Balance;
use sp_runtime::traits::BadOrigin;

const ALICE_FARM: u32 = BSX_FARM;
const BOB_FARM: u32 = KSM_FARM;
const CHARLIE_FARM: u32 = ACA_FARM;
const DAVE_FARM: u32 = 5;
const EVE_FARM: u32 = 6;

use warehouse_liquidity_mining::Config;
use warehouse_liquidity_mining::GlobalFarmData;
use warehouse_liquidity_mining::GlobalFarmState;
use warehouse_liquidity_mining::LoyaltyCurve;
use warehouse_liquidity_mining::YieldFarmData;
use warehouse_liquidity_mining::YieldFarmState;

const PREDEFINED_GLOBAL_POOLS: [GlobalFarmData<Test>; 6] = [
	GlobalFarmData {
		id: ALICE_FARM,
		updated_at: 0,
		reward_currency: BSX,
		yield_per_period: Permill::from_percent(20),
		planned_yielding_periods: 300_u64,
		blocks_per_period: 1_000_u64,
		owner: ALICE,
		incentivized_asset: BSX,
		max_reward_per_period: 333_333_333,
		accumulated_rpz: 0,
		yield_farms_count: (0, 0),
		paid_accumulated_rewards: 0,
		total_shares_z: 0,
		accumulated_rewards: 0,
		state: GlobalFarmState::Active,
	},
	GlobalFarmData {
		id: BOB_FARM,
		updated_at: 0,
		reward_currency: KSM,
		yield_per_period: Permill::from_percent(38),
		planned_yielding_periods: 5_000_u64,
		blocks_per_period: 10_000_u64,
		owner: BOB,
		incentivized_asset: BSX,
		max_reward_per_period: 200_000,
		accumulated_rpz: 0,
		yield_farms_count: (0, 0),
		paid_accumulated_rewards: 0,
		total_shares_z: 0,
		accumulated_rewards: 0,
		state: GlobalFarmState::Active,
	},
	GlobalFarmData {
		id: GC_FARM,
		updated_at: 0,
		reward_currency: BSX,
		yield_per_period: Permill::from_percent(50),
		planned_yielding_periods: 500_u64,
		blocks_per_period: 100_u64,
		owner: GC,
		incentivized_asset: BSX,
		max_reward_per_period: 60_000_000,
		accumulated_rpz: 0,
		yield_farms_count: (2, 2),
		paid_accumulated_rewards: 0,
		total_shares_z: 0,
		accumulated_rewards: 0,
		state: GlobalFarmState::Active,
	},
	GlobalFarmData {
		id: CHARLIE_FARM,
		updated_at: 0,
		reward_currency: ACA,
		yield_per_period: Permill::from_percent(50),
		planned_yielding_periods: 500_u64,
		blocks_per_period: 100_u64,
		owner: CHARLIE,
		incentivized_asset: KSM,
		max_reward_per_period: 60_000_000,
		accumulated_rpz: 0,
		yield_farms_count: (2, 2),
		paid_accumulated_rewards: 0,
		total_shares_z: 0,
		accumulated_rewards: 0,
		state: GlobalFarmState::Active,
	},
	GlobalFarmData {
		id: DAVE_FARM,
		updated_at: 0,
		reward_currency: ACA,
		yield_per_period: Permill::from_percent(20),
		planned_yielding_periods: 300_u64,
		blocks_per_period: 1_000_u64,
		owner: DAVE,
		incentivized_asset: TKN1,
		max_reward_per_period: 333_333_333,
		accumulated_rpz: 0,
		yield_farms_count: (0, 0),
		paid_accumulated_rewards: 0,
		total_shares_z: 0,
		accumulated_rewards: 0,
		state: GlobalFarmState::Active,
	},
	GlobalFarmData {
		id: EVE_FARM,
		updated_at: 0,
		reward_currency: KSM,
		yield_per_period: Permill::from_percent(20),
		planned_yielding_periods: 300_u64,
		blocks_per_period: 1_000_u64,
		owner: EVE,
		incentivized_asset: BSX,
		max_reward_per_period: 333_333_333,
		accumulated_rpz: 0,
		yield_farms_count: (0, 0),
		paid_accumulated_rewards: 0,
		total_shares_z: 0,
		accumulated_rewards: 0,
		state: GlobalFarmState::Active,
	},
];

const BSX_TKN1_LIQ_POOL_ID: u32 = 7;
const BSX_TKN2_LIQ_POOL_ID: u32 = 8;
const ACA_KSM_LIQ_POOL_ID: u32 = 9;
const DAVE_BSX_TKN1_YIELD_FARM_ID: u32 = 10;
const EVE_BSX_TKN1_YIELD_FARM_ID: u32 = 11;
const EVE_BSX_TKN2_YIELD_FARM_ID: u32 = 12;

thread_local! {
	static PREDEFINED_LIQ_POOLS: [YieldFarmData<Test>; 6] = [
		YieldFarmData {
			id: BSX_TKN1_LIQ_POOL_ID,
			updated_at: 0,
			total_shares: 0,
			total_valued_shares: 0,
			accumulated_rpvs: 0,
			accumulated_rpz: 0,
			loyalty_curve: Some(LoyaltyCurve::default()),
			multiplier: FixedU128::from(5),
			state: YieldFarmState::Active,
			entries_count: 0
		},
		YieldFarmData {
			id: BSX_TKN2_LIQ_POOL_ID,
			updated_at: 0,
			total_shares: 0,
			total_valued_shares: 0,
			accumulated_rpvs: 0,
			accumulated_rpz: 0,
			loyalty_curve: Some(LoyaltyCurve::default()),
			multiplier: FixedU128::from(10),
			state: YieldFarmState::Active,
			entries_count: 0
		},
		YieldFarmData {
			id: ACA_KSM_LIQ_POOL_ID,
			updated_at: 0,
			total_shares: 0,
			total_valued_shares: 0,
			accumulated_rpvs: 0,
			accumulated_rpz: 0,
			loyalty_curve: Some(LoyaltyCurve::default()),
			multiplier: FixedU128::from(10),
			state: YieldFarmState::Active,
			entries_count: 0
		},
		YieldFarmData {
			id: DAVE_BSX_TKN1_YIELD_FARM_ID,
			updated_at: 0,
			total_shares: 0,
			total_valued_shares: 0,
			accumulated_rpvs: 0,
			accumulated_rpz: 0,
			loyalty_curve: Some(LoyaltyCurve::default()),
			multiplier: FixedU128::from(10),
			state: YieldFarmState::Active,
			entries_count: 0,
		},
		YieldFarmData {
			id: EVE_BSX_TKN1_YIELD_FARM_ID,
			updated_at: 0,
			total_shares: 0,
			total_valued_shares: 0,
			accumulated_rpvs: 0,
			accumulated_rpz: 0,
			loyalty_curve: Some(LoyaltyCurve::default()),
			multiplier: FixedU128::from(10),
			state: YieldFarmState::Active,
			entries_count: 0,
		},

		YieldFarmData {
			id: EVE_BSX_TKN2_YIELD_FARM_ID,
			updated_at: 0,
			total_shares: 0,
			total_valued_shares: 0,
			accumulated_rpvs: 0,
			accumulated_rpz: 0,
			loyalty_curve: Some(LoyaltyCurve::default()),
			multiplier: FixedU128::from(10),
			state: YieldFarmState::Active,
			entries_count: 0,
		},
	]
}

//TODO: Dani - do I still need it?
//nft_ids for deposits from "predefined_test_ext_with_deposits()"
const PREDEFINED_NFT_IDS: [u128; 7] = [
	4294967301,
	8589934597,
	12884901894,
	17179869190,
	21474836486,
	25769803782,
	30064771077,
];

const PREDEFINED_DEPOSIT_IDS: [u128; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

fn last_events(n: usize) -> Vec<TestEvent> {
	frame_system::Pallet::<Test>::events()
		.into_iter()
		.rev()
		.take(n)
		.rev()
		.map(|e| e.event)
		.collect()
}

fn expect_events(e: Vec<TestEvent>) {
	pretty_assertions::assert_eq!(last_events(e.len()), e);
}

pub mod cancel_liquidity_pool;
pub mod claim_rewards;
pub mod create_global_farm;
pub mod create_yield_farm;
pub mod deposit_shares;
pub mod destroy_global_farm;
pub mod redeposit_shares;
pub mod remove_liquidity_pool;
pub mod resume_liquidity_pool;
pub mod test_ext;
pub mod update_liquidity_pool;
pub mod withdraw_shares;
