#![cfg_attr(not(feature = "std"), no_std)]

use core::{convert::TryInto};
use frame_support::{
	debug, decl_module, decl_storage, decl_event,
	dispatch
};
use frame_system::{
	self as system, ensure_signed,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer
	},
};
use sp_core::crypto::KeyTypeId;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocw8");

pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {

	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	type Call: From<Call<Self>>;

	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct AuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}


	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for AuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}


decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Numbers get(fn numbers): map hasher(blake2_128_concat) u64 => u64;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		NumberAppended(AccountId, u64, u64),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn save_number(origin, index: u64, number: u64) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			Numbers::insert(index, number);
			Self::deposit_event(RawEvent::NumberAppended(who, index, number));
			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers: {:?}", block_number);

			Self::submit_number(block_number);
		} 
	}
}

impl<T: Trait> Module<T> {
	fn submit_number(block_number: T::BlockNumber) {
		let index: u64 = block_number.try_into().ok().unwrap() as u64;
		let latest = if index > 0 {
			Self::numbers((index - 1) as u64)
		} else {
			0
		};

		let new: u64 = latest.saturating_add((index + 1).saturating_pow(2));

		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			debug::error!("No local account available");
			return;
		}

		let results = signer.send_signed_transaction(|_acct| {
			Call::save_number(index, new)
		});

		for (_acc, res) in &results {
			match res {
				Ok(()) => { debug::native::info!("off-chain tx succeeded: number: {}", new); }
				Err(_e) => { debug::error!("off-chain tx failed: number: {}", new); }
			};
		}
	}
}