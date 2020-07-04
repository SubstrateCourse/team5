#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	traits::{Get, Currency, ExistenceRequirement},
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// The pallet's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// 附加题答案
	type MaxClaimLength: Get<u32>;
	type MaxNoteLength: Get<u32>;

	type Currency: Currency<Self::AccountId>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
		Price get(fn price): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;

		// hw4
		ProofsWithNote get(fn proofs_with_note): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, T::Moment, Vec<u8>);
		AccountProofs get(fn account_proofs): map hasher(identity) T::AccountId => Vec<Vec<u8>>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
		Moment = <T as timestamp::Trait>::Moment
	{
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		/// the format is (from, to, claim)
		ClaimTransfered(AccountId, AccountId, Vec<u8>),
		/// the format is (creator, claim, timestamp, note)
		ClaimWithNoteCreated(AccountId, Vec<u8>, Moment, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
		BuySelfClaim,
		ClaimNotForSell,
		BidPriceNotEnough,
		AccountBalanceNotEnough,
		NoteTooLong,
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

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		#[weight = 0]
		pub fn create_claim_with_note(origin, claim: Vec<u8>, note: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);
			ensure!(T::MaxNoteLength::get() >= note.len() as u32, Error::<T>::NoteTooLong);

			let now = <timestamp::Module<T>>::now();

			ProofsWithNote::<T>::insert(&claim, (
				sender.clone(),
				system::Module::<T>::block_number(),
				now,
				note.clone()));
            if AccountProofs::<T>::contains_key(&sender) {
				let mut proofs = AccountProofs::<T>::get(&sender);
				if let Err(index) = proofs.binary_search(&claim) {
					proofs.insert(index, claim.clone());
				}
				AccountProofs::<T>::insert(&sender, proofs);
			} else {
				let proofs = vec![claim.clone()];
				AccountProofs::<T>::insert(&sender, proofs);
			}

			Self::deposit_event(RawEvent::ClaimWithNoteCreated(sender, claim, now, note));

			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

		// 第二题答案
		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			let dest = T::Lookup::lookup(dest)?;

			Proofs::<T>::insert(&claim, (dest.clone(), system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimTransfered(sender, dest, claim));

			Ok(())
		}


		#[weight = 0]
		pub fn set_claim_price(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Price::<T>::insert(&claim, price);

			Ok(())
		}

		#[weight = 0]
		pub fn buy_claim(origin, claim: Vec<u8>, bid_price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner != sender, Error::<T>::BuySelfClaim);

			ensure!(Price::<T>::contains_key(&claim), Error::<T>::ClaimNotForSell);

			ensure!(Price::<T>::get(&claim) <= bid_price, Error::<T>::BidPriceNotEnough);

			T::Currency::transfer(&sender, &owner, bid_price, ExistenceRequirement::AllowDeath)
				.map_err(|_| Error::<T>::AccountBalanceNotEnough)?;

			// change owner and clear price
			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
			Price::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimTransfered(owner, sender, claim));

			Ok(())

		}
	}
}
