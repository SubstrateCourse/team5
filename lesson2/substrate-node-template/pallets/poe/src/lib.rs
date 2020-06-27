#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports


/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{self as system, ensure_signed};
use frame_support::inherent::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber)
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTranslated(AccountId, AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAllreadyExist,
		ProofNoFound,
		NotClaimOwner,
		ClaimTooLong,
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
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let max: u8 = 10;
			println!("{}", claim.len());
			ensure!(claim.len() < 10, Error::<T>::ClaimTooLong);
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAllreadyExist);
			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number() ));
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}
		#[weight = 10_000]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNoFound);
			let (owner, block_number) = Proofs::<T>::get(&claim);
			ensure!(sender == owner, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
			Ok(())
		}
		#[weight = 10_000]
		pub fn translate_claim(origin, claim: Vec<u8>, reciver: T::AccountId) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			// let reciverOver = ensure_signed(reciver)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNoFound);
			let (owner, block_number) = Proofs::<T>::get(&claim);
			ensure!(sender == owner, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim);
			Proofs::<T>::insert(&claim, (reciver.clone(), system::Module::<T>::block_number() ));
			Self::deposit_event(RawEvent::ClaimTranslated(sender, reciver, claim));
			Ok(())
		}
	}
}
