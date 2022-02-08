#![cfg(test)]
use crate::kusama_test_net::*;

use frame_support::{assert_noop, assert_ok};

use pallet_transaction_multi_payment::Price;
use polkadot_xcm::latest::prelude::*;

use cumulus_primitives_core::ParaId;
use orml_traits::currency::MultiCurrency;
use primitives::asset::AssetPair;
use sp_runtime::traits::AccountIdConversion;
use xcm_emulator::TestExt;

#[test]
fn transfer_from_relay_chain() {
	Basilisk::execute_with(|| {
		assert_ok!(basilisk_runtime::AssetRegistry::set_location(
			basilisk_runtime::Origin::root(),
			1,
			basilisk_runtime::AssetLocation(MultiLocation::parent())
		));
	});
	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(2000).into().into()),
			Box::new(
				Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				}
				.into()
				.into()
			),
			Box::new((Here, 3 * BSX).into()),
			0,
		));

		assert_eq!(
			kusama_runtime::Balances::free_balance(&ParaId::from(2000).into_account()),
			13 * BSX
		);
	});

	Basilisk::execute_with(|| {
		assert_eq!(
			basilisk_runtime::Tokens::free_balance(1, &AccountId::from(BOB)),
			3 * BSX
		);
	});
}

#[test]
fn transfer_to_relay_chain() {
	Basilisk::execute_with(|| {
		assert_ok!(basilisk_runtime::AssetRegistry::set_location(
			basilisk_runtime::Origin::root(),
			1,
			basilisk_runtime::AssetLocation(MultiLocation::parent())
		));

		assert_ok!(basilisk_runtime::XTokens::transfer(
			basilisk_runtime::Origin::signed(ALICE.into()),
			1,
			3 * BSX,
			Box::new(
				MultiLocation::new(
					1,
					X1(Junction::AccountId32 {
						id: BOB,
						network: NetworkId::Any,
					})
				)
				.into()
			),
			4_600_000_000
		));
		assert_eq!(
			basilisk_runtime::Tokens::free_balance(1, &AccountId::from(ALICE)),
			200 * BSX - 3 * BSX
		);
	});

	KusamaRelay::execute_with(|| {
		assert_eq!(
			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
			2999893333340 // 3 * BSX - fee
		);
	});
}

#[test]
fn transfer_from_hydra() {
	TestNet::reset();

	Basilisk::execute_with(|| {
		assert_ok!(basilisk_runtime::AssetRegistry::set_location(
			basilisk_runtime::Origin::root(),
			1,
			basilisk_runtime::AssetLocation(MultiLocation::new(1, X2(Parachain(3000), GeneralKey(vec![0, 0, 0, 0]))))
		));
	});

	Hydra::execute_with(|| {
		assert_ok!(basilisk_runtime::XTokens::transfer(
			basilisk_runtime::Origin::signed(ALICE.into()),
			0,
			3 * BSX,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(2000),
						Junction::AccountId32 {
							id: BOB,
							network: NetworkId::Any,
						}
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			basilisk_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200 * BSX - 3 * BSX
		);
	});

	Basilisk::execute_with(|| {
		assert_eq!(
			basilisk_runtime::Tokens::free_balance(1, &AccountId::from(BOB)),
			3 * BSX
		);
	});
}
#[test]
fn transfer_insufficient_amount_should_fail() {
	TestNet::reset();

	Basilisk::execute_with(|| {
		assert_ok!(basilisk_runtime::AssetRegistry::set_location(
			basilisk_runtime::Origin::root(),
			1,
			basilisk_runtime::AssetLocation(MultiLocation::new(1, X2(Parachain(3000), GeneralKey(vec![0, 0, 0, 0]))))
		));
	});

	Hydra::execute_with(|| {
		assert_noop!(
			basilisk_runtime::XTokens::transfer(
				basilisk_runtime::Origin::signed(ALICE.into()),
				0,
				1_000_000 - 1,
				Box::new(
					MultiLocation::new(
						1,
						X2(
							Junction::Parachain(2000),
							Junction::AccountId32 {
								id: BOB,
								network: NetworkId::Any,
							}
						)
					)
					.into()
				),
				399_600_000_000
			),
			orml_xtokens::Error::<basilisk_runtime::Runtime>::XcmExecutionFailed
		);
		assert_eq!(
			basilisk_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200000000000000
		);
	});

	Basilisk::execute_with(|| {
		// Xcm should fail therefore nothing should be deposit into beneficiary account
		assert_eq!(basilisk_runtime::Tokens::free_balance(1, &AccountId::from(BOB)), 0);
	});
}

#[test]
fn non_native_fee_payment_works() {
	TestNet::reset();

	Basilisk::execute_with(|| {

		let curr_0 = 0;
		let curr_1 = 1;

		// Alice wants to pay in the native currency
		assert_ok!(basilisk_runtime::Tokens::set_balance(
			basilisk_runtime::Origin::root(),
			ALICE.into(),
			curr_0,
			1_000_000_000_000_000_000,
			0,
		));

		assert_ok!(basilisk_runtime::Tokens::set_balance(
			basilisk_runtime::Origin::root(),
			BOB.into(),
			curr_0,
			1_000_000_000_000_000_000,
			0,
		));

		assert_ok!(basilisk_runtime::Tokens::transfer(
			basilisk_runtime::Origin::signed(ALICE.into()),
			BOB.into(),
			curr_0,
			1_000,
		));

		// Bob wants to pay in non-native currency (KSM = id 1)
	
		assert_ok!(basilisk_runtime::MultiTransactionPayment::add_currency(
			basilisk_runtime::Origin::root(),
			curr_1,
			1_000_000.into()
		));

		assert_ok!(basilisk_runtime::Tokens::set_balance(
			basilisk_runtime::Origin::root(),
			ALICE.into(),
			curr_1,
			1_000_000_000_000_000_000,
			0,
		));
		
		assert_ok!(basilisk_runtime::Tokens::set_balance(
			basilisk_runtime::Origin::root(),
			BOB.into(),
			curr_1,
			1_000_000_000_000_000_000,
			0,
		));
		
		assert_ok!(basilisk_runtime::MultiTransactionPayment::set_currency(
			basilisk_runtime::Origin::signed(BOB.into()),
			1,
		));

		assert_ok!(basilisk_runtime::Currencies::transfer(
			basilisk_runtime::Origin::signed(BOB.into()),
			ALICE.into(),
			curr_1,
			1,
		));

		assert_eq!(basilisk_runtime::Tokens::free_balance(1, &AccountId::from(BOB)), 537_323_499_999_999_999);
		
		assert_eq!(basilisk_runtime::Tokens::free_balance(0, &AccountId::from(ALICE)), 999_999_999_999_999_000);
		assert_eq!(basilisk_runtime::Tokens::free_balance(1, &AccountId::from(ALICE)), 1_000_000_000_000_000_001);

		let pair_account = basilisk_runtime::XYK::get_pair_id(AssetPair {
			asset_in: curr_0,
			asset_out: curr_1,
		});
		let share_token = basilisk_runtime::XYK::share_token(pair_account);

		assert_ok!(basilisk_runtime::XYK::create_pool(
			basilisk_runtime::Origin::signed(ALICE.into()),
			0,
			1,
			100_000,
			Price::from(10)
		));

		assert_ok!(basilisk_runtime::XYK::buy(
			basilisk_runtime::Origin::signed(ALICE.into()),
			0,
			1,
			66_666_666,
			1_000_000_000,
			false,
		));




	});
}