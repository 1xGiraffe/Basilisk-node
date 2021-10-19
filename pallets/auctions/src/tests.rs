use super::*;
use crate::{mock::*};
use frame_support::{assert_noop, assert_ok};

pub type AuctionsModule = Pallet<Test>;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

#[test]
fn can_create_auction() {
  let valid_auction_info = AuctionInfo {
    name: "Auction 1".as_bytes().to_vec(),
    last_bid: None,
    start: 10u64,
    end: 21u64,
    owner: ALICE,
    auction_type: AuctionType::English,
    token: (NFT_CLASS_ID, 0u16.into()),
    minimal_bid: 55,
  };

  ExtBuilder::default().build().execute_with(|| {
    assert_ok!(Nft::create_class(Origin::signed(ALICE), NFT_CLASS_ID, ALICE, bvec![0]));
    assert_ok!(Nft::mint(Origin::signed(ALICE), NFT_CLASS_ID, 0u16.into(), ALICE, 10u8, bvec![0]));
    
    // start before current block
    let mut auction_info = valid_auction_info.clone();
    auction_info.start = 0u64;
    assert_noop!(
      AuctionsModule::create_auction(Origin::signed(ALICE), auction_info),
      Error::<Test>::AuctionStartTimeAlreadyPassed
    );

    // end is zero
    auction_info = valid_auction_info.clone();
    auction_info.end = 0u64;
    assert_noop!(
      AuctionsModule::create_auction(Origin::signed(ALICE), auction_info),
      Error::<Test>::InvalidTimeConfiguration
    );

    // duration too short
    auction_info = valid_auction_info.clone();
    auction_info.end = 20u64;
    assert_noop!(
      AuctionsModule::create_auction(Origin::signed(ALICE), auction_info),
      Error::<Test>::InvalidTimeConfiguration
    );

    // auction name empty
    auction_info = valid_auction_info.clone();
    auction_info.name = "".as_bytes().to_vec();
    assert_noop!(
      AuctionsModule::create_auction(Origin::signed(ALICE), auction_info),
      Error::<Test>::EmptyAuctionName
    );

    // Caller isn't owner
    auction_info = valid_auction_info.clone();
    auction_info.owner = BOB;
    assert_noop!(
      AuctionsModule::create_auction(Origin::signed(ALICE), auction_info),
      Error::<Test>::NotATokenOwner
    );

    // TODO add test for Error::<T>::TokenFrozen

    // happy path
    assert_ok!(AuctionsModule::create_auction(
      Origin::signed(ALICE),
      valid_auction_info.clone(),
    ));

    let auction = AuctionsModule::auctions(0).unwrap();
    assert_eq!(String::from_utf8(auction.name).unwrap(), "Auction 1");
    assert_eq!(auction.last_bid, None);
    assert_eq!(auction.start, 10u64);
    assert_eq!(auction.end, 21u64);
    assert_eq!(auction.owner, ALICE);
    assert_eq!(auction.auction_type, AuctionType::English);
    assert_eq!(auction.token, (NFT_CLASS_ID, 0u16.into()));
    assert_eq!(auction.minimal_bid, 55);

    assert_eq!(AuctionsModule::auction_owner_by_id(0), ALICE);
    assert_eq!(AuctionsModule::auction_end_time(21u64, 0).unwrap(), ());

    // TODO add test for token freeze

    expect_event(crate::Event::<Test>::AuctionCreated(ALICE, 0));
  });
}

#[test]
fn can_delete_auction() {
  ExtBuilder::default().build().execute_with(|| {
    assert_ok!(Nft::create_class(Origin::signed(ALICE), NFT_CLASS_ID, ALICE, bvec![0]));
    assert_ok!(Nft::mint(Origin::signed(ALICE), NFT_CLASS_ID, 0u16.into(), ALICE, 10u8, bvec![0]));
    let auction_info = AuctionInfo {
      name: "Auction 1".as_bytes().to_vec(),
      last_bid: None,
      start: 10u64,
      end: 21u64,
      owner: ALICE,
      auction_type: AuctionType::English,
      token: (NFT_CLASS_ID, 0u16.into()),
      minimal_bid: 55,
    };

    // Error AuctionNotExist when auction is not found
    assert_noop!(
      AuctionsModule::delete_auction(Origin::signed(ALICE), 0),
      Error::<Test>::AuctionNotExist,
    );
    
    assert_ok!(
      AuctionsModule::create_auction(Origin::signed(ALICE), auction_info)
    );
    
    // TODO weird panic
    // Error NotAuctionOwner when caller is not owner
    // assert_noop!(
    //   AuctionsModule::delete_auction(Origin::signed(BOB), 0),
    //   Error::<Test>::NotAuctionOwner,
    // );

    // TODO test error AuctionAlreadyStarted

    // TODO test for pallet_uniqueness

    // Happy path
    assert_ok!(
      AuctionsModule::delete_auction(Origin::signed(ALICE), 0)
    );

    assert_eq!(AuctionsModule::auctions(0), None);

    // TODO how to test this case?
    // assert_eq!(AuctionsModule::auction_owner_by_id(0), None);

    expect_event(crate::Event::<Test>::AuctionRemoved(0));
  });

}

#[test]
fn can_update_auction() {
  ExtBuilder::default().build().execute_with(|| {
    assert_ok!(Nft::create_class(Origin::signed(ALICE), NFT_CLASS_ID, ALICE, bvec![0]));
    assert_ok!(Nft::mint(Origin::signed(ALICE), NFT_CLASS_ID, 0u16.into(), ALICE, 10u8, bvec![0]));
    let auction_info = AuctionInfo {
      name: "Auction 1".as_bytes().to_vec(),
      last_bid: None,
      start: 10u64,
      end: 21u64,
      owner: ALICE,
      auction_type: AuctionType::English,
      token: (NFT_CLASS_ID, 0u16.into()),
      minimal_bid: 55,
    };

    assert_ok!(AuctionsModule::create_auction(Origin::signed(ALICE), auction_info));

    let update_auction_info = AuctionInfo {
      name: "Auction renamed".as_bytes().to_vec(),
      last_bid: None,
      start: 10u64,
      end: 21u64,
      owner: ALICE,
      auction_type: AuctionType::English,
      token: (NFT_CLASS_ID, 0u16.into()),
      minimal_bid: 55,
    };

    // Error NotAuctionOwner when caller is not owner
    assert_noop!(
      AuctionsModule::update_auction(Origin::signed(BOB), 0, update_auction_info.clone()),
      Error::<Test>::NotAuctionOwner,
    );

    // Happy path
    assert_ok!(AuctionsModule::update_auction(Origin::signed(ALICE), 0, update_auction_info.clone()));

    let auction = AuctionsModule::auctions(0).unwrap();
    assert_eq!(String::from_utf8(auction.name).unwrap(), "Auction renamed");
  });
}
