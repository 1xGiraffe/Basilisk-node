use super::*;
use crate::{self as pallet_auctions};
use frame_support::{BoundedVec, parameter_types, traits::Everything, PalletId};
use frame_system as system;
use primitives::{
	nft::{ClassType, NftPermissions},
	ReserveIdentifier,
};
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureRoot;

mod auction {
	// Re-export needed for `impl_outer_event!`.
	pub use super::super::*;
}

pub use crate::mock::Event as TestEvent;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = AccountId32;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Auctions: pallet_auctions::{Pallet, Call, Storage, Event<T>},
		Nft: pallet_nft::{Pallet, Call, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
	}
);

/// Balance of an account.
pub type Balance = u128;

pub const ALICE: AccountId = AccountId::new([1u8; 32]);
pub const BOB: AccountId = AccountId::new([2u8; 32]);
pub const CHARLIE: AccountId = AccountId::new([3u8; 32]);
pub const DAVE: AccountId = AccountId::new([4u8; 32]);
pub const EVE: AccountId = AccountId::new([5u8; 32]);
pub const BSX: Balance = 100_000_000_000;
// Classes reserved up to 999 so available ids starting from 1000
pub const NFT_CLASS_ID_1: u32 = 1001;
pub const NFT_INSTANCE_ID_1: u32 = 1;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

parameter_types! {
	pub ReserveClassIdUpTo: u32 = 999;
}

impl pallet_nft::Config for Test {
	type Currency = Balances;
	type Event = Event;
	type WeightInfo = pallet_nft::weights::BasiliskWeight<Test>;
	type NftClassId = u32;
	type NftInstanceId = u32;
	type ProtocolOrigin = EnsureRoot<AccountId>;
	type ClassType = ClassType;
	type Permissions = NftPermissions;
	type ReserveClassIdUpTo = ReserveClassIdUpTo;
}

parameter_types! {
	pub const AuctionsStringLimit: u32 = 128;
	pub const BidAddBlocks: u32 = 10;
	pub const BidStepPerc: u32 = 10;
	pub const MinAuctionDuration: u32 = 10;
	pub const BidMinAmount: u32 = 1;
	pub const AuctionsPalletId: PalletId = PalletId(*b"auctions");
	pub const CandleDefaultDuration: u32 = 99_356;
	pub const CandleDefaultClosingPeriodDuration: u32 = 72_000;
	pub const CandleDefaultClosingRangesCount: u32 = 100;
}

pub struct TestRandomness<T>(sp_std::marker::PhantomData<T>);

impl<Output: codec::Decode + Default, T> frame_support::traits::Randomness<Output, T::BlockNumber> for TestRandomness<T>
where
	T: frame_system::Config,
{
	fn random(subject: &[u8]) -> (Output, T::BlockNumber) {
		use sp_runtime::traits::TrailingZeroInput;

		(
			Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
			frame_system::Pallet::<T>::block_number(),
		)
	}
}

impl pallet_auctions::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type AuctionId = u64;
	type Currency = Balances;
	type Randomness = TestRandomness<Test>;
	type WeightInfo = pallet_auctions::weights::BasiliskWeight<Test>;
	type AuctionsStringLimit = AuctionsStringLimit;
	type BidAddBlocks = BidAddBlocks;
	type BidStepPerc = BidStepPerc;
	type MinAuctionDuration = MinAuctionDuration;
	type BidMinAmount = BidMinAmount;
	type PalletId = AuctionsPalletId;
	type CandleDefaultDuration = CandleDefaultDuration;
	type CandleDefaultClosingPeriodDuration = CandleDefaultClosingPeriodDuration;
	type CandleDefaultClosingRangesCount = CandleDefaultClosingRangesCount;
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

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ClassDeposit: Balance = 9_900 * BSX; // 1 UNIT deposit to create asset class
	pub const InstanceDeposit: Balance = 100 * BSX; // 1/100 UNIT deposit to create asset instance
	pub const KeyLimit: u32 = 32;	// Max 32 bytes per key
	pub const ValueLimit: u32 = 64;	// Max 64 bytes per value
	pub const UniquesMetadataDepositBase: Balance = 100 * BSX;
	pub const AttributeDepositBase: Balance = 10 * BSX;
	pub const DepositPerByte: Balance = BSX;
	pub const UniquesStringLimit: u32 = 128;
}

impl pallet_uniques::Config for Test {
	type Event = Event;
	type ClassId = u32;
	type InstanceId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
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

pub struct ExtBuilder;
impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(ALICE, 40_000 * BSX),
				(BOB, 2_000 * BSX),
				(CHARLIE, 4_000 * BSX),
				(DAVE, 4_000 * BSX),
				(EVE, 4_000 * BSX),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

fn last_event() -> Event {
	frame_system::Pallet::<Test>::events()
		.pop()
		.expect("An event expected")
		.event
}

pub fn expect_event<E: Into<TestEvent>>(e: E) {
	assert_eq!(last_event(), e.into());
}

pub fn set_block_number<T: frame_system::Config<BlockNumber = u64>>(n: u64) {
	frame_system::Pallet::<T>::set_block_number(n);
}

pub fn to_bounded_name(name: Vec<u8>) -> Result<BoundedVec<u8, AuctionsStringLimit>, Error<Test>> {
  name.try_into().map_err(|_| Error::<Test>::TooLong)
}

pub fn valid_common_auction_data() -> CommonAuctionData<Test> {
	CommonAuctionData {
		name: to_bounded_name(b"Auction 0".to_vec()).unwrap(),
		reserve_price: None,
		last_bid: None,
		start: 10u64,
		end: 21u64,
		closed: false,
		owner: ALICE,
		token: (NFT_CLASS_ID_1, NFT_INSTANCE_ID_1),
		next_bid_min: 1,
	}
}

/// English auction tests
pub fn english_auction_object(common_data: CommonAuctionData<Test>, specific_data: EnglishAuctionData) -> Auction<Test> {
	let auction_data = EnglishAuction {
		common_data,
		specific_data,
	};

	Auction::English(auction_data)
}

pub fn valid_english_specific_data() -> EnglishAuctionData {
	EnglishAuctionData {}
}

/// TopUp auction tests
pub fn topup_auction_object(common_data: CommonAuctionData<Test>, specific_data: TopUpAuctionData) -> Auction<Test> {
	let auction_data = TopUpAuction {
		common_data,
		specific_data,
	};

	Auction::TopUp(auction_data)
}

pub fn valid_topup_specific_data() -> TopUpAuctionData {
	TopUpAuctionData {}
}

pub fn bid_object(amount: BalanceOf<Test>, block_number: <Test as frame_system::Config>::BlockNumber) -> Bid<Test> {
	Bid { amount, block_number }
}

pub fn get_auction_subaccount_id(auction_id: <Test as pallet::Config>::AuctionId) -> AccountId32 {
	<Test as pallet::Config>::PalletId::get().into_sub_account(("ac", auction_id))
}

/// Candle auction tests
pub fn candle_auction_object(common_data: CommonAuctionData<Test>, specific_data: CandleAuctionData<Test>) -> Auction<Test> {
	let auction_data = CandleAuction {
		common_data,
		specific_data,
	};

	Auction::Candle(auction_data)
}

pub fn valid_candle_common_auction_data() -> CommonAuctionData<Test> {
	CommonAuctionData {
		name: to_bounded_name(b"Auction 0".to_vec()).unwrap(),
		reserve_price: None,
		last_bid: None,
		start: 10u64,
		end: 99_366u64,
		closed: false,
		owner: ALICE,
		token: (NFT_CLASS_ID_1, NFT_INSTANCE_ID_1),
		next_bid_min: 1,
	}
}

pub fn valid_candle_specific_data() -> CandleAuctionData<Test> {
	CandleAuctionData {
		closing_start: 27_366,
		winner: None,
		winning_closing_range: None
	}
}
