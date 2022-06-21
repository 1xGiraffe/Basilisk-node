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

//TODO: put this to tests folder

#![cfg(test)]
use super::*;

use crate as liq_mining;
use crate::Config;
use frame_support::weights::RuntimeDbWeight;
use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild, Nothing},
	PalletId,
};
use frame_system as system;
use hydradx_traits_amm::AMM;
use orml_traits::parameter_type_with_key;
use primitives::nft::{ClassType, NftPermissions};
use primitives::ReserveIdentifier;
use primitives::{asset::AssetPair, Amount, AssetId, Balance};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, BlockNumberProvider, IdentityLookup},
};
use std::{cell::RefCell, collections::HashMap};

pub type AccountId = u128;
pub type PoolId = crate::PoolId;
pub type BlockNumber = u64;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DAVE: AccountId = 4;
pub const EVE: AccountId = 5;
pub const TREASURY: AccountId = 6;
pub const ACCOUNT_WITH_1M: AccountId = 7;
pub const GC: AccountId = 8;
pub const PALLET_SERVICE_ACCOUNT: AccountId = 29533360621462889584138678125;

pub const INITIAL_BALANCE: u128 = 1_000_000_000_000;

pub const BSX_ACA_SHARE_ID: AssetId = 100;
pub const BSX_KSM_SHARE_ID: AssetId = 101;
pub const BSX_DOT_SHARE_ID: AssetId = 102;
pub const BSX_ETH_SHARE_ID: AssetId = 103;
pub const BSX_HDX_SHARE_ID: AssetId = 104;
pub const BSX_TKN1_SHARE_ID: AssetId = 105;
pub const BSX_TKN2_SHARE_ID: AssetId = 106;
pub const KSM_DOT_SHARE_ID: AssetId = 107;
pub const ACA_KSM_SHARE_ID: AssetId = 108;

pub const BSX: AssetId = 1000;
pub const HDX: AssetId = 2000;
pub const ACA: AssetId = 3000;
pub const KSM: AssetId = 4000;
pub const DOT: AssetId = 5000;
pub const ETH: AssetId = 6000;
pub const TKN1: AssetId = 7_001;
pub const TKN2: AssetId = 7_002;

pub const BSX_ACA_AMM: AccountId = 11_000;
pub const BSX_KSM_AMM: AccountId = 11_001;
pub const BSX_DOT_AMM: AccountId = 11_002;
pub const BSX_ETH_AMM: AccountId = 11_003;
pub const BSX_HDX_AMM: AccountId = 11_004;
pub const BSX_TKN1_AMM: AccountId = 11_005;
pub const BSX_TKN2_AMM: AccountId = 11_006;
pub const DEFAULT_AMM: AccountId = 11_007;
pub const KSM_DOT_AMM: AccountId = 11_008;
pub const ACA_KSM_AMM: AccountId = 11_009;

pub const BSX_FARM: PoolId = 1;
pub const KSM_FARM: PoolId = 2;
pub const GC_FARM: PoolId = 3;
pub const ACA_FARM: PoolId = 4;

pub const INITIAL_READ_WEIGHT: u64 = 1;
pub const INITIAL_WRITE_WEIGHT: u64 = 1;

pub const LIQ_MINING_NFT_CLASS: primitives::ClassId = 1;

pub const BSX_TKN1_ASSET_PAIR: AssetPair = AssetPair {
	asset_in: BSX,
	asset_out: TKN1,
};

pub const BSX_TKN2_ASSET_PAIR: AssetPair = AssetPair {
	asset_in: BSX,
	asset_out: TKN2,
};

pub const BSX_KSM_ASSET_PAIR: AssetPair = AssetPair {
	asset_in: BSX,
	asset_out: KSM,
};

pub const BSX_DOT_ASSET_PAIR: AssetPair = AssetPair {
	asset_in: BSX,
	asset_out: DOT,
};

pub const BSX_ETH_ASSET_PAIR: AssetPair = AssetPair {
	asset_in: BSX,
	asset_out: ETH,
};

pub const ACA_KSM_ASSET_PAIR: AssetPair = AssetPair {
	asset_in: ACA,
	asset_out: KSM,
};

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
		NFT: pallet_nft::{Pallet, Call, Event<T>, Storage},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		WarehouseLM: warehouse_liquidity_mining::{Pallet, Storage},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 63;
	pub static MockBlockNumberProvider: u64 = 0;
	pub const DbWeight: RuntimeDbWeight = RuntimeDbWeight{
		read: INITIAL_READ_WEIGHT, write: INITIAL_WRITE_WEIGHT
	};
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
	type DbWeight = DbWeight;
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub struct Amm;

thread_local! {
	pub static NFT_CLASS: RefCell<(u128, u128, u128)>= RefCell::new((0,0,0));

	pub static DEPOSITS: RefCell<HashMap<u128, u128>> = RefCell::new(HashMap::default());

	pub static AMM_POOLS: RefCell<HashMap<String, (AccountId, AssetId, AssetPair)>> = RefCell::new(HashMap::new());

	//This is used to check if `on_accumulated_rpvs_update()` was called with correct values
	//`(global_farm_id, yield_farm_id, accumulated_rpvs, total_valued_shares)`
	pub static RPVS_UPDATED: RefCell<(GlobalFarmId, u32, Balance, Balance)> = RefCell::new((0,0,0,0));

	//This is used to check if `on_accumulated_rpz_update()` was called with correct values
	//`(global_farm_id, accumulated_rpz, total_shares_z)`
	pub static RPZ_UPDATED: RefCell<(GlobalFarmId, Balance, Balance)> = RefCell::new((0,0,0));
}

impl AMM<AccountId, AssetId, AssetPair, Balance> for Amm {
	fn get_max_out_ratio() -> u128 {
		0_u32.into()
	}

	fn get_fee(_pool_account_id: &AccountId) -> (u32, u32) {
		(0, 0)
	}

	fn get_max_in_ratio() -> u128 {
		0_u32.into()
	}

	fn get_pool_assets(pool_account_id: &AccountId) -> Option<Vec<AssetId>> {
		AMM_POOLS.with(|amm_pools| {
			for (_k, v) in amm_pools.borrow().iter() {
				if v.0 == *pool_account_id {
					return Some(vec![v.2.asset_in, v.2.asset_out]);
				}
			}

			None
		})
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
		hydradx_traits_amm::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
		frame_support::sp_runtime::DispatchError,
	> {
		Err(sp_runtime::DispatchError::Other("NotImplemented"))
	}

	fn execute_buy(
		_transfer: &hydradx_traits_amm::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
	) -> frame_support::dispatch::DispatchResult {
		Err(sp_runtime::DispatchError::Other("NotImplemented"))
	}

	fn execute_sell(
		_transfer: &hydradx_traits_amm::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
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
		hydradx_traits_amm::AMMTransfer<AccountId, AssetId, AssetPair, Balance>,
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

	// Fn bellow are used by liq. mining pallet
	fn exists(assets: AssetPair) -> bool {
		AMM_POOLS.with(|v| v.borrow().contains_key(&asset_pair_to_map_key(assets)))
	}

	fn get_pair_id(assets: AssetPair) -> AccountId {
		AMM_POOLS.with(|v| match v.borrow().get(&asset_pair_to_map_key(assets)) {
			Some(p) => p.0,
			None => DEFAULT_AMM,
		})
	}

	fn get_share_token(assets: AssetPair) -> AssetId {
		AMM_POOLS.with(|v| match v.borrow().get(&asset_pair_to_map_key(assets)) {
			Some(p) => p.1,
			None => BSX,
		})
	}
}

pub fn asset_pair_to_map_key(assets: AssetPair) -> String {
	format!("in:{}_out:{}", assets.asset_in, assets.asset_out)
}

parameter_types! {
	pub const WarehouseLMPalletId: PalletId = PalletId(*b"WhouseLm");
	pub const MinDeposit: Balance = 1;
}

impl warehouse_liquidity_mining::Config for Test {
	type CurrencyId = AssetId;
	type MultiCurrency = Tokens;
	type PalletId = WarehouseLMPalletId;
	type MinTotalFarmRewards = MinTotalFarmRewards;
	type MinPlannedYieldingPeriods = MinPlannedYieldingPeriods;
	type MinDeposit = MinDeposit;
	type BlockNumberProvider = MockBlockNumberProvider;
	type AmmPoolId = AccountId;
	type Handler = LiquidityMining;
	type MaxFarmEntriesPerDeposit = MaxEntriesPerDeposit;
}

parameter_types! {
	pub const MaxLocks: u32 = 1;
	pub const LMPalletId: PalletId = PalletId(*b"TEST_lm_");
	pub const MinPlannedYieldingPeriods: BlockNumber = 100;
	pub const MinTotalFarmRewards: Balance = 1_000_000;
	pub const MaxEntriesPerDeposit: u8 = 10;
	pub const NftClassId: primitives::ClassId = LIQ_MINING_NFT_CLASS;
	pub const ReserveClassIdUpTo: u128 = 2;
}

impl Config for Test {
	type Event = Event;
	type MultiCurrency = Tokens;
	type CreateOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
	type PalletId = LMPalletId;
	type MinPlannedYieldingPeriods = MinPlannedYieldingPeriods;
	type MinTotalFarmRewards = MinTotalFarmRewards;
	type BlockNumberProvider = MockBlockNumberProvider;
	type AMM = Amm;
	type NftClassId = NftClassId;
	type ReserveClassIdUpTo = ReserveClassIdUpTo;
	type NFTHandler = NftHandlerStub;
}

pub struct NftHandlerStub;

impl<AccountId: From<u128>> Inspect<AccountId> for NftHandlerStub {
	type InstanceId = u128;
	type ClassId = u128;

	fn owner(_class: &Self::ClassId, instance: &Self::InstanceId) -> Option<AccountId> {
		let mut owner: Option<AccountId> = None;

		DEPOSITS.with(|v| {
			if let Some(o) = v.borrow().get(instance) {
				owner = Some((*o).into());
			}
		});
		owner
	}
}

impl<AccountId: From<u128> + Into<u128> + Copy> Create<AccountId> for NftHandlerStub {
	fn create_class(_class: &Self::ClassId, _who: &AccountId, _admin: &AccountId) -> DispatchResult {
		NFT_CLASS.with(|v| {
			//let mut class = v.borrow_mut();
			v.replace((*_class, (*_who).into(), (*_admin).into()));
		});
		Ok(())
	}
}

impl<AccountId: From<u128> + Into<u128> + Copy> Mutate<AccountId> for NftHandlerStub {
	fn mint_into(_class: &Self::ClassId, _instance: &Self::InstanceId, _who: &AccountId) -> DispatchResult {
		DEPOSITS.with(|v| {
			let mut m = v.borrow_mut();
			m.insert(*_instance, (*_who).into());
		});
		Ok(())
	}

	fn burn_from(_class: &Self::ClassId, instance: &Self::InstanceId) -> DispatchResult {
		DEPOSITS.with(|v| {
			let mut m = v.borrow_mut();
			m.remove(instance);
		});
		Ok(())
	}
}

impl pallet_nft::Config for Test {
	type Currency = Balances;
	type Event = Event;
	type WeightInfo = pallet_nft::weights::BasiliskWeight<Test>;
	type NftClassId = primitives::ClassId;
	type NftInstanceId = primitives::InstanceId;
	type ProtocolOrigin = frame_system::EnsureRoot<AccountId>;
	type ClassType = ClassType;
	type Permissions = NftPermissions;
	type ReserveClassIdUpTo = ReserveClassIdUpTo;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Test>;
	type MaxLocks = ();
	type WeightInfo = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = ReserveIdentifier;
}

parameter_types! {
	pub const ClassDeposit: Balance = 1_000_000_000_000_000; // 1 UNIT deposit to create asset class
	pub const InstanceDeposit: Balance = 10_000_000_000_000; // 1/100 UNIT deposit to create asset instance
	pub const KeyLimit: u32 = 32;	// Max 32 bytes per key
	pub const ValueLimit: u32 = 64;	// Max 64 bytes per value
	pub const UniquesMetadataDepositBase: Balance = 100_000_000_000_000;
	pub const AttributeDepositBase: Balance = 10_000_000_000_000;
	pub const DepositPerByte: Balance = 1_000_000_000_000;
	pub const UniquesStringLimit: u32 = 32;
}

impl pallet_uniques::Config for Test {
	type Event = Event;
	type ClassId = primitives::ClassId;
	type InstanceId = primitives::InstanceId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type ClassDeposit = ClassDeposit;
	type InstanceDeposit = InstanceDeposit;
	type MetadataDepositBase = UniquesMetadataDepositBase;
	type AttributeDepositBase = AttributeDepositBase;
	type DepositPerByte = DepositPerByte;
	type StringLimit = UniquesStringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
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
		DEPOSITS.with(|v| {
			v.borrow_mut().clear();
		}); //TODO: Dani - do we need it here?
		Self {
			endowed_accounts: vec![
				(ALICE, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(ALICE, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(ALICE, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(ALICE, BSX_TKN1_SHARE_ID, 3_000_000),
				(ALICE, BSX_TKN2_SHARE_ID, 3_000_000),
				(ALICE, ACA_KSM_SHARE_ID, 3_000_000),
				(ALICE, BSX, INITIAL_BALANCE),
				(ACCOUNT_WITH_1M, BSX, 1_000_000),
				(BOB, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(BOB, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(BOB, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(BOB, BSX_TKN1_SHARE_ID, 2_000_000),
				(BOB, BSX_TKN2_SHARE_ID, 2_000_000),
				(BOB, ACA_KSM_SHARE_ID, 2_000_000),
				(BOB, BSX, INITIAL_BALANCE),
				(BOB, KSM, INITIAL_BALANCE),
				(CHARLIE, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(CHARLIE, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(CHARLIE, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(CHARLIE, BSX_TKN1_SHARE_ID, 5_000_000),
				(CHARLIE, BSX_TKN2_SHARE_ID, 5_000_000),
				(CHARLIE, BSX, INITIAL_BALANCE),
				(CHARLIE, KSM, INITIAL_BALANCE),
				(CHARLIE, ACA, INITIAL_BALANCE),
				(DAVE, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(DAVE, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(DAVE, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(DAVE, BSX_TKN1_SHARE_ID, 10_000_000),
				(DAVE, BSX_TKN2_SHARE_ID, 10_000_000),
				(DAVE, BSX, INITIAL_BALANCE),
				(DAVE, KSM, INITIAL_BALANCE),
				(DAVE, ACA, INITIAL_BALANCE),
				(GC, BSX, INITIAL_BALANCE),
				(TREASURY, BSX, 1_000_000_000_000_000_000),
				(TREASURY, ACA, 1_000_000_000_000_000_000),
				(TREASURY, HDX, 1_000_000_000_000_000_000),
				(TREASURY, KSM, 1_000_000_000_000_000_000),
				(EVE, BSX_ACA_SHARE_ID, INITIAL_BALANCE),
				(EVE, BSX_DOT_SHARE_ID, INITIAL_BALANCE),
				(EVE, BSX_KSM_SHARE_ID, INITIAL_BALANCE),
				(EVE, BSX_TKN1_SHARE_ID, 10_000_000),
				(EVE, BSX_TKN2_SHARE_ID, 10_000_000),
				(EVE, BSX, INITIAL_BALANCE),
				(EVE, KSM, INITIAL_BALANCE),
				(EVE, ACA, INITIAL_BALANCE),
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

pub fn set_block_number(n: u64) {
	MockBlockNumberProvider::set(n);
	System::set_block_number(n);
}
