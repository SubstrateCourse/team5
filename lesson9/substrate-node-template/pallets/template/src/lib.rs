#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_std::str;
use codec::{Decode, Encode};
use sp_runtime::{offchain::storage::StorageValueRef, offchain as rt_offchain};
use core::convert::TryInto;
use alt_serde::{Deserialize, Deserializer};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const COINCAP_API_URL: &[u8] = b"https://api.coincap.io/v2/assets/ethereum";
pub const CRYPTOCOMPARE_API_URL: &[u8] = b"https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD";

#[serde(crate = "alt_serde")]
#[derive(Debug, Deserialize)]
pub struct CoinCap {
    data: CoinCapData,
}

#[serde(crate = "alt_serde")]
#[derive(Debug, Deserialize)]
pub struct CoinCapData {
    #[serde(deserialize_with = "de_string_to_f64")]
    priceUsd: f64,
}

#[serde(crate = "alt_serde")]
#[derive(Debug, Deserialize)]
pub struct CryptoCompare {
    USD: f64,
}

pub fn de_string_to_f64<'de, D>(de: D) -> Result<f64, D::Error>
    where D: Deserializer<'de> {
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.parse::<f64>().unwrap())
}

#[derive(Debug, Encode, Decode)]
struct PriceList {
    /// the price stored as 1/1000$
    prices: Vec<u64>,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(fn something): Option<u32>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
		/// Storage was fetched by other thread
		AlreadyFetched,
		/// Parsing URL failed
		URLParsingError,
		/// Fetching http failed
		HttpFetchingError,
		/// Parsing response failed
		ResponseParseError
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn save_number(origin, number: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			/*******
			 * 学员们在这里追加逻辑
			 *******/

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			/*******
			 * 学员们在这里追加逻辑
			 *******/
            let result = Self::price_trending(block_number);
            if let Err(e) = result {
	            debug::error!("Error: {:?}", e);
            }
		}
	}
}

impl<T: Trait> Module<T> {
    fn price_trending(block_number: T::BlockNumber) -> Result<(), Error::<T>> {
        let height = block_number.try_into().ok().unwrap() as u64;

        // fetch every 5 blocks (about 30 seconds)
        if height % 5 == 0 {
            let mut price_total = Self::fetch_coincap_price()?;
            price_total += Self::fetch_cryptocompare_price()?;

            Self::append_price(price_total/2)?;
        }
        Ok(())
    }

    fn fetch_coincap_price() -> Result<u64, Error::<T>> {
        let content_bytes = Self::fetch_api_info(COINCAP_API_URL)?;
        let content = str::from_utf8(&content_bytes)
            .map_err(|_| Error::<T>::URLParsingError)?;
        debug::info!("coincap response content: {:?}", content);

        let coin_cap = serde_json::from_str::<CoinCap>(content)
            .map_err(|_| Error::<T>::ResponseParseError)?;
        debug::info!("coincap object: {:?}", coin_cap);

        Ok((coin_cap.data.priceUsd as u64) * 1000)
    }

    fn fetch_cryptocompare_price() -> Result<u64, Error::<T>> {
        let content_bytes = Self::fetch_api_info(CRYPTOCOMPARE_API_URL)?;
        let content = str::from_utf8(&content_bytes)
            .map_err(|_| Error::<T>::URLParsingError)?;
        debug::info!("cryptocompare response content: {:?}", content);

        let crypto_compare = serde_json::from_str::<CryptoCompare>(content)
            .map_err(|_| Error::<T>::ResponseParseError)?;
        debug::info!("cryptocompare object: {:?}", crypto_compare);

        Ok((crypto_compare.USD as u64) * 1000)
    }

    fn fetch_api_info(url_bytes: &[u8]) -> Result<Vec<u8>, Error::<T>> {
        let url = str::from_utf8(url_bytes)
            .map_err(|_| Error::<T>::URLParsingError)?;

        let request = rt_offchain::http::Request::get(url);
        let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(5000));

        let pending = request
            .deadline(timeout)
            .send()
            .map_err(|_| Error::<T>::HttpFetchingError)?;

        let response = pending
            .try_wait(timeout)
            .map_err(|_| Error::<T>::HttpFetchingError)?
            .map_err(|_| Error::<T>::HttpFetchingError)?;

        if response.code != 200 {
            debug::error!("Unexpected http request status code: {}", response.code);
            return Err(Error::<T>::HttpFetchingError);
        }
        Ok(response.body().collect::<Vec<u8>>())
    }

    fn append_price(price: u64) -> Result<(), Error::<T>> {
        let price_store = StorageValueRef::persistent(b"pallet_template::prices_store");
        let price_lock = StorageValueRef::persistent(b"pallet_template::prices_lock");

        let res: Result<Result<bool, bool>, Error<T>> = price_lock.mutate(|s: Option<Option<bool>>| {
            match s {
                // `s` can be one of the following:
                //   `None`: the lock has never been set. Treated as the lock is free
                //   `Some(None)`: unexpected case, treated it as AlreadyFetch
                //   `Some(Some(false))`: the lock is free
                //   `Some(Some(true))`: the lock is held
                None | Some(Some(false)) => Ok(true),
                _ => Err(Error::<T>::AlreadyFetched)
            }
        });

        if let Ok(Ok(true)) = res {
            let mut price_list = PriceList { prices: vec![] };
            if let Some(Some(prices)) = price_store.get::<PriceList>() {
                price_list.prices = prices.prices;
            }
            price_list.prices.push(price);
            debug::info!("current price list: {:?}", price_list);

            price_store.set(&price_list);
            price_lock.set(&false);
        }

        Ok(())
    }
}
