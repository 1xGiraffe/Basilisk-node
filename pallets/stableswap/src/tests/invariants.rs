use crate::tests::mock::*;
use crate::traits::ShareAccountIdFor;
use crate::types::PoolInfo;
use frame_support::assert_ok;
use sp_runtime::{FixedU128, Permill};

use hydra_dx_math::stableswap::two_asset_pool_math::calculate_d;
use proptest::prelude::*;
use proptest::proptest;

pub const ONE: Balance = 1_000_000_000_000;

const RESERVE_RANGE: (Balance, Balance) = (500_000 * ONE, 100_000_000 * ONE);

fn trade_amount() -> impl Strategy<Value = Balance> {
	1000..100_000 * ONE
}

fn asset_reserve() -> impl Strategy<Value = Balance> {
	RESERVE_RANGE.0..RESERVE_RANGE.1
}

fn amplification() -> impl Strategy<Value = u16> {
	2..10000u16
}

fn trade_fee() -> impl Strategy<Value = Permill> {
	(0f64..50f64).prop_map(Permill::from_float)
}

fn percent() -> impl Strategy<Value = Permill> {
	(1u32..100u32).prop_map(Permill::from_percent)
}

#[macro_export]
macro_rules! assert_eq_approx {
	( $x:expr, $y:expr, $z:expr, $r:expr) => {{
		let diff = if $x >= $y { $x - $y } else { $y - $x };
		if diff > $z {
			panic!("\n{} not equal\n left: {:?}\nright: {:?}\n", $r, $x, $y);
		}
	}};
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]
	#[test]
	fn add_liquidity_price_no_changes(
		initial_liquidity in asset_reserve(),
		added_liquidity in asset_reserve(),
		amplification in amplification(),
		fee in trade_fee()

	) {
		let asset_a: AssetId = 1000;
		let asset_b: AssetId = 2000;

		ExtBuilder::default()
			.with_endowed_accounts(vec![
				(BOB, asset_a, added_liquidity),
				(BOB, asset_b, added_liquidity * 1000),
				(ALICE, asset_a, initial_liquidity),
				(ALICE, asset_b, initial_liquidity),
			])
			.with_registered_asset("one".as_bytes().to_vec(), asset_a)
			.with_registered_asset("two".as_bytes().to_vec(), asset_b)
			.with_pool(
				ALICE,
				PoolInfo::<AssetId> {
					assets: vec![asset_a,asset_b].try_into().unwrap(),
					amplification,
					fee,
				},
				InitialLiquidity{ account: ALICE, asset:asset_a, amount:initial_liquidity},
			)
			.build()
			.execute_with(|| {
				let pool_id = get_pool_id_at(0);

				let pool_account = AccountIdConstructor::from_assets(&vec![asset_a, asset_b], None);

				let asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);

				assert_ok!(Stableswap::add_liquidity(
					Origin::signed(BOB),
					pool_id,
					asset_a,
					added_liquidity
				));

				let new_asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let new_asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);

				assert_eq_approx!(
					FixedU128::from((asset_a_reserve, asset_b_reserve)),
					FixedU128::from((new_asset_a_reserve, new_asset_b_reserve)),
					FixedU128::from_float(0.0000000001),
					"Price has changed after add liquidity"
				);
			});
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]
	#[test]
	fn remove_liquidity_price_no_changes(
		initial_liquidity in asset_reserve(),
		added_liquidity in asset_reserve(),
		amplification in amplification(),
		fee in trade_fee(),
		withdraw_percentage in percent(),
	) {

		let asset_a: AssetId = 1000;
		let asset_b: AssetId = 2000;
		ExtBuilder::default()
			.with_endowed_accounts(vec![
				(BOB, asset_a, added_liquidity),
				(BOB, asset_b, added_liquidity * 1000),
				(ALICE, asset_a, initial_liquidity),
				(ALICE, asset_b, initial_liquidity),
			])
			.with_registered_asset("one".as_bytes().to_vec(), asset_a)
			.with_registered_asset("two".as_bytes().to_vec(), asset_b)
			.with_pool(
				ALICE,
				PoolInfo::<AssetId> {
					assets: vec![asset_a,asset_b].try_into().unwrap(),
					amplification,
					fee,
				},
				InitialLiquidity{ account: ALICE, asset:asset_a, amount:initial_liquidity},
			)
			.build()
			.execute_with(|| {
				let pool_id = get_pool_id_at(0);

				let pool_account = AccountIdConstructor::from_assets(&vec![asset_a, asset_b], None);

				assert_ok!(Stableswap::add_liquidity(
					Origin::signed(BOB),
					pool_id,
					asset_a,
					added_liquidity
				));

				let shares = Tokens::free_balance(pool_id.0, &BOB);

				let amount_withdrawn = withdraw_percentage.mul_floor(shares);

				let asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);
				assert_ok!(Stableswap::remove_liquidity(Origin::signed(BOB), pool_id, amount_withdrawn));

				let new_asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let new_asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);

				assert_eq_approx!(FixedU128::from((asset_a_reserve,asset_b_reserve)),
					FixedU128::from((new_asset_a_reserve,new_asset_b_reserve)),
					FixedU128::from_float(0.0000000001),
					"Price has changed after remove liquidity");
			});
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]
	#[test]
	fn sell_invariants(
		initial_liquidity in asset_reserve(),
		amount in trade_amount(),
		amplification in amplification(),
	) {
		let asset_a: AssetId = 1000;
		let asset_b: AssetId = 2000;

		ExtBuilder::default()
			.with_endowed_accounts(vec![
				(BOB, asset_a, amount),
				(ALICE, asset_a, initial_liquidity),
				(ALICE, asset_b, initial_liquidity),
			])
			.with_registered_asset("one".as_bytes().to_vec(), asset_a)
			.with_registered_asset("two".as_bytes().to_vec(), asset_b)
			.with_pool(
				ALICE,
				PoolInfo::<AssetId> {
					assets: vec![asset_a,asset_b].try_into().unwrap(),
					amplification,
					fee: Permill::from_percent(0),
				},
				InitialLiquidity{ account: ALICE, asset:asset_a, amount:initial_liquidity},
			)
			.build()
			.execute_with(|| {
				let pool_id = get_pool_id_at(0);

				let pool_account = AccountIdConstructor::from_assets(&vec![asset_a, asset_b], None);

				let asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);
				let ann: Balance = amplification as u128 * 2 * 2;
				let d_prev = calculate_d::<128u8>(&[asset_a_reserve,asset_b_reserve], ann, 1u128).unwrap();

				assert_ok!(Stableswap::sell(
					Origin::signed(BOB),
					pool_id,
					asset_a,
					asset_b,
					amount,
					0u128, // not interested in this
				));

				let asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);
				let ann: Balance = amplification as u128 * 2 * 2;
				let d = calculate_d::<128u8>(&[asset_a_reserve,asset_b_reserve], ann, 1u128).unwrap();

				assert!(d >= d_prev);
				assert!(d - d_prev <= 10u128);
			});
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]
	#[test]
	fn buy_invariants(
		initial_liquidity in asset_reserve(),
		amount in trade_amount(),
		amplification in amplification(),
	) {
		let asset_a: AssetId = 1;
		let asset_b: AssetId = 2;

		ExtBuilder::default()
			.with_endowed_accounts(vec![
				(BOB, asset_a, amount * 1000),
				(ALICE, asset_a, initial_liquidity),
				(ALICE, asset_b, initial_liquidity),
			])
			.with_registered_asset("one".as_bytes().to_vec(), asset_a)
			.with_registered_asset("two".as_bytes().to_vec(), asset_b)
			.with_pool(
				ALICE,
				PoolInfo::<AssetId> {
					assets: vec![asset_a,asset_b].try_into().unwrap(),
					amplification,
					fee: Permill::from_percent(0),
				},
				InitialLiquidity{ account: ALICE, asset:asset_a, amount:initial_liquidity},
			)
			.build()
			.execute_with(|| {

				let pool_id = get_pool_id_at(0);

				let pool_account = AccountIdConstructor::from_assets(&vec![asset_a, asset_b], None);

				let asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);
				let ann: Balance = amplification as u128 * 2 * 2;
				let d_prev = calculate_d::<128u8>(&[asset_a_reserve,asset_b_reserve], ann, 1u128).unwrap();

				assert_ok!(Stableswap::buy(
					Origin::signed(BOB),
					pool_id,
					asset_b,
					asset_a,
					amount,
					u128::MAX, // not interested in this
				));
				let asset_a_reserve = Tokens::free_balance(asset_a, &pool_account);
				let asset_b_reserve = Tokens::free_balance(asset_b, &pool_account);
				let ann: Balance = amplification as u128 * 2 * 2;
				let d = calculate_d::<128u8>(&[asset_a_reserve,asset_b_reserve], ann, 1u128).unwrap();

				assert!(d >= d_prev);
				assert!(d - d_prev <= 10u128);
			});
	}
}
