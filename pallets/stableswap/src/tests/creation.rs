use crate::tests::mock::*;
use crate::types::PoolInfo;
use crate::Pools;
use crate::{Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_runtime::Permill;

#[test]
fn create_two_asset_pool_should_work_when_assets_are_registered() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 1, 200 * ONE), (ALICE, 2, 200 * ONE)])
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			let pool_id = retrieve_current_asset_id();

			assert_ok!(Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![asset_a, asset_b],
				100u16,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			let pool = <Pools<Test>>::get(pool_id);
			assert!(pool.is_some());

			assert_eq!(
				pool.unwrap(),
				PoolInfo::<AssetId> {
					assets: vec![asset_a, asset_b].try_into().unwrap(),
					amplification: 100u16,
					trade_fee: Permill::from_percent(10),
					withdraw_fee: Permill::from_percent(20),
				}
			);
		});
}

#[test]
fn create_multi_asset_pool_should_work_when_assets_are_registered() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let asset_c: AssetId = 3;
	let asset_d: AssetId = 4;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 1, 200 * ONE), (ALICE, 2, 200 * ONE)])
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.with_registered_asset("three".as_bytes().to_vec(), asset_c)
		.with_registered_asset("four".as_bytes().to_vec(), asset_d)
		.build()
		.execute_with(|| {
			let pool_id = retrieve_current_asset_id();

			assert_ok!(Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![asset_a, asset_b, asset_c, asset_d],
				100u16,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			let pool = <Pools<Test>>::get(pool_id);
			assert!(pool.is_some());

			assert_eq!(
				pool.unwrap(),
				PoolInfo::<AssetId> {
					assets: vec![asset_a, asset_b, asset_c, asset_d].try_into().unwrap(),
					amplification: 100u16,
					trade_fee: Permill::from_percent(10),
					withdraw_fee: Permill::from_percent(20),
				}
			);
		});
}

#[test]
fn create_multi_asset_pool_should_work_when_assets_are_not_in_order() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let asset_c: AssetId = 3;
	let asset_d: AssetId = 4;
	let asset_e: AssetId = 5;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 1, 200 * ONE), (ALICE, 2, 200 * ONE)])
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.with_registered_asset("three".as_bytes().to_vec(), asset_c)
		.with_registered_asset("four".as_bytes().to_vec(), asset_d)
		.with_registered_asset("five".as_bytes().to_vec(), asset_e)
		.build()
		.execute_with(|| {
			let pool_id = retrieve_current_asset_id();

			assert_ok!(Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![asset_c, asset_a, asset_d, asset_b, asset_e],
				100u16,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			let pool = <Pools<Test>>::get(pool_id);
			assert!(pool.is_some());

			assert_eq!(
				pool.unwrap(),
				PoolInfo::<AssetId> {
					assets: vec![asset_a, asset_b, asset_c, asset_d, asset_e].try_into().unwrap(),
					amplification: 100u16,
					trade_fee: Permill::from_percent(10),
					withdraw_fee: Permill::from_percent(20),
				}
			);
		});
}

#[test]
fn create_pool_should_fail_when_number_of_asset_exceeds() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_a: AssetId = 1;
		let asset_b: AssetId = 2;
		let asset_c: AssetId = 3;
		let asset_d: AssetId = 4;
		let asset_e: AssetId = 5;
		let asset_f: AssetId = 6;
		let amplification: u16 = 100;

		assert_noop!(
			Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![asset_a, asset_b, asset_c, asset_d, asset_e, asset_f],
				amplification,
				Permill::from_percent(0),
				Permill::from_percent(0),
			),
			Error::<Test>::MaxAssetsExceeded
		);
	});
}

#[test]
fn create_pool_should_fail_when_number_of_asset_is_below_two() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_a: AssetId = 1;
		let amplification: u16 = 100;

		assert_noop!(
			Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![],
				amplification,
				Permill::from_percent(0),
				Permill::from_percent(0),
			),
			Error::<Test>::MinAssets
		);
		assert_noop!(
			Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![asset_a],
				amplification,
				Permill::from_percent(0),
				Permill::from_percent(0),
			),
			Error::<Test>::MinAssets
		);
	});
}

#[test]
fn create_pool_should_fail_when_asset_ids_are_not_unique() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let asset_c: AssetId = 3;
	let asset_d: AssetId = 1;
	ExtBuilder::default()
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.with_registered_asset("three".as_bytes().to_vec(), asset_c)
		.build()
		.execute_with(|| {
			let amplification: u16 = 100;

			assert_noop!(
				Stableswap::create_pool(
					Origin::signed(ALICE),
					vec![asset_a, asset_b, asset_c, asset_d],
					amplification,
					Permill::from_percent(0),
					Permill::from_percent(0),
				),
				Error::<Test>::SameAssets
			);
		});
}

#[test]
fn create_pool_should_fail_when_asset_is_not_registered() {
	ExtBuilder::default()
		.with_registered_asset("one".as_bytes().to_vec(), 1000)
		.build()
		.execute_with(|| {
			let registered: AssetId = 1000;
			let not_registered: AssetId = 2000;
			let amplification: u16 = 100;

			assert_noop!(
				Stableswap::create_pool(
					Origin::signed(ALICE),
					vec![not_registered, registered],
					amplification,
					Permill::from_percent(0),
					Permill::from_percent(0),
				),
				Error::<Test>::AssetNotRegistered
			);

			assert_noop!(
				Stableswap::create_pool(
					Origin::signed(ALICE),
					vec![registered, not_registered],
					amplification,
					Permill::from_percent(0),
					Permill::from_percent(0),
				),
				Error::<Test>::AssetNotRegistered
			);
		});
}

#[test]
fn create_pool_should_fail_when_same_pool_already_exists() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 1, 200 * ONE), (ALICE, 2, 200 * ONE)])
		.with_registered_asset("one".as_bytes().to_vec(), 1)
		.with_registered_asset("two".as_bytes().to_vec(), 2)
		.build()
		.execute_with(|| {
			let asset_a: AssetId = 1;
			let asset_b: AssetId = 2;
			let amplification: u16 = 100;

			assert_ok!(Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![asset_a, asset_b],
				amplification,
				Permill::from_percent(0),
				Permill::from_percent(0),
			));

			assert_noop!(
				Stableswap::create_pool(
					Origin::signed(ALICE),
					vec![asset_b, asset_a],
					amplification,
					Permill::from_percent(10),
					Permill::from_percent(20),
				),
				Error::<Test>::PoolExists
			);
		});
}

#[test]
fn create_pool_should_fail_when_amplification_is_out_of_range() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 1000, 200 * ONE), (ALICE, 2000, 200 * ONE)])
		.with_registered_asset("one".as_bytes().to_vec(), 1000)
		.with_registered_asset("two".as_bytes().to_vec(), 2000)
		.build()
		.execute_with(|| {
			let asset_a: AssetId = 1000;
			let asset_b: AssetId = 2000;
			let amplification_min: u16 = 1;
			let amplification_max: u16 = 10_001;

			assert_noop!(
				Stableswap::create_pool(
					Origin::signed(ALICE),
					vec![asset_a, asset_b],
					amplification_min,
					Permill::from_percent(0),
					Permill::from_percent(0),
				),
				Error::<Test>::InvalidAmplification
			);

			assert_noop!(
				Stableswap::create_pool(
					Origin::signed(ALICE),
					vec![asset_a, asset_b],
					amplification_max,
					Permill::from_percent(0),
					Permill::from_percent(0)
				),
				Error::<Test>::InvalidAmplification
			);
		});
}

#[test]
fn create_pool_should_emit_event() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let asset_c: AssetId = 3;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, 1, 200 * ONE), (ALICE, 2, 200 * ONE)])
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.with_registered_asset("three".as_bytes().to_vec(), asset_c)
		.build()
		.execute_with(|| {
			System::set_block_number(1);
			let pool_id = retrieve_current_asset_id();

			assert_ok!(Stableswap::create_pool(
				Origin::signed(ALICE),
				vec![asset_c, asset_b, asset_a],
				100u16,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			let event = Event::PoolCreated {
				pool_id,
				assets: vec![asset_a, asset_b, asset_c],
				amplification: 100u16,
				trade_fee: Permill::from_percent(10),
				withdraw_fee: Permill::from_percent(20),
			};
			assert_eq!(last_event(), event.into());
		});
}
