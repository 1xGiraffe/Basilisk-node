// This file is part of HydraDX.

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
pub use crate::mock::{
	Currency, EndowedAmount, Event as TestEvent, Exchange, ExtBuilder, Origin, System, Test, ALICE, BOB, CHARLIE, DAVE,
	DOT, ETH, FERDIE, GEORGE, HDX, XYK as XYKPallet,
};
use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_runtime::FixedPointNumber;
use frame_support::traits::Get;
use frame_support::traits::OnFinalize;
use frame_support::{assert_noop, assert_ok};
use hydradx_traits::Resolver;
use primitives::Price;
use sp_runtime::{DispatchError, ModuleError};

use pallet_xyk as xyk;

fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext = ExtBuilder::default().build();
	ext.execute_with(|| System::set_block_number(1));
	ext
}

fn expect_event<E: Into<TestEvent>>(e: E) {
	frame_system::Pallet::<Test>::assert_has_event(e.into());
}

fn expect_events(e: Vec<TestEvent>) {
	println!("left: {:?}\n", frame_system::Pallet::<Test>::events());
	println!("right: {:?}", e);
	e.into_iter().for_each(frame_system::Pallet::<Test>::assert_has_event);
}

fn generate_intention_id(account: &<Test as system::Config>::AccountId, c: u32) -> crate::IntentionId<Test> {
	let b = <system::Pallet<Test>>::current_block_number();
	(c, &account, b, DOT, ETH).using_encoded(<Test as system::Config>::Hashing::hash)
}

/// HELPER FOR INITIALIZING POOLS
fn initialize_pool(asset_a: u32, asset_b: u32, user: u64, amount: u128, price: Price) {
	assert_ok!(XYKPallet::create_pool(
		Origin::signed(user),
		asset_a,
		asset_b,
		amount,
		price
	));

	let shares = if asset_a <= asset_b {
		amount
	} else {
		price.checked_mul_int(amount).unwrap()
	};

	let pair_account = XYKPallet::get_pair_id(AssetPair {
		asset_in: asset_a,
		asset_out: asset_b,
	});
	let share_token = XYKPallet::share_token(pair_account);

	expect_event(xyk::Event::PoolCreated {
        who: user,
        asset_a,
        asset_b,
        initial_shares_amount: shares,
        share_token,
        pool_account_id: pair_account
    });

	let amount_b = price.saturating_mul_int(amount);

	// Check users state
	assert_eq!(Currency::free_balance(asset_a, &user), EndowedAmount::get() - amount);
	assert_eq!(Currency::free_balance(asset_b, &user), EndowedAmount::get() - amount_b);

	// Check initial state of the pool
	assert_eq!(Currency::free_balance(asset_a, &pair_account), amount);
	assert_eq!(Currency::free_balance(asset_b, &pair_account), amount_b);

	// Check pool shares
	assert_eq!(Currency::free_balance(share_token, &user), shares);

	// Advance blockchain so that we kill old events
	System::initialize(&1, &[0u8; 32].into(), &Default::default());
}

#[test]
fn no_intentions_should_work() {
	new_test_ext().execute_with(|| {
		let user = ALICE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user, pool_amount, initial_price);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user), 99_900_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user), 99_800_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200_000_000_000_000);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);
	});
}

#[test]
fn sell_test_pool_finalization_states() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000_000_000_000,
			20_000_000_000,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			1_000_000_000_000,
			4_000_000_000_000,
			false,
		));

		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 2);

		// Balance should not change yet
		assert_eq!(Currency::free_balance(asset_a, &user_2), EndowedAmount::get());
		assert_eq!(Currency::free_balance(asset_b, &user_2), EndowedAmount::get());

		assert_eq!(Currency::free_balance(asset_a, &user_3), EndowedAmount::get());
		assert_eq!(Currency::free_balance(asset_b, &user_3), EndowedAmount::get());

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100_000_000_000_000);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_2,
				amount: 1000000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_3,
				amount: 2004000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1000000000000,
				amount_b: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 8000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_2,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 1000000000000,
				sale_price: 1976316673268,
				fee_asset: asset_b,
				fee_amount: 3960554454,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 1000000000000,
				amount_sold_or_bought: 1980277227722,
				pool_account_id: pair_account,
			}
			.into(),
		]);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_998_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_003_972_316_673_268);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_001_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_997_996_000_000_000);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 101_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 198_031_683_326_732);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);
	});
}

#[test]
fn sell_test_standard() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000_000_000_000,
			300_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			1_000_000_000_000,
			4_000_000_000_000,
			false,
		));

		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 2);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances -> SEEMS LEGIT
		let user_2_balance_a = Currency::free_balance(asset_a, &user_2);
		let user_2_balance_b = Currency::free_balance(asset_b, &user_2);
		assert_eq!(user_2_balance_a, 99_998_000_000_000_000);
		assert_eq!(user_2_balance_b, 100_003_972_316_673_268);

		let user_3_balance_a = Currency::free_balance(asset_a, &user_3);
		let user_3_balance_b = Currency::free_balance(asset_b, &user_3);
		assert_eq!(user_3_balance_a, 100_001000000000000);
		assert_eq!(user_3_balance_b, 99_997996000000000);

		// Check final pool balances -> SEEMS LEGIT
		let pool_balance_a = Currency::free_balance(asset_a, &pair_account);
		let pool_balance_b = Currency::free_balance(asset_b, &pair_account);
		assert_eq!(pool_balance_a, 101_000_000_000_000);
		assert_eq!(pool_balance_b, 198_031_683_326_732);

		assert_eq!(
			user_2_balance_a + user_3_balance_a + pool_balance_a,
			200_100_000_000_000_000
		);
		assert_eq!(
			user_2_balance_b + user_3_balance_b + pool_balance_b,
			200_200_000_000_000_000
		);

		// No tokens should be created or lost
		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_2,
				amount: 1000000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_3,
				amount: 2004000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1000000000000,
				amount_b: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 8000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_2,
				asset_in: 3000,
				asset_out: 2000,
				amount: 1000000000000,
				sale_price: 1976316673268,
				fee_asset: 2000,
				fee_amount: 3960554454,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 1000000000000,
				amount_sold_or_bought: 1980277227722,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn sell_test_inverse_standard() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			4_000_000_000_000,
			1_000_000_000_000,
			false,
		));

		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 2);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances  -> SEEMS LEGIT
		let user_2_balance_a = Currency::free_balance(asset_a, &user_2);
		let user_2_balance_b = Currency::free_balance(asset_b, &user_2);
		assert_eq!(user_2_balance_a, 99_999_000_000_000_000);
		assert_eq!(user_2_balance_b, 100_001_996_000_000_000);

		let user_3_balance_a = Currency::free_balance(asset_a, &user_3);
		let user_3_balance_b = Currency::free_balance(asset_b, &user_3);
		assert_eq!(user_3_balance_a, 100_001_986_118_811_882);
		assert_eq!(user_3_balance_b, 99_996_000_000_000_000);

		// Check final pool balances  -> SEEMS LEGIT
		let pool_balance_a = Currency::free_balance(asset_a, &pair_account);
		let pool_balance_b = Currency::free_balance(asset_b, &pair_account);
		assert_eq!(pool_balance_a, 99_013_881_188_118);
		assert_eq!(pool_balance_b, 202_004_000_000_000);

		assert_eq!(
			user_2_balance_a + user_3_balance_a + pool_balance_a,
			200_100_000_000_000_000
		);
		assert_eq!(
			user_2_balance_b + user_3_balance_b + pool_balance_b,
			200_200_000_000_000_000
		);
		// No tokens should be created or lost

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 4_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_2,
				amount: 1000000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_3,
				amount: 2000000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: 3,
				asset_in: 2000,
				asset_out: 3000,
				amount: 2000000000000,
				sale_price: 988118811882,
				fee_asset: 3000,
				fee_amount: 1980198018,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_3,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
				amount: 2000000000000,
				amount_sold_or_bought: 990099009900,
				pool_account_id: pair_account,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1000000000000,
				amount_b: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 2000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 4000000000,
			}
			.into(),
		]);
	});
}

#[test]
fn sell_test_exact_match() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000_000_000,
			1_500_000_000_000,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			2_000_000_000_000,
			200_000_000_000,
			false,
		));

		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 2);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_001_996_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_000_998_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_998_000_000_000_000);

		// Check final pool balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100002000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200004000000000);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_2,
				amount: 1000000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_3,
				amount: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1000000000000,
				amount_b: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 2000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 4000000000,
			}
			.into(),
		]);
	});
}

#[test]
fn sell_test_single_eth_sells() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			2_000_000_000_000,
			200_000_000_000,
			false,
		));

		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 2);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100001899942737485);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_998_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100003913725490196);

		// Check final pool balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 103_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 194_186_331_772_319);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_3,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 2000000000000,
				sale_price: 3913725490196,
				fee_asset: asset_b,
				fee_amount: 7843137254,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_3,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
				amount: 2000000000000,
				amount_sold_or_bought: 3921568627450,
				pool_account_id: pair_account,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_2,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 1000000000000,
				sale_price: 1899942737485,
				fee_asset: asset_b,
				fee_amount: 3807500474,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 1000000000000,
				amount_sold_or_bought: 1903750237959,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn sell_test_single_dot_sells() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			2_000_000_000_000,
			200_000_000_000,
			false,
		));

		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 2);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &user_2), 100000486767770570);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_999_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100000988118811882);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_998_000_000_000_000);

		// Check final pool balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 98525113417548);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 203_000_000_000_000);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);
		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_3,
				asset_in: asset_b,
				asset_out: asset_a,
				amount: 2000000000000,
				sale_price: 988118811882,
				fee_asset: asset_a,
				fee_amount: 1980198018,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_3,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
				amount: 2000000000000,
				amount_sold_or_bought: 990099009900,
				pool_account_id: pair_account,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_2,
				asset_in: asset_b,
				asset_out: asset_a,
				amount: 1000000000000,
				sale_price: 486767770570,
				fee_asset: asset_a,
				fee_amount: 975486514,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 1000000000000,
				amount_sold_or_bought: 487743257084,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn sell_trade_limits_respected_for_matched_intention() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));

		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			1_000_000_000_000,
			100_000_000_000_000_000, // Limit set to absurd amount which can't go through
			false,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionResolveErrorEvent {
				who: user_3,
				asset_ids: AssetPair {
					asset_in: asset_b,
					asset_out: asset_a,
				},
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
				error_detail: DispatchError::Module(ModuleError {
					index: 1,
					error: 3,
					message: None,
				}),
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_2,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 1000000000000,
				sale_price: 1976237623763,
				fee_asset: asset_b,
				fee_amount: 3960396038,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 1000000000000,
				amount_sold_or_bought: 1980198019801,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn buy_trade_limits_respected_for_matched_intention() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			1_000_000_000_000,
			10_000_000_000_000_000,
			false,
		));

		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			100_000_000_000,
			1,
			false,
		));
		let user_3_buy_intention_id = generate_intention_id(&user_3, 1);

		let user_2_buy_intention_id = generate_intention_id(&user_2, 0);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_buy_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 100_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_buy_intention_id,
			}
			.into(),
			Event::IntentionResolveErrorEvent {
				who: user_3,
				asset_ids: AssetPair {
					asset_in: asset_b,
					asset_out: asset_a,
				},
				intention_type: IntentionType::BUY,
				intention_id: user_3_buy_intention_id,
				error_detail: DispatchError::Module(ModuleError {
					index: 1,
					error: 2,
					message: None,
				}),
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_2,
				asset_out: asset_b,
				asset_in: asset_a,
				amount: 1000000000000,
				buy_price: 502512562815,
				fee_asset: asset_a,
				fee_amount: 1005025124,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_buy_intention_id,
				amount: 1000000000000,
				amount_sold_or_bought: 503517587939,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn sell_test_single_multiple_sells() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let user_4 = DAVE;
		let user_5 = FERDIE;
		let user_6 = GEORGE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);
		assert_ok!(Exchange::sell(
			Origin::signed(user_4),
			asset_a,
			asset_b,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));
		let user_4_sell_intention_id = generate_intention_id(&user_4, 2);
		assert_ok!(Exchange::sell(
			Origin::signed(user_5),
			asset_b,
			asset_a,
			1_000_000_000_000,
			100_000_000_000,
			false,
		));
		let user_5_sell_intention_id = generate_intention_id(&user_5, 3);
		assert_ok!(Exchange::sell(
			Origin::signed(user_6),
			asset_b,
			asset_a,
			2_000_000_000_000,
			200_000_000_000,
			false,
		));
		let user_6_sell_intention_id = generate_intention_id(&user_6, 4);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 5);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_001996000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_000499000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_999000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_4), 99_999000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_4), 100001996000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_5), 100000499000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_5), 99_999000000000000);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100004000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200008000000000);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_4,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_5,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_5_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_6,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_6_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 2,
				amount: 1000000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 6,
				amount: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_6,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_6_sell_intention_id,
				amount_a: 1000000000000,
				amount_b: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 2000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_6,
				intention_id: user_6_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 4000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 4,
				amount: 500000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 3,
				amount: 1000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_4,
				account_id_b: user_3,
				intention_id_a: user_4_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 500000000000,
				amount_b: 1000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 1000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 2000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 4,
				amount: 500000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 5,
				amount: 1000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_4,
				account_id_b: user_5,
				intention_id_a: user_4_sell_intention_id,
				intention_id_b: user_5_sell_intention_id,
				amount_a: 500000000000,
				amount_b: 1000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 1000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_5,
				intention_id: user_5_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 2000000000,
			}
			.into(),
		]);
	});
}

#[test]
fn sell_test_group_sells() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let user_4 = DAVE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			5_000_000_000_000,
			200_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			3_000_000_000_000,
			200_000_000_000,
			false,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);
		assert_ok!(Exchange::sell(
			Origin::signed(user_4),
			asset_a,
			asset_b,
			10_000_000_000_000,
			200_000_000_000,
			false,
		));
		let user_4_sell_intention_id = generate_intention_id(&user_4, 2);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 3);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_002495000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_995000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_001497000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_997000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_4), 99_990000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_4), 100019282164364955);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 106008000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 188717835635045);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 5_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 3_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_4,
				asset_a,
				asset_b,
				amount: 10_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 4,
				amount: 2500000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 5000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_4,
				account_id_b: user_2,
				intention_id_a: user_4_sell_intention_id,
				intention_id_b: user_2_sell_intention_id,
				amount_a: 2500000000000,
				amount_b: 5000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 5000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 10000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 4,
				amount: 1500000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 3,
				amount: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_4,
				account_id_b: user_3,
				intention_id_a: user_4_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1500000000000,
				amount_b: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 3000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 6000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_4,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 6000000000000,
				sale_price: 11298164364955,
				fee_asset: asset_b,
				fee_amount: 22641611952,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_4,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
				amount: 6000000000000,
				amount_sold_or_bought: 11320805976907,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn trades_without_pool_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Exchange::sell(Origin::signed(ALICE), HDX, ETH, 1000, 200, false),
			Error::<Test>::TokenPoolNotFound
		);

		assert_noop!(
			Exchange::buy(Origin::signed(ALICE), HDX, ETH, 1000, 200, false),
			Error::<Test>::TokenPoolNotFound
		);
	});
}

#[test]
fn trade_min_limit() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Exchange::sell(Origin::signed(ALICE), HDX, ETH, 10, 200, false),
			Error::<Test>::MinimumTradeLimitNotReached
		);

		assert_noop!(
			Exchange::buy(Origin::signed(ALICE), HDX, ETH, 10, 200, false),
			Error::<Test>::MinimumTradeLimitNotReached
		);
	});
}

#[test]
fn sell_more_than_owner_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(XYKPallet::create_pool(
			Origin::signed(ALICE),
			HDX,
			ETH,
			200_000,
			Price::from(2)
		));

		// With SELL
		assert_noop!(
			Exchange::sell(Origin::signed(ALICE), HDX, ETH, 10 * EndowedAmount::get(), 1, false),
			Error::<Test>::InsufficientAssetBalance
		);

		// With BUY
		assert_noop!(
			Exchange::buy(Origin::signed(ALICE), ETH, HDX, 10 * EndowedAmount::get(), 1, false),
			Error::<Test>::InsufficientAssetBalance
		);
	});
}

#[test]
fn sell_test_mixed_buy_sells() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let user_4 = DAVE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			5_000_000_000_000,
			20_000_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			3_000_000_000_000,
			1_400_000_000_000,
			false,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);
		assert_ok!(Exchange::sell(
			Origin::signed(user_4),
			asset_a,
			asset_b,
			10_000_000_000_000,
			2_000_000_000_000,
			false,
		));
		let user_4_sell_intention_id = generate_intention_id(&user_4, 2);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 3);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99996969377448952);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_005000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_001497000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_997000000000000);
		assert_eq!(Currency::free_balance(asset_a, &user_4), 99_990000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_4), 100018630903108671);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 111533622551048);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 179369096891329);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 5_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 3_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_4,
				asset_a,
				asset_b,
				amount: 10_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 4,
				amount: 1500000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 3,
				amount: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_4,
				account_id_b: user_3,
				intention_id_a: user_4_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1500000000000,
				amount_b: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 3000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 6000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_4,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 8500000000000,
				sale_price: 15636903108671,
				fee_asset: asset_b,
				fee_amount: 31336479174,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_4,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
				amount: 8500000000000,
				amount_sold_or_bought: 15668239587845,
				pool_account_id: pair_account,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_2,
				asset_out: asset_b,
				asset_in: asset_a,
				amount: 5000000000000,
				buy_price: 3024573404240,
				fee_asset: asset_a,
				fee_amount: 6049146808,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 5000000000000,
				amount_sold_or_bought: 3030622551048,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn discount_tests_no_discount() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let user_4 = DAVE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			5_000_000_000_000,
			20_000_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			3_000_000_000_000,
			1_400_000_000_000,
			false,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);
		assert_ok!(Exchange::sell(
			Origin::signed(user_4),
			asset_a,
			asset_b,
			10_000_000_000_000,
			2_000_000_000_000,
			false,
		));
		let user_4_sell_intention_id = generate_intention_id(&user_4, 2);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 3);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99996969377448952);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_005000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_001497000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_997000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_4), 99_990000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_4), 100018630903108671);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 111533622551048);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 179369096891329);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 5_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 3_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_4,
				asset_a,
				asset_b,
				amount: 10_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 4,
				amount: 1500000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 3,
				amount: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_4,
				account_id_b: user_3,
				intention_id_a: user_4_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1500000000000,
				amount_b: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 3000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 6000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_4,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 8500000000000,
				sale_price: 15636903108671,
				fee_asset: asset_b,
				fee_amount: 31336479174,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_4,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
				amount: 8500000000000,
				amount_sold_or_bought: 15668239587845,
				pool_account_id: pair_account,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_2,
				asset_out: asset_b,
				asset_in: asset_a,
				amount: 5000000000000,
				buy_price: 3024573404240,
				fee_asset: asset_a,
				fee_amount: 6049146808,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 5000000000000,
				amount_sold_or_bought: 3030622551048,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn discount_tests_with_discount() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let user_4 = DAVE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);
		initialize_pool(asset_a, HDX, user_2, pool_amount, initial_price);
		initialize_pool(asset_b, HDX, user_3, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			5_000_000_000_000,
			20_000_000_000_000,
			true,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			3_000_000_000_000,
			1_400_000_000_000,
			true,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);
		assert_ok!(Exchange::sell(
			Origin::signed(user_4),
			asset_a,
			asset_b,
			10_000_000_000_000,
			2_000_000_000_000,
			true,
		));
		let user_4_sell_intention_id = generate_intention_id(&user_4, 2);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 3);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99896972965651840);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_005000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_001497000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_897000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_4), 99_990000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_4), 100018651271820139);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 111530034348160);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 179348728179861);

		assert_eq!(Currency::free_balance(HDX, &user_4), 99999978064464588);
		assert_eq!(Currency::free_balance(HDX, &user_2), 99799995765116340);
		assert_eq!(Currency::free_balance(HDX, &user_3), 99_800000000000000);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 5_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 3_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_4,
				asset_a,
				asset_b,
				amount: 10_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 4,
				amount: 1500000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 3,
				amount: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_4,
				account_id_b: user_3,
				intention_id_a: user_4_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1500000000000,
				amount_b: 3000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 3000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 6000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: user_4,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 8500000000000,
				sale_price: 15657271820139,
				fee_asset: asset_b,
				fee_amount: 10967767706,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_4,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
				amount: 8500000000000,
				amount_sold_or_bought: 15668239587845,
				pool_account_id: pair_account,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_2,
				asset_out: asset_b,
				asset_in: asset_a,
				amount: 5000000000000,
				buy_price: 3024916906330,
				fee_asset: asset_a,
				fee_amount: 2117441830,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 5000000000000,
				amount_sold_or_bought: 3027034348160,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn buy_test_exact_match() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000_000_000,
			4_000_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			2_000_000_000_000,
			4_000_000_000_000,
			false,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 2);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_001000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_997996000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_998998000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_002000000000000);

		// Check final pool balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100002000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200004000000000);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 1002000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 2004000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_3,
				account_id_b: user_2,
				intention_id_a: user_3_sell_intention_id,
				intention_id_b: user_2_sell_intention_id,
				amount_a: 1000000000000,
				amount_b: 2000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 2000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 4000000000,
			}
			.into(),
		]);
	});
}

#[test]
fn buy_test_group_buys() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let user_4 = DAVE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			5_000_000_000_000,
			20_000_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			3_000_000_000_000,
			20_000_000_000_000,
			false,
		));
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);
		assert_ok!(Exchange::buy(
			Origin::signed(user_4),
			asset_a,
			asset_b,
			10_000_000_000_000,
			22_000_000_000_000,
			false,
		));
		let user_4_sell_intention_id = generate_intention_id(&user_4, 2);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 3);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_997495000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_005000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_998696090255838);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_003000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_4), 100_010000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_4), 99_978741351351351);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 93808909744162);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 213258648648649);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 5_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 3_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_4,
				asset_a,
				asset_b,
				amount: 10_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_4_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 2,
				amount: 2505000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 4,
				amount: 5010000000000,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_4,
				asset_out: asset_a,
				asset_in: asset_b,
				amount: 7500000000000,
				buy_price: 16216216216217,
				fee_asset: asset_b,
				fee_amount: 32432432432,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_4,
				intention_type: IntentionType::BUY,
				intention_id: user_4_sell_intention_id,
				amount: 7500000000000,
				amount_sold_or_bought: 16248648648649,
				pool_account_id: pair_account,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_4,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_4_sell_intention_id,
				amount_a: 2500000000000,
				amount_b: 5000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 5000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_4,
				intention_id: user_4_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 10000000000,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_3,
				asset_out: asset_b,
				asset_in: asset_a,
				amount: 3000000000000,
				buy_price: 1301307129904,
				fee_asset: asset_a,
				fee_amount: 2602614258,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_3,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
				amount: 3000000000000,
				amount_sold_or_bought: 1303909744162,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn discount_tests_with_error() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let user_4 = DAVE;
		let asset_a = ETH;
		let asset_b = DOT;

		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_b,
			asset_a,
			5_000_000_000_000,
			20_000_000_000_000,
			true,
		));
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			3_000_000_000_000,
			20_000_000_000_000,
			true,
		));
		assert_ok!(Exchange::sell(
			Origin::signed(user_4),
			asset_a,
			asset_b,
			10_000_000_000_000,
			20_000_000_000_000,
			true,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);
		let user_4_sell_intention_id = generate_intention_id(&user_4, 2);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 3);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances
		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_000000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_000000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_4), 100_000000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_4), 100_000000000000000);

		// Check final pool balances
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000000000);

		assert_eq!(Currency::free_balance(HDX, &user_4), EndowedAmount::get());
		assert_eq!(Currency::free_balance(HDX, &user_2), EndowedAmount::get());
		assert_eq!(Currency::free_balance(HDX, &user_3), EndowedAmount::get());

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 5_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 3_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_4,
				asset_a,
				asset_b,
				amount: 10_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
			}
			.into(),
			Event::IntentionResolveErrorEvent {
				who: user_4,
				asset_ids: AssetPair {
					asset_in: asset_a,
					asset_out: asset_b,
				},
				intention_type: IntentionType::SELL,
				intention_id: user_4_sell_intention_id,
				error_detail: DispatchError::Module(ModuleError {
					index: 2,
					error: 20,
					message: None,
				}),
			}
			.into(),
			Event::IntentionResolveErrorEvent {
				who: user_2,
				asset_ids: AssetPair {
					asset_in: asset_a,
					asset_out: asset_b,
				},
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				error_detail: DispatchError::Module(ModuleError {
					index: 2,
					error: 20,
					message: None,
				}),
			}
			.into(),
			Event::IntentionResolveErrorEvent {
				who: user_3,
				asset_ids: AssetPair {
					asset_in: asset_b,
					asset_out: asset_a,
				},
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
				error_detail: DispatchError::Module(ModuleError {
					index: 2,
					error: 20,
					message: None,
				}),
			}
			.into(),
		]);
	});
}

#[test]
fn simple_sell_sell() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000,
			400,
			false,
		));
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			1_000,
			400,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999999999998000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000000000003993);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_000000000000500);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_999999999999000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100001500);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 199997007);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 2,
				amount: 500,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 3,
				amount: 1000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 500,
				amount_b: 1000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 0,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 2,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: 2,
				asset_in: 3000,
				asset_out: 2000,
				amount: 1500,
				sale_price: 2995,
				fee_asset: 2000,
				fee_amount: 4,
				pool_account_id: pair_account
			}.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 1500,
				amount_sold_or_bought: 2999,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn simple_buy_buy() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000,
			5000,
			false,
		));
		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			1_000,
			5000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000000000002000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_999999999995991);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_999999999999500);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_000000000001000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 99998500);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200003009);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 500,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 1002,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: 2,
				asset_out: 3000,
				asset_in: 2000,
				amount: 1500,
				buy_price: 3001,
				fee_asset: 2000,
				fee_amount: 6,
				pool_account_id: pair_account
			}.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 1500,
				amount_sold_or_bought: 3007,
				pool_account_id: pair_account,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_3,
				account_id_b: user_2,
				intention_id_a: user_3_sell_intention_id,
				intention_id_b: user_2_sell_intention_id,
				amount_a: 500,
				amount_b: 1000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 0,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 2,
			}
			.into(),
		]);
	});
}

#[test]
fn simple_sell_buy() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000,
			400,
			false,
		));
		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			1_000,
			2_000,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999999999998000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000000000003993);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_000000000001000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_999999999997996);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100001000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 199998011);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 1_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 2,
				amount: 1000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 3,
				amount: 2004,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1000,
				amount_b: 2000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 8,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: 2,
				asset_in: 3000,
				asset_out: 2000,
				amount: 1000,
				sale_price: 1997,
				fee_asset: 2000,
				fee_amount: 2,
				pool_account_id: pair_account
			}.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 1000,
				amount_sold_or_bought: 1999,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn simple_buy_sell() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000,
			5000,
			false,
		));
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			1_000,
			1500,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000000000002000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_999999999995991);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_999999999999000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_000000000001996);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 99999000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200002013);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 1_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 1000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 2004,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_2,
				asset_out: 3000,
				asset_in: 2000,
				amount: 1000,
				buy_price: 2001,
				fee_asset: 2000,
				fee_amount: 4,
				pool_account_id: pair_account
			}.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 1000,
				amount_sold_or_bought: 2005,
				pool_account_id: pair_account,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_3,
				account_id_b: user_2,
				intention_id_a: user_3_sell_intention_id,
				intention_id_b: user_2_sell_intention_id,
				amount_a: 1000,
				amount_b: 2000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 8,
			}
			.into(),
		]);
	});
}

#[test]
fn single_sell_intention_test() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000_000_000_000,
			400_000_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 1);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_998_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100003913725490196);

		// Check final pool balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 102000000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 196086274509804);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: 2,
				asset_in: 3000,
				asset_out: 2000,
				amount: 2000000000000,
				sale_price: 3913725490196,
				fee_asset: 2000,
				fee_amount: 7843137254,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				amount: 2000000000000,
				amount_sold_or_bought: 3921568627450,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn single_buy_intention_test() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000_000_000_000,
			15_000_000_000_000,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 1);

		// Finalize block
		<Exchange as OnFinalize<u64>>::on_finalize(9);

		// Check final account balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_002000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_995910204081632);

		// Check final pool balances -> SEEMS LEGIT
		assert_eq!(Currency::free_balance(asset_a, &pair_account), 98000000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 204089795918368);

		assert_eq!(Exchange::get_intentions_count((asset_b, asset_a)), 0);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000_000_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: 2,
				asset_out: 3000,
				asset_in: 2000,
				amount: 2000000000000,
				buy_price: 4081632653062,
				fee_asset: 2000,
				fee_amount: 8163265306,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 2000000000000,
				amount_sold_or_bought: 4089795918368,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn simple_sell_sell_with_error_should_not_pass() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			2_000,
			5_000,
			false,
		));

		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);

		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			1_000,
			5_000,
			false,
		));

		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000000000000000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 100_000000000000000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_000000000000000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 2_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 1_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			Event::IntentionResolveErrorEvent {
				who: user_2,
				asset_ids: AssetPair {
					asset_in: asset_a,
					asset_out: asset_b,
				},
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
				error_detail: DispatchError::Module(ModuleError {
					index: 2,
					error: 9,
					message: None,
				}),
			}
			.into(),
			Event::IntentionResolveErrorEvent {
				who: user_3,
				asset_ids: AssetPair {
					asset_in: asset_b,
					asset_out: asset_a,
				},
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
				error_detail: DispatchError::Module(ModuleError {
					index: 2,
					error: 9,
					message: None,
				}),
			}
			.into(),
		]);
	});
}

#[test]
fn matching_limits_buy_buy_should_work() {
	new_test_ext().execute_with(|| {
		let one: Balance = 1_000_000_000_000;
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1000 * one;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000 * one);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000 * one);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			100_000_000_000_000,
			223333333333334,
			false,
		));

		let b = <system::Pallet<Test>>::current_block_number();
		let user_2_sell_intention_id = (0, &user_2, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			220 * one,
			124213483146068,
			false,
		));

		let user_3_sell_intention_id = (1, &user_3, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_100 * one);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_799_600_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1010321212121213);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 1980400000000000);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 100 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 220 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 200400000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 100200000000000,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: 3,
				asset_out: asset_b,
				asset_in: asset_a,
				amount: 20_000_000_000_000,
				buy_price: 10101010101011,
				fee_asset: asset_a,
				fee_amount: 20202020202,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_3,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
				amount: 20_000_000_000_000,
				amount_sold_or_bought: 10121212121213,
				pool_account_id: pair_account,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 200000000000000,
				amount_b: 100000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 400000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 200000000000,
			}
			.into(),
		]);
	});
}

#[test]
fn matching_limits_sell_buy_should_work() {
	new_test_ext().execute_with(|| {
		let one: Balance = 1_000_000_000_000;
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1000 * one;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000 * one);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000 * one);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			30_000_000_000_000,
			62164948453608,
			false,
		));

		let b = <system::Pallet<Test>>::current_block_number();
		let user_2_sell_intention_id = (0, &user_2, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			50 * one,
			94761904761906,
			false,
		));

		let user_3_sell_intention_id = (1, &user_3, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_030 * one);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_939_880_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1020000000000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 1_961_102_745_098_039);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 30 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 50 * one,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 60120000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 30000000000000,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: 3,
				asset_in: asset_a,
				asset_out: asset_b,
				amount: 20_000_000_000_000,
				sale_price: 39137254901961,
				fee_asset: asset_b,
				fee_amount: 78431372548,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_3,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
				amount: 20_000_000_000_000,
				amount_sold_or_bought: 39215686274509,
				pool_account_id: pair_account,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 60000000000000,
				amount_b: 30000000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 240000000000,
			}
			.into(),
		]);
	});
}

#[test]
fn exact_match_limit_should_work() {
	new_test_ext().execute_with(|| {
		let one: Balance = 1_000_000_000_000;
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1000 * one;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000 * one);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000 * one);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			50_000_000_000_000,
			106_315_789_473_684,
			false,
		));

		let b = <system::Pallet<Test>>::current_block_number();
		let user_2_sell_intention_id = (0, &user_2, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			100_000_000_000_000,
			53_157_894_736_843,
			false,
		));

		let user_3_sell_intention_id = (1, &user_3, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_050_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_899_800_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_949_900_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_100_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000_100_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000_200_000_000_000);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 50 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 100 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 100200000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 50100000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 100 * one,
				amount_b: 50 * one,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 200000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 100000000000,
			}
			.into(),
		]);
	});
}

#[test]
fn matching_limit_scenario_2() {
	new_test_ext().execute_with(|| {
		let one: Balance = 1_000_000_000_000;
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1000 * one;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000 * one);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000 * one);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			100_000_000_000_000,
			223_067_143_076_693,
			false,
		));

		let b = <system::Pallet<Test>>::current_block_number();
		let user_2_sell_intention_id = (0, &user_2, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			180_000_000_000_000,
			220_242_387_444_707,
			false,
		));

		let user_3_sell_intention_id = (1, &user_3, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_100_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_799_397_612_555_294);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_909_820_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_180_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 990_180_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_020_602_387_444_706);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 100 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 180 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 180360000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 90180000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 180 * one,
				amount_b: 90 * one,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 360000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 180000000000,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_2,
				asset_out: asset_a,
				asset_in: asset_b,
				amount: 100_00000000000,
				buy_price: 20201983477752,
				fee_asset: asset_b,
				fee_amount: 40403966954,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 10_000_000_000_000,
				amount_sold_or_bought: 20242387444706,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn matching_limit_scenario_3() {
	new_test_ext().execute_with(|| {
		let one: Balance = 1_000_000_000_000;
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1000 * one;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000 * one);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000 * one);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000 * one);

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			150_000_000_000_000,
			356_315_789_473_684,
			false,
		));

		let b = <system::Pallet<Test>>::current_block_number();
		let user_2_sell_intention_id = (0, &user_2, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			200_000_000_000_000,
			253_157_894_736_843,
			false,
		));

		let user_3_sell_intention_id = (1, &user_3, b, HDX, DOT).using_encoded(<Test as system::Config>::Hashing::hash);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_150_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_694_127_425_805_095);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_899_800_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 100_200_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 950_200_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_105_872_574_194_905);

		expect_events(vec![
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 150 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 200 * one,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: 2,
				amount: 200400000000000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: 3,
				amount: 100200000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 200 * one,
				amount_b: 100 * one,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 400000000000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 200000000000,
			}
			.into(),
			xyk::Event::BuyExecuted {
				who: user_2,
				asset_out: asset_a,
				asset_in: asset_b,
				amount: 50_000_000_000_000,
				buy_price: 105262050094717,
				fee_asset: asset_b,
				fee_amount: 210524100188,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: user_2,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
				amount: 50_000_000_000_000,
				amount_sold_or_bought: 105472574194905,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn process_invalid_intention_should_work() {
	// this test targets the `continue` statement in `process_exchange_intention`
	new_test_ext().execute_with(|| {
		let one: Balance = 1_000_000_000_000;
		let user = ALICE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1000 * one;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user, pool_amount, initial_price);

		let main_intention = ExchangeIntention {
			who: user,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000,
			amount_out: 10 * pool_amount,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user, 0),
		};

		let mut intentions_a = vec![main_intention];

		Exchange::process_exchange_intentions(&pair_account, &mut intentions_a, &mut Vec::<Intention<Test>>::new());

		assert_eq!(Currency::free_balance(asset_a, &user), 99_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user), 98_000_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000_000_000_000_000);
	});
}

#[test]
fn main_intention_greater_than_matched_should_work() {
	// this test targets the `break` statement in `process_exchange_intentions`
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		let main_intention = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000,
			amount_out: 1_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 3_000_000,
			amount_out: 6_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let mut intentions_a = vec![main_intention];
		let mut intentions_b = vec![matched_intention];

		Exchange::process_exchange_intentions(&pair_account, &mut intentions_a, &mut intentions_b);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000_000_002_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 1_999_999_996_015_999);

		assert_eq!(Currency::free_balance(asset_a, &user_1), 99_000_000_001_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_1), 97_999_999_997_996_000);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999_999_997_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 100_000_000_005_988_001);
	});
}

#[test]
fn in_out_calculations_error_should_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		// amount_a_in > amount_b_out scenario
		let main_intention = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 3_000_000,
			amount_out: 1_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 3_000_000,
			amount_out: 2_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let maybe_error = std::panic::catch_unwind(|| {
			let mut intentions_a = vec![main_intention];
			let mut intentions_b = vec![matched_intention];

			Exchange::process_exchange_intentions(&pair_account, &mut intentions_a, &mut intentions_b)
		});
		assert!(maybe_error.is_err());

		// amount_a_in < amount_b_out scenario
		let main_intention = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 1_000_000,
			amount_out: 2_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 2),
		};

		let matched_intention = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000,
			amount_out: 2_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 3),
		};

		let maybe_error = std::panic::catch_unwind(|| {
			let mut intentions_a = vec![main_intention];
			let mut intentions_b = vec![matched_intention];

			Exchange::process_exchange_intentions(&pair_account, &mut intentions_a, &mut intentions_b)
		});
		assert!(maybe_error.is_err());
	});
}

#[test]
fn revert_invalid_direct_trades_should_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Currency::transfer(Origin::signed(ALICE), BOB, asset_b, 98_000_000_000_000_000));

		// amount_a_in > amount_b_out scenario
		let main_intention = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 4_000_000,
			amount_out: 2_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000,
			amount_out: 2_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let intentions_b = vec![&matched_intention];

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention, &intentions_b);

		// amount_a_in < amount_b_out scenario
		let main_intention = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000,
			amount_out: 1_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 2_000_000,
			amount_out: 4_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let intentions_b = vec![&matched_intention];

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention, &intentions_b);

		// amount_a_in == amount_b_out scenario
		let main_intention = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000,
			amount_out: 1_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000,
			amount_out: 2_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let intentions_b = vec![&matched_intention];

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention, &intentions_b);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &user_1), 99_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_1), 0);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 198_000_000_000_000_000);
	});
}

#[test]
fn invalid_transfers_in_resolver_should_not_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Currency::transfer(Origin::signed(ALICE), BOB, asset_b, 98_000_000_000_000_000));

		let main_intention = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000_000_000_000,
			amount_out: 1_000_000_000_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 2_000_000_000_000_000,
			amount_out: 4_000_000_000_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let intentions_b = vec![&matched_intention];

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention, &intentions_b);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &user_1), 99_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_1), 0);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 198_000_000_000_000_000);
	});
}

#[test]
fn trade_limits_in_exact_match_scenario_should_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		assert_ok!(Currency::transfer(Origin::signed(ALICE), BOB, asset_b, 98_000_000_000_000_000));

		let main_intention_1 = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000,
			amount_out: 1_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention_1 = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000,
			amount_out: 2_000_000,
			trade_limit: 10,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let intentions_b = vec![&matched_intention_1];

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention_1, &intentions_b);

		let main_intention_2 = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000,
			amount_out: 1_000_000,
			trade_limit: 100_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_1, 0),
		};

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention_2, &intentions_b);

		let main_intention_3 = ExchangeIntention {
			who: user_1,
			assets: AssetPair {
				asset_in: asset_b,
				asset_out: asset_a,
			},
			amount_in: 2_000_000,
			amount_out: 1_000_000,
			trade_limit: 1_000_000,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_1, 0),
		};

		let matched_intention_3 = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000,
			amount_out: 2_000_000,
			trade_limit: 10_000_000,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let intentions_b = vec![&matched_intention_3];

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention_3, &intentions_b);

		let matched_intention_4 = ExchangeIntention {
			who: user_2,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000,
			amount_out: 2_000_000,
			trade_limit: 100_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user_2, 1),
		};

		let intentions_b = vec![&matched_intention_4];

		<Exchange as Resolver<<Test as system::Config>::AccountId, Intention<Test>, Error<Test>>>::resolve_matched_intentions(&pair_account, &main_intention_3, &intentions_b);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 1_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 2_000_000_000_000_000);

		assert_eq!(Currency::free_balance(asset_a, &user_1), 99_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_1), 0);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 100_000_000_000_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 198_000_000_000_000_000);
	});
}

#[test]
fn correct_matching() {
	new_test_ext().execute_with(|| {
		// Note: this test scenario came from dynamic testing where it led to panic
		// This was due to incorrect matching of some intentions.
		let user_1 = ALICE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 139_637_976_727_557;
		let initial_price = Price::from_float(0.072_057_594_037_927_94);

		initialize_pool(asset_b, asset_a, user_1, pool_amount, initial_price);

		assert_ok!(Exchange::sell(Origin::signed(3), asset_b, asset_a, 1048577, 0, false,));

		assert_ok!(Exchange::buy(
			Origin::signed(4),
			asset_b,
			asset_a,
			7602433,
			4722366482869645213696,
			false,
		));

		assert_ok!(Exchange::buy(
			Origin::signed(6),
			asset_b,
			asset_a,
			65536,
			4722366482869645213696,
			false,
		));

		assert_eq!(Exchange::get_intentions_count((asset_a, asset_b)), 3);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Exchange::get_intentions_count((asset_a, asset_b)), 0);
	});
}

#[test]
fn trade_limit_test() {
	ExtBuilder::default()
		.with_endowed_amount(10_000_000_000_000_000_000)
		.build()
		.execute_with(|| {
			System::set_block_number(1);
			let user_1 = ALICE;
			let asset_a = HDX;
			let asset_b = DOT;
			let pool_amount = 139637976727557;
			let initial_price = Price::from_float(4_722.438_541_558_899_5);

			initialize_pool(asset_b, asset_a, user_1, pool_amount, initial_price);

			assert_ok!(Exchange::buy(
				Origin::signed(4),
				asset_a,
				asset_b,
				281474976710656,
				127547660566528,
				false,
			));

			assert_ok!(Exchange::sell(
				Origin::signed(5),
				asset_a,
				asset_b,
				190275657924608,
				12075401216,
				false,
			));

			assert_eq!(Exchange::get_intentions_count((asset_a, asset_b)), 2);

			<Exchange as OnFinalize<u64>>::on_finalize(9);

			assert_eq!(Exchange::get_intentions_count((asset_a, asset_b)), 0);
		});
}

#[test]
fn register_intention_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Exchange::register_intention(
			&ALICE,
			IntentionType::SELL,
			AssetPair {
				asset_in: HDX,
				asset_out: DOT
			},
			1,
			1,
			1,
			false
		));
		assert_ok!(Exchange::register_intention(
			&ALICE,
			IntentionType::BUY,
			AssetPair {
				asset_in: HDX,
				asset_out: DOT
			},
			1,
			1,
			1,
			false
		));

		assert_eq!(Exchange::get_intentions_count((HDX, DOT)), 2);
		assert_eq!(Exchange::get_intentions((HDX, DOT)).len(), 2);
	});
}

#[test]
fn register_intention_should_return_error_on_overflow() {
	new_test_ext().execute_with(|| {
		ExchangeAssetsIntentionCount::<Test>::insert((HDX, DOT), u32::MAX);
		assert_eq!(Exchange::get_intentions_count((HDX, DOT)), u32::MAX);

		assert_noop!(
			Exchange::register_intention(
				&ALICE,
				IntentionType::SELL,
				AssetPair {
					asset_in: HDX,
					asset_out: DOT
				},
				1,
				1,
				1,
				false
			),
			Error::<Test>::IntentionCountOverflow
		);
	});
}

#[test]
fn execute_amm_transfer_should_work() {
	new_test_ext().execute_with(|| {
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from_float(0.072);
		initialize_pool(asset_b, asset_a, ALICE, pool_amount, initial_price);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		let alice_buy_intention_id = generate_intention_id(&ALICE, 0);
		assert_ok!(Exchange::execute_amm_transfer(
			IntentionType::BUY,
			alice_buy_intention_id,
			&AMMTransfer {
				origin: ALICE,
				assets: AssetPair {
					asset_in: HDX,
					asset_out: DOT
				},
				amount: 1_000_000_000_000,
				amount_out: 1_000_000_000,
				discount: false,
				discount_amount: 0,
				fee: (1000, 1_000_000),
			}
		));

		let alice_sell_intention_id = generate_intention_id(&ALICE, 1);
		assert_ok!(Exchange::execute_amm_transfer(
			IntentionType::SELL,
			alice_sell_intention_id,
			&AMMTransfer {
				origin: ALICE,
				assets: AssetPair {
					asset_in: HDX,
					asset_out: DOT
				},
				amount: 1_000_000_000_000,
				amount_out: 1_000_000_000,
				discount: false,
				discount_amount: 0,
				fee: (1000, 1_000_000),
			}
		));

		expect_events(vec![
			xyk::Event::BuyExecuted {
				who: ALICE,
				asset_out: DOT,
				asset_in: HDX,
				amount: 1_000_000_000_000,
				buy_price: 1_000_000_000,
				fee_asset: HDX,
				fee_amount: 1000000,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: ALICE,
				intention_type: IntentionType::BUY,
				intention_id: alice_buy_intention_id,
				amount: 1_000_000_000_000,
				amount_sold_or_bought: 1_001_000_000,
				pool_account_id: pair_account,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: ALICE,
				asset_in: HDX,
				asset_out: DOT,
				amount: 1_000_000_000_000,
				sale_price: 1_000_000_000,
				fee_asset: HDX,
				fee_amount: 1000000,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: ALICE,
				intention_type: IntentionType::SELL,
				intention_id: alice_sell_intention_id,
				amount: 1_000_000_000_000,
				amount_sold_or_bought: 1_001_000_000,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn resolve_single_intention_should_work() {
	new_test_ext().execute_with(|| {
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from_float(0.072);
		initialize_pool(asset_b, asset_a, ALICE, pool_amount, initial_price);

		let alice_buy_intention_id = generate_intention_id(&ALICE, 0);
		Exchange::resolve_single_intention(&ExchangeIntention {
			who: ALICE,
			assets: AssetPair {
				asset_in: DOT,
				asset_out: HDX,
			},
			amount_in: 150_000_000,
			amount_out: 2_000_000_000,
			trade_limit: 1_500_000_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: alice_buy_intention_id,
		});

		let alice_sell_intention_id = generate_intention_id(&ALICE, 1);
		Exchange::resolve_single_intention(&ExchangeIntention {
			who: ALICE,
			assets: AssetPair {
				asset_in: DOT,
				asset_out: HDX,
			},
			amount_in: 150_000_000,
			amount_out: 2_000_000_000,
			trade_limit: 101_000,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: alice_sell_intention_id,
		});

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		expect_events(vec![
			xyk::Event::BuyExecuted {
				who: ALICE,
				asset_out: HDX,
				asset_in: DOT,
				amount: 2_000_000_000,
				buy_price: 27_778_549_405,
				fee_asset: DOT,
				fee_amount: 55_557_098,
				pool_account_id: pair_account
			}
			.into(),
			Event::IntentionResolvedAMMTrade {
				who: ALICE,
				intention_type: IntentionType::BUY,
				intention_id: alice_buy_intention_id,
				amount: 2_000_000_000,
				amount_sold_or_bought: 27_834_106_503,
				pool_account_id: pair_account,
			}
			.into(),
			xyk::Event::SellExecuted {
				who: ALICE,
				asset_in: DOT,
				asset_out: HDX,
				amount: 150000000,
				sale_price: 10777799,
				fee_asset: HDX,
				fee_amount: 21598,
				pool_account_id: pair_account
			}.into(),
			Event::IntentionResolvedAMMTrade {
				who: ALICE,
				intention_type: IntentionType::SELL,
				intention_id: alice_sell_intention_id,
				amount: 150000000,
				amount_sold_or_bought: 10799397,
				pool_account_id: pair_account,
			}
			.into(),
		]);
	});
}

#[test]
fn verify_intention_should_work() {
	new_test_ext().execute_with(|| {
		let user = ALICE;
		let asset_a = HDX;
		let asset_b = DOT;
		let pool_amount = 1_000_000_000_000_000;
		let initial_price = Price::from_float(2.0);
		initialize_pool(asset_a, asset_b, user, pool_amount, initial_price);

		assert!(Exchange::verify_intention(&Intention::<Test> {
			who: user,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000_000,
			amount_out: 2_000_000_000,
			trade_limit: 3_000_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user, 0),
		}));

		assert!(Exchange::verify_intention(&Intention::<Test> {
			who: user,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000_000,
			amount_out: 2_000_000_000,
			trade_limit: 100_000_000,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user, 0),
		}));

		assert!(!Exchange::verify_intention(&Intention::<Test> {
			who: user,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000_000,
			amount_out: 2_000_000_000,
			trade_limit: 100_000_000,
			discount: false,
			sell_or_buy: IntentionType::BUY,
			intention_id: generate_intention_id(&user, 0),
		}));

		assert!(!Exchange::verify_intention(&Intention::<Test> {
			who: user,
			assets: AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			},
			amount_in: 1_000_000_000,
			amount_out: 2_000_000_000,
			trade_limit: 10_000_000_000,
			discount: false,
			sell_or_buy: IntentionType::SELL,
			intention_id: generate_intention_id(&user, 0),
		}));
	});
}

#[test]
fn direct_sell_sell_transfers_without_other_asset_should_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		// BOB doesn't need to own asset_b when selling asset_a
		assert_ok!(Currency::transfer(
			Origin::signed(user_2),
			user_1,
			asset_b,
			100_000_000_000_000_000
		));
		// CHARLIE doesn't need to own asset_a when selling asset_b
		assert_ok!(Currency::transfer(
			Origin::signed(user_3),
			user_1,
			asset_a,
			100_000_000_000_000_000
		));

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000,
			400,
			false,
		));
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			2_000_000,
			400,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100_000_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200_000_000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999_999_999_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 1_996_000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 998_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_999_999_998_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100002000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200004000);

		expect_events(vec![
			orml_tokens::Event::Transfer {
				currency_id: asset_b,
				from: user_2,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			orml_tokens::Event::Transfer {
				currency_id: asset_a,
				from: user_3,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 2_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_2,
				amount: 1_000_000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_3,
				amount: 2_000_000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1_000_000,
				amount_b: 2_000_000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 2_000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 4_000,
			}
			.into(),
		]);
	});
}

#[test]
fn direct_buy_buy_transfers_without_other_asset_should_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		// BOB doesn't need to own asset_a when buying asset_a
		assert_ok!(Currency::transfer(
			Origin::signed(user_2),
			user_1,
			asset_a,
			100_000_000_000_000_000
		));
		// CHARLIE doesn't need to own asset_b when buying asset_b
		assert_ok!(Currency::transfer(
			Origin::signed(user_3),
			user_1,
			asset_b,
			100_000_000_000_000_000
		));

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000,
			40_000_000,
			false,
		));
		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_b,
			asset_a,
			2_000_000,
			40_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 1_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_999_999_997_996_000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_999_999_998_998_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 2_000_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100_002_000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200_004_000);

		expect_events(vec![
			orml_tokens::Event::Transfer {
				currency_id: asset_a,
				from: user_2,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			orml_tokens::Event::Transfer {
				currency_id: asset_b,
				from: user_3,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a: asset_b,
				asset_b: asset_a,
				amount: 2_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_3,
				amount: 1_002_000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_2,
				amount: 2_004_000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_3,
				account_id_b: user_2,
				intention_id_a: user_3_sell_intention_id,
				intention_id_b: user_2_sell_intention_id,
				amount_a: 1_000_000,
				amount_b: 2_000_000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_a,
				fee_amount: 2_000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 4_000,
			}
			.into(),
		]);
	});
}

#[test]
fn direct_sell_buy_transfers_without_other_asset_should_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		// BOB doesn't need to own asset_b when selling asset_a
		assert_ok!(Currency::transfer(
			Origin::signed(user_2),
			user_1,
			asset_b,
			100_000_000_000_000_000
		));
		// CHARLIE doesn't need to own asset_a when buying asset_a
		assert_ok!(Currency::transfer(
			Origin::signed(user_3),
			user_1,
			asset_a,
			100_000_000_000_000_000
		));

		assert_ok!(Exchange::sell(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000,
			40_000,
			false,
		));
		assert_ok!(Exchange::buy(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			1_000_000,
			40_000_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 99_999_999_999_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 1_996_000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 1_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 99_999_999_997_996_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200008000);

		expect_events(vec![
			orml_tokens::Event::Transfer {
				currency_id: asset_b,
				from: user_2,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			orml_tokens::Event::Transfer {
				currency_id: asset_a,
				from: user_3,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 1_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_2,
				amount: 1_000_000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_3,
				amount: 2_004_000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_2,
				account_id_b: user_3,
				intention_id_a: user_2_sell_intention_id,
				intention_id_b: user_3_sell_intention_id,
				amount_a: 1_000_000,
				amount_b: 2_000_000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_3,
				intention_id: user_3_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 8_000,
			}
			.into(),
		]);
	});
}

#[test]
fn direct_buy_sell_transfers_without_other_asset_should_work() {
	new_test_ext().execute_with(|| {
		let user_1 = ALICE;
		let user_2 = BOB;
		let user_3 = CHARLIE;
		let asset_a = ETH;
		let asset_b = DOT;
		let pool_amount = 100_000_000;
		let initial_price = Price::from(2);

		let pair_account = XYKPallet::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		initialize_pool(asset_a, asset_b, user_1, pool_amount, initial_price);

		// BOB doesn't need to own asset_a when buying asset_a
		assert_ok!(Currency::transfer(
			Origin::signed(user_2),
			user_1,
			asset_a,
			100_000_000_000_000_000
		));
		// CHARLIE doesn't need to own asset_b when selling asset_a
		assert_ok!(Currency::transfer(
			Origin::signed(user_3),
			user_1,
			asset_b,
			100_000_000_000_000_000
		));

		assert_ok!(Exchange::buy(
			Origin::signed(user_2),
			asset_a,
			asset_b,
			1_000_000,
			40_000_000,
			false,
		));
		assert_ok!(Exchange::sell(
			Origin::signed(user_3),
			asset_a,
			asset_b,
			1_000_000,
			40_000,
			false,
		));
		let user_2_sell_intention_id = generate_intention_id(&user_2, 0);
		let user_3_sell_intention_id = generate_intention_id(&user_3, 1);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200000000);

		<Exchange as OnFinalize<u64>>::on_finalize(9);

		assert_eq!(Currency::free_balance(asset_a, &user_2), 1_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_2), 99_999_999_997_996_000);

		assert_eq!(Currency::free_balance(asset_a, &user_3), 99_999_999_999_000_000);
		assert_eq!(Currency::free_balance(asset_b, &user_3), 1_996_000);

		assert_eq!(Currency::free_balance(asset_a, &pair_account), 100000000);
		assert_eq!(Currency::free_balance(asset_b, &pair_account), 200008000);

		expect_events(vec![
			orml_tokens::Event::Transfer {
				currency_id: asset_a,
				from: user_2,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			orml_tokens::Event::Transfer {
				currency_id: asset_b,
				from: user_3,
				to: ALICE,
				amount: 100000000000000000,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_2,
				asset_a,
				asset_b,
				amount: 1_000_000,
				intention_type: IntentionType::BUY,
				intention_id: user_2_sell_intention_id,
			}
			.into(),
			Event::IntentionRegistered {
				who: user_3,
				asset_a,
				asset_b,
				amount: 1_000_000,
				intention_type: IntentionType::SELL,
				intention_id: user_3_sell_intention_id,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_a,
				who: user_3,
				amount: 1_000_000,
			}
			.into(),
			orml_tokens::Event::Reserved {
				currency_id: asset_b,
				who: user_2,
				amount: 2_004_000,
			}
			.into(),
			Event::IntentionResolvedDirectTrade {
				account_id_a: user_3,
				account_id_b: user_2,
				intention_id_a: user_3_sell_intention_id,
				intention_id_b: user_2_sell_intention_id,
				amount_a: 1_000_000,
				amount_b: 2_000_000,
			}
			.into(),
			Event::IntentionResolvedDirectTradeFees {
				who: user_2,
				intention_id: user_2_sell_intention_id,
				fee_receiver: pair_account,
				asset_id: asset_b,
				fee_amount: 8_000,
			}
			.into(),
		]);
	});
}
