// Tests to be written here

use crate::{Error, mock::*, CoinCap};
use frame_support::{assert_ok, assert_noop};

#[test]
fn test_coincap_deserialize() {
	let json = "{\"data\":{\"id\":\"ethereum\",\"rank\":\"2\",\"symbol\":\"ETH\",\"name\":\"Ethereum\",\"supply\":\"111669845.3740000000000000\",\"maxSupply\":null,\"marketCapUsd\":\"26877027658.2790761689676288\",\"volumeUsd24Hr\":\"1573167228.6739012923707169\",\"priceUsd\":\"240.6829486354499138\",\"changePercent24Hr\":\"1.3232863841667301\",\"vwap24Hr\":\"239.0067810371416018\"},\"timestamp\":1594197537505}";
	let coin = serde_json::from_str::<CoinCap>(json);
	if let Ok(coin_cap) = coin {
		assert_eq!(coin_cap.data.priceUsd, 240.68294863544992);
	} else {
		panic!("serialize error");
	}
}

#[test]
fn test_offchain() {
	new_test_ext().execute_with(|| {
		// Test offchain worker here
	});
}
