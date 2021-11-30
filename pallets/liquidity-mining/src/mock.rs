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
//
use crate as liq_mining;
use crate::Config;
use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild, Nothing},
	PalletId,
};
use frame_system as system;
use hydradx_traits::AMM;
use orml_traits::parameter_type_with_key;
use primitives::{asset::AssetPair, Amount, AssetId, Balance};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, BlockNumberProvider, IdentityLookup},
};
use std::{cell::RefCell, collections::HashMap};

type AccountId = u128;
pub type PoolId = crate::PoolId;
pub type BlockNumber = u64;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DAVE: AccountId = 4;
pub const TREASURY: AccountId = 5;
pub const ACC_1M: AccountId = 6;
pub const GC: AccountId = 7;

pub const INITIAL_BALANCE: u128 = 1_000_000_000_000;

pub const BSX_ACA_SHARE_ID: AssetId = 100;
pub const BSX_KSM_SHARE_ID: AssetId = 101;
pub const BSX_DOT_SHARE_ID: AssetId = 102;
pub const BSX_ETH_SHARE_ID: AssetId = 103;
pub const BSX_HDX_SHARE_ID: AssetId = 104;

pub const BSX_TO1_SHARE_ID: AssetId = 105;
pub const BSX_TO2_SHARE_ID: AssetId = 106;

pub const BSX: AssetId = 1000;
pub const HDX: AssetId = 2000;
pub const ACA: AssetId = 3000;
pub const KSM: AssetId = 4000;
pub const DOT: AssetId = 5000;
pub const ETH: AssetId = 6000;
pub const TO1: AssetId = 7_001;
pub const TO2: AssetId = 7_002;

pub const BSX_ACA_AMM: AccountId = 11_000;
pub const BSX_KSM_AMM: AccountId = 11_001;
pub const BSX_DOT_AMM: AccountId = 11_002;
pub const BSX_ETH_AMM: AccountId = 11_003;
pub const BSX_HDX_AMM: AccountId = 11_004;
pub const BSX_TO1_AMM: AccountId = 11_005;
pub const BSX_TO2_AMM: AccountId = 11_006;

pub const BSX_ACA_LM_POOL: PoolId = 12_000;
pub const BSX_KSM_LM_POOL: PoolId = 12_001;
pub const BSX_DOT_LM_POOL: PoolId = 12_002;

pub const BSX_FARM: PoolId = 1;
pub const KSM_FARM: PoolId = 2;
pub const GC_FARM: PoolId = 3;
pub const ACA_FARM: PoolId = 4;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
frame_support::construct_runtime!(
	pub enum Test where
	Block = Block,
	NodeBlock = Block,
	UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		LiquidityMining: liq_mining::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 63;
	pub static MockBlockNumberProvider: u64 = 0;
}

impl BlockNumberProvider for MockBlockNumberProvider {
	type BlockNumber = u64;

	fn current_block_number() -> Self::BlockNumber {
		Self::get()
	}
}
impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

pub struct Amm;

thread_local! {
	pub static AMM_POOLS: RefCell<HashMap<String, (AccountId, AssetId)>> = RefCell::new(HashMap::new());
}

impl AMM<AccountId, AssetId, AssetPair, Balance> for Amm {
	fn get_max_out_ratio() -> u128 {
		0_u32.into()
	}

	fn get_max_in_ratio() -> u128 {
		0_u32.into()
	}

	fn get_pool_assets(_pool_account_id: &AccountId) -> Option<Vec<AssetId>> {
		None
	}

	fn get_spot_price_unchecked(_asset_a: AssetId, _asset_b: AssetId, _amount: Balance) -> Balance {
		Balance::from(0_u32)
	}

	fn validate_sell(
		_origin: &AccountId,
		_assets: AssetPair,
		_amount: Balance,
		_min_bought: Balance,
		_discount: bool,
	) -> Result<
		hydradx_traits::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
		frame_support::sp_runtime::DispatchError,
	> {
		Err(sp_runtime::DispatchError::Other("NotImplemented"))
	}

	fn execute_buy(
		_transfer: &hydradx_traits::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
	) -> frame_support::dispatch::DispatchResult {
		Err(sp_runtime::DispatchError::Other("NotImplemented"))
	}

	fn execute_sell(
		_transfer: &hydradx_traits::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
	) -> frame_support::dispatch::DispatchResult {
		Err(sp_runtime::DispatchError::Other("NotImplemented"))
	}

	fn validate_buy(
		_origin: &AccountId,
		_assets: AssetPair,
		_amount: Balance,
		_max_limit: Balance,
		_discount: bool,
	) -> Result<
		hydradx_traits::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
		frame_support::sp_runtime::DispatchError,
	> {
		Err(sp_runtime::DispatchError::Other("NotImplemented"))
	}

	fn get_min_pool_liquidity() -> Balance {
		Balance::from(0_u32)
	}

	fn get_min_trading_limit() -> Balance {
		Balance::from(0_u32)
	}

	// Only fn bellow are used by liq. mining pallet
	fn exists(assets: AssetPair) -> bool {
		AMM_POOLS.with(|v| v.borrow().contains_key(&asset_pair_to_map_key(assets)))
	}

	fn get_pair_id(assets: AssetPair) -> AccountId {
		AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(assets)).unwrap().0)
	}

	fn get_share_token(assets: AssetPair) -> AssetId {
		AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(assets)).unwrap().1)
	}
}

pub fn asset_pair_to_map_key(assets: AssetPair) -> String {
	format!("in:{}_out:{}", assets.asset_in, assets.asset_out)
}

parameter_types! {
	pub NativeCurrencyId: AssetId = 0;

	pub AccumulatePeriod: BlockNumber = 10;     // 10 blocks
	pub const MaxLocks: u32 = 1;
	pub const LMPalletId: PalletId = PalletId(*b"TEST_lm_");
	pub const MinPlannedYieldingPeriods: BlockNumber = 100;
	pub const MinTotalFarmRewards: Balance = 1_000_000;
}

impl Config for Test {
	type Event = Event;
	type CurrencyId = AssetId;
	type MultiCurrency = Tokens;
	type CreateOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
	type PalletId = LMPalletId;
	type MinPlannedYieldingPeriods = MinPlannedYieldingPeriods;
	type MinTotalFarmRewards = MinTotalFarmRewards;
	type BlockNumberProvider = MockBlockNumberProvider;
	type AMM = Amm;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		1u128
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = Nothing;
}

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(ALICE, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(ALICE, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(ALICE, BSX, INITIAL_BALANCE),
				(ACC_1M, BSX, 1_000_000),
				(BOB, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(BOB, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(BOB, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(BOB, KSM, INITIAL_BALANCE),
				(CHARLIE, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(CHARLIE, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(CHARLIE, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(DAVE, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(DAVE, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(DAVE, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(GC, BSX, INITIAL_BALANCE),
				(TREASURY, BSX, 1_000_000_000_000_000_000),
				(TREASURY, ACA, 1_000_000_000_000_000_000),
				(TREASURY, HDX, 1_000_000_000_000_000_000),
				(TREASURY, KSM, 1_000_000_000_000_000_000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		orml_tokens::GenesisConfig::<Test> {
			balances: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}

pub fn run_to_block(n: u64) {
	MockBlockNumberProvider::set(n);
	System::set_block_number(System::block_number() + 1);
}