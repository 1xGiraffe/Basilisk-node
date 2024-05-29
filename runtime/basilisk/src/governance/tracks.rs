// This file is part of Basilisk-node.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
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

//! Track configurations for governance.

use super::*;
use primitives::constants::{
	currency::UNITS,
	time::{HOURS, MINUTES},
};
const fn percent(x: i32) -> sp_arithmetic::FixedI64 {
	sp_arithmetic::FixedI64::from_rational(x as u128, 100)
}

use pallet_referenda::Curve;
const APP_LINEAR: Curve = Curve::make_linear(7, 7, percent(50), percent(100));
const APP_LINEAR_FLAT: Curve = Curve::make_linear(4, 7, percent(50), percent(100));
const APP_RECIP: Curve = Curve::make_reciprocal(1, 7, percent(80), percent(50), percent(100));
const SUP_LINEAR: Curve = Curve::make_linear(7, 7, percent(0), percent(50));
const SUP_RECIP: Curve = Curve::make_reciprocal(5, 7, percent(1), percent(0), percent(50));
const SUP_FAST_RECIP: Curve = Curve::make_reciprocal(3, 7, percent(1), percent(0), percent(50));
const SUP_WHITELISTED_CALLER: Curve = Curve::make_reciprocal(1, 28, percent(20), percent(1), percent(50));

const TRACKS_DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 9] = [
	(
		0,
		pallet_referenda::TrackInfo {
			name: "root",
			max_deciding: 1,
			decision_deposit: 100_000_000 * UNITS,
			prepare_period: 24 * HOURS,
			decision_period: 7 * DAYS,
			confirm_period: 24 * HOURS,
			min_enactment_period: 24 * HOURS,
			min_approval: APP_RECIP,
			min_support: SUP_LINEAR,
		},
	),
	(
		1,
		pallet_referenda::TrackInfo {
			name: "whitelisted_caller",
			max_deciding: 10,
			decision_deposit: 1_000_000 * UNITS,
			prepare_period: 10 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 10 * MINUTES,
			min_enactment_period: 10 * MINUTES,
			min_approval: APP_RECIP,
			min_support: SUP_WHITELISTED_CALLER,
		},
	),
	(
		2,
		pallet_referenda::TrackInfo {
			name: "referendum_canceller",
			max_deciding: 10,
			decision_deposit: 10_000_000 * UNITS,
			prepare_period: 60 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 24 * HOURS,
			min_enactment_period: 10 * MINUTES,
			min_approval: APP_LINEAR_FLAT,
			min_support: SUP_FAST_RECIP,
		},
	),
	(
		3,
		pallet_referenda::TrackInfo {
			name: "referendum_killer",
			max_deciding: 10,
			decision_deposit: 50_000_000 * UNITS,
			prepare_period: 60 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 3 * HOURS,
			min_enactment_period: 10 * MINUTES,
			min_approval: APP_LINEAR_FLAT,
			min_support: SUP_FAST_RECIP,
		},
	),
	(
		4,
		pallet_referenda::TrackInfo {
			name: "general_admin",
			max_deciding: 10,
			decision_deposit: 10_000_000 * UNITS,
			prepare_period: 60 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 3 * HOURS,
			min_enactment_period: 10 * MINUTES,
			min_approval: APP_RECIP,
			min_support: SUP_RECIP,
		},
	),
	(
		5,
		pallet_referenda::TrackInfo {
			name: "tech_fellowship_admin",
			max_deciding: 10,
			decision_deposit: 50_000_000 * UNITS,
			prepare_period: 60 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 24 * HOURS,
			min_enactment_period: 24 * HOURS,
			min_approval: APP_LINEAR_FLAT,
			min_support: SUP_RECIP,
		},
	),
	(
		6,
		pallet_referenda::TrackInfo {
			name: "treasurer",
			max_deciding: 10,
			decision_deposit: 50_000_000 * UNITS,
			prepare_period: 60 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 12 * HOURS,
			min_enactment_period: 10 * MINUTES,
			min_approval: APP_RECIP,
			min_support: SUP_LINEAR,
		},
	),
	(
		7,
		pallet_referenda::TrackInfo {
			name: "spender",
			max_deciding: 10,
			decision_deposit: 5_000_000 * UNITS,
			prepare_period: 60 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 3 * HOURS,
			min_enactment_period: 10 * MINUTES,
			min_approval: APP_LINEAR,
			min_support: SUP_RECIP,
		},
	),
	(
		8,
		pallet_referenda::TrackInfo {
			name: "tipper",
			max_deciding: 10,
			decision_deposit: 500_000 * UNITS,
			prepare_period: 60 * MINUTES,
			decision_period: 7 * DAYS,
			confirm_period: 3 * HOURS,
			min_enactment_period: 10 * MINUTES,
			min_approval: APP_LINEAR_FLAT,
			min_support: SUP_FAST_RECIP,
		},
	),
];

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
	type Id = u16;
	type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
	fn tracks() -> &'static [(Self::Id, pallet_referenda::TrackInfo<Balance, BlockNumber>)] {
		&TRACKS_DATA[..]
	}
	fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
		if let Ok(system_origin) = frame_system::RawOrigin::try_from(id.clone()) {
			match system_origin {
				frame_system::RawOrigin::Root => Ok(0),
				_ => Err(()),
			}
		} else if let Ok(custom_origin) = origins::Origin::try_from(id.clone()) {
			match custom_origin {
				origins::Origin::WhitelistedCaller => Ok(1),
				origins::Origin::ReferendumCanceller => Ok(2),
				origins::Origin::ReferendumKiller => Ok(3),
				origins::Origin::GeneralAdmin => Ok(4),
				origins::Origin::TechFellowshipAdmin => Ok(5),
				origins::Origin::Treasurer => Ok(6),
				origins::Origin::Spender => Ok(7),
				origins::Origin::Tipper => Ok(8),
			}
		} else {
			Err(())
		}
	}
}
pallet_referenda::impl_tracksinfo_get!(TracksInfo, Balance, BlockNumber);
