use crate::tests::mock::*;
use crate::traits::ShareAccountIdFor;
use crate::types::{PoolAssets, PoolId};
use crate::{assert_balance, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::Permill;

#[test]
fn add_initial_liquidity_works() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 200 * ONE),
			(BOB, 2, 200 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.build()
		.execute_with(|| {
			let asset_a: AssetId = 1;
			let asset_b: AssetId = 2;
			let amplification: u16 = 100;

			let pool_id = PoolId(retrieve_current_asset_id());

			assert_ok!(Stableswap::create_pool(
				Origin::signed(ALICE),
				(asset_a, asset_b),
				amplification,
				Permill::from_percent(0)
			));

			let initial_liquidity_amount = 100 * ONE;

			let pool_account = AccountIdConstructor::from_assets(&PoolAssets(asset_a, asset_b), None);

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_a,
				initial_liquidity_amount
			));

			assert_balance!(BOB, asset_a, 100 * ONE);
			assert_balance!(BOB, asset_b, 100 * ONE);
			assert_balance!(BOB, pool_id.0, 200 * ONE);
			assert_balance!(pool_account, asset_a, 100 * ONE);
			assert_balance!(pool_account, asset_b, 100 * ONE);
		});
}

#[test]
fn add_initial_liquidity_with_insufficient_balance_fails() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 200 * ONE),
			(BOB, 2, 20 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.build()
		.execute_with(|| {
			let asset_a: AssetId = 1;
			let asset_b: AssetId = 2;
			let amplification: u16 = 100;

			let pool_id = PoolId(retrieve_current_asset_id());

			assert_ok!(Stableswap::create_pool(
				Origin::signed(ALICE),
				(asset_a, asset_b),
				amplification,
				Permill::from_percent(0)
			));

			let initial_liquidity_amount = 100 * ONE;

			let pool_account = AccountIdConstructor::from_assets(&PoolAssets(asset_a, asset_b), None);

			assert_noop!(
				Stableswap::add_liquidity(Origin::signed(BOB), pool_id, asset_a, initial_liquidity_amount),
				Error::<Test>::InsufficientBalance
			);

			assert_balance!(BOB, asset_a, 200 * ONE);
			assert_balance!(BOB, asset_b, 20 * ONE);
			assert_balance!(BOB, pool_id.0, 0u128);
			assert_balance!(pool_account, asset_a, 0u128);
			assert_balance!(pool_account, asset_b, 0u128);
		});
}
#[test]
fn add_liquidity_works() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;

	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 200 * ONE),
			(BOB, 2, 200 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);

			let amount_added = 100 * ONE;

			let pool_account = AccountIdConstructor::from_assets(&PoolAssets(asset_a, asset_b), None);

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_a,
				amount_added
			));

			assert_balance!(BOB, asset_a, 100 * ONE);
			assert_balance!(BOB, asset_b, 100 * ONE);
			assert_balance!(BOB, pool_id.0, 199999999999996u128);
			assert_balance!(pool_account, asset_a, 200 * ONE);
			assert_balance!(pool_account, asset_b, 200 * ONE);
		});
}

#[test]
fn add_liquidity_other_asset_works() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;

	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 200 * ONE),
			(BOB, 2, 200 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);

			let amount_added = 100 * ONE;

			let pool_account = AccountIdConstructor::from_assets(&PoolAssets(asset_a, asset_b), None);

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_b,
				amount_added
			));

			assert_balance!(BOB, asset_a, 100 * ONE);
			assert_balance!(BOB, asset_b, 100 * ONE);
			assert_balance!(BOB, pool_id.0, 199999999999996u128);
			assert_balance!(pool_account, asset_a, 200 * ONE);
			assert_balance!(pool_account, asset_b, 200 * ONE);
		});
}

#[test]
fn add_insufficient_liquidity_fails() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;

	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 200 * ONE),
			(BOB, 2, 200 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);
			let amount_added = 100;

			assert_noop!(
				Stableswap::add_liquidity(Origin::signed(BOB), pool_id, asset_a, amount_added),
				Error::<Test>::InsufficientTradingAmount
			);
		});
}

#[test]
fn remove_all_liquidity_works() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 100 * ONE),
			(BOB, 2, 100 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);
			let amount_added = 100 * ONE;

			let pool_account = AccountIdConstructor::from_assets(&PoolAssets(asset_a, asset_b), None);

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_a,
				amount_added
			));

			let shares = Tokens::free_balance(pool_id.0, &BOB);

			assert_eq!(shares, 199999999999996u128);

			assert_ok!(Stableswap::remove_liquidity(Origin::signed(BOB), pool_id, shares));

			assert_balance!(BOB, asset_a, 100 * ONE - 2);
			assert_balance!(BOB, asset_b, 100 * ONE - 2);

			assert_balance!(BOB, pool_id.0, 0u128);

			assert_balance!(pool_account, asset_a, 100 * ONE + 2);
			assert_balance!(pool_account, asset_b, 100 * ONE + 2);
		});
}

#[test]
fn remove_partial_liquidity_works() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;

	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 100 * ONE),
			(BOB, 2, 100 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);
			let amount_added = 100 * ONE;

			let pool_account = AccountIdConstructor::from_assets(&PoolAssets(asset_a, asset_b), None);

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_a,
				amount_added
			));

			let asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
			let asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);

			let shares = Tokens::free_balance(pool_id.0, &BOB);

			assert_eq!(shares, 199999999999996u128);

			let shares_withdrawn = 80_000_000_000_000u128;

			let lp_a = Tokens::free_balance(asset_a, &BOB);
			let lp_b = Tokens::free_balance(asset_b, &BOB);

			assert_ok!(Stableswap::remove_liquidity(
				Origin::signed(BOB),
				pool_id,
				shares_withdrawn
			));

			let lp_a = Tokens::free_balance(asset_a, &BOB) - lp_a;
			let lp_b = Tokens::free_balance(asset_b, &BOB) - lp_b;

			assert_balance!(BOB, asset_a, 40 * ONE);
			assert_balance!(BOB, asset_b, 40 * ONE);

			assert_balance!(BOB, pool_id.0, shares - shares_withdrawn);

			let a_diff = asset_a_reserve - Tokens::free_balance(asset_a, &pool_account);
			let b_diff = asset_b_reserve - Tokens::free_balance(asset_b, &pool_account);

			assert_balance!(pool_account, asset_a, 160 * ONE);
			assert_balance!(pool_account, asset_b, 160 * ONE);

			assert_eq!(a_diff, lp_a);
			assert_eq!(b_diff, lp_b);
		});
}

#[test]
fn add_liquidity_with_insufficient_amount_fails() {
	let asset_a: AssetId = 1000;
	let asset_b: AssetId = 2000;

	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1000, 200 * ONE),
			(BOB, 2000, 200 * ONE),
			(ALICE, 1000, 200 * ONE),
			(ALICE, 2000, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1000)
		.with_registered_asset("two".as_bytes().to_vec(), 2000)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);

			assert_noop!(
				Stableswap::add_liquidity(Origin::signed(BOB), pool_id, asset_a, 100u128),
				Error::<Test>::InsufficientTradingAmount
			);

			assert_noop!(
				Stableswap::add_liquidity(Origin::signed(BOB), pool_id, asset_a, 1000 * ONE,),
				Error::<Test>::InsufficientBalance
			);
		});
}

#[test]
fn add_liquidity_with_invalid_data_fails() {
	let asset_a: AssetId = 1000;
	let asset_b: AssetId = 2000;
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1000, 200 * ONE),
			(BOB, 3000, 200 * ONE),
			(ALICE, 1000, 200 * ONE),
			(ALICE, 2000, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1000)
		.with_registered_asset("two".as_bytes().to_vec(), 2000)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);

			assert_noop!(
				Stableswap::add_liquidity(
					Origin::signed(BOB),
					PoolId(pool_id.0 + 1), // let's just take next id
					asset_a,
					100 * ONE,
				),
				Error::<Test>::PoolNotFound
			);

			assert_noop!(
				Stableswap::add_liquidity(Origin::signed(BOB), pool_id, 3000, 100 * ONE,),
				Error::<Test>::AssetNotInPool
			);

			assert_noop!(
				Stableswap::add_liquidity(Origin::signed(BOB), pool_id, asset_a, 100 * ONE,),
				Error::<Test>::InsufficientBalance
			);
		});
}

#[test]
fn remove_liquidity_with_invalid_amounts_fails() {
	let asset_a: AssetId = 1000;
	let asset_b: AssetId = 2000;
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1000, 100 * ONE),
			(BOB, 2000, 100 * ONE),
			(ALICE, 1000, 200 * ONE),
			(ALICE, 2000, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1000)
		.with_registered_asset("two".as_bytes().to_vec(), 2000)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);

			let amount_added = 100 * ONE;

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_a,
				amount_added
			));

			assert_noop!(
				Stableswap::remove_liquidity(Origin::signed(BOB), pool_id, 0u128),
				Error::<Test>::InvalidAssetAmount
			);

			assert_noop!(
				Stableswap::remove_liquidity(Origin::signed(BOB), pool_id, 2000 * ONE),
				Error::<Test>::InsufficientShares
			);
		});
}

#[test]
fn remove_liquidity_with_invalid_data_fails() {
	let asset_a: AssetId = 1000;
	let asset_b: AssetId = 2000;

	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1000, 100 * ONE),
			(BOB, 2000, 100 * ONE),
			(ALICE, 1000, 200 * ONE),
			(ALICE, 2000, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1000)
		.with_registered_asset("two".as_bytes().to_vec(), 2000)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);
			let amount_added = 100 * ONE;

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_a,
				amount_added
			));

			assert_ok!(Tokens::set_balance(Origin::root(), BOB, 5, 100 * ONE, 0u128));

			assert_noop!(
				Stableswap::remove_liquidity(Origin::signed(BOB), PoolId(pool_id.0 + 1), 100 * ONE),
				Error::<Test>::PoolNotFound
			);
		});
}

#[test]
fn remove_partial_with_insufficient_remaining_works() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(BOB, 1, 100 * ONE),
			(BOB, 2, 100 * ONE),
			(ALICE, 1, 200 * ONE),
			(ALICE, 2, 200 * ONE),
		])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.with_pool(
			ALICE,
			(asset_a, asset_b),
			100u16,
			Permill::from_percent(0),
			(ALICE, asset_a, 100 * ONE),
		)
		.build()
		.execute_with(|| {
			let pool_id = get_pool_id_at(0);

			let amount_added = 100 * ONE;

			assert_ok!(Stableswap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				asset_a,
				amount_added
			));

			let shares = Tokens::free_balance(pool_id.0, &BOB);

			// Withdraw so much that remaining will be below ED
			let shares_withdrawn = shares - MinimumLiquidity::get() + 1u128;

			assert_noop!(
				Stableswap::remove_liquidity(Origin::signed(BOB), pool_id, shares_withdrawn),
				Error::<Test>::InsufficientShareBalance
			);
		});
}
