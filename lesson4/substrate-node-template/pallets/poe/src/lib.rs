#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
    traits::{Get, Currency, ExistenceRequirement::AllowDeath},
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

    type MaxCommentLength: Get<u32>;

    type Currency: Currency<Self::AccountId>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
		Prices get(fn prices): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;
		AccountDocs get(fn account_docs): map hasher(identity) T::AccountId => Vec<(Vec<u8>, T::Moment, Vec<u8>)>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where
	    AccountId = <T as system::Trait>::AccountId,
	    Balance = BalanceOf<T> {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimBidded(AccountId, Vec<u8>, Balance),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
		ClaimNotForBidding,
		InsufficientPrice,
		AccountBalanceNotEnough,
		DocumentNotExist,
		CommentTooLong,
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
		pub fn create_claim(origin, claim: Vec<u8>, comments: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);
			ensure!(T::MaxCommentLength::get() >= comments.len() as u32, Error::<T>::CommentTooLong);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
			Self::add_document(sender.clone(), claim.clone(), comments);

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

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

			Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number()));

			Ok(())
		}

		#[weight = 0]
		pub fn bid_claim(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Prices::<T>::insert(&claim, price);	
			Self::deposit_event(RawEvent::ClaimBidded(sender, claim, price));

			Ok(())
		}

		#[weight = 0]
		pub fn buy_claim(origin, claim: Vec<u8>, amount: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			ensure!(Prices::<T>::contains_key(&claim), Error::<T>::ClaimNotForBidding);
            ensure!(amount >= Prices::<T>::get(&claim), Error::<T>::InsufficientPrice);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            T::Currency::transfer(&sender, &owner, amount, AllowDeath)
                .map_err(|_| Error::<T>::AccountBalanceNotEnough)?;

			Proofs::<T>::insert(&claim, (sender, system::Module::<T>::block_number()));
            // set price automatically after bought the claim
            Prices::<T>::insert(&claim, amount);

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
    pub fn add_document(sender: T::AccountId, claim: Vec<u8>, comments: Vec<u8>) {
        if AccountDocs::<T>::contains_key(sender.clone()) {
            let mut docs = AccountDocs::<T>::get(sender.clone());
            docs.push((claim, <timestamp::Module<T>>::get(), comments));
            AccountDocs::<T>::insert(sender.clone(), docs);
        } else {
            AccountDocs::<T>::insert(sender.clone(), vec![(claim, <timestamp::Module<T>>::get(), comments)]);
        }
    }
}
