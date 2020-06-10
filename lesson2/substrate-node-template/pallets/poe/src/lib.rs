#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, StorageMap};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

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
	trait Store for Module<T: Trait> as poeMoudle {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ProofTransferto(AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		NoSuchProof,
		NotProofOwner,
		TooLongHash,//limit size for proof
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

		/// This fuction is for claim a proof 
		#[weight = 10_000]
		pub fn crate_proof(origin, proof: Vec<u8>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&proof),Error::<T>::ProofAlreadyExist);//ensure there are no same proof alreay exist
			ensure!(proof.len()<=64, Error::<T>::TooLongHash);//limit size for proof 
			let current_block = <system::Module<T>>::block_number();
			Proofs::<T>::insert(&proof, (&sender,current_block));
			Self::deposit_event(RawEvent::ClaimCreated(sender, proof));

			
			Ok(())
		}

		/// This function can revoke the proof you have claimed  
		#[weight = 10_000]
		pub fn revoke_claim(origin, proof:Vec<u8>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);//ensure the proof already exist 
			let (owner, _) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);//ensure only the owner of proof can revoke it
			Proofs::<T>::remove(&proof);
			Self::deposit_event(RawEvent::ClaimRevoked(sender,proof)); 
			Ok(())
		}

		///This function can transfer one proof from one account to another
		#[weight = 10_000]
		pub fn transfer_proof(origin, proof:Vec<u8>, new_owner:T::AccountId) -> dispatch::DispatchResult{
			
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			Proofs::<T>::insert(&proof, (&new_owner,<system::Module<T>>::block_number()));//cover the old proof cliam
			Self::deposit_event(RawEvent::ProofTransferto(new_owner,proof));

			
			
			Ok(())
		}




	}
}
