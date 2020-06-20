#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Currency, ExistenceRequirement, Get},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    // 附加题答案
    type MaxClaimLength: Get<u32>;

    type Currency: Currency<Self::AccountId>;

    type MaxNoteLength: Get<u32>;
}

type AccountIdOf<T> = <T as system::Trait>::AccountId;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as PoeModule {
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, Option<Vec<u8>>, T::BlockNumber);
        Prices get(fn prices): map hasher(blake2_128_concat) Vec<u8> => (BalanceOf<T>, T::BlockNumber);
        Claims get(fn claims): map hasher(blake2_128_concat) T::AccountId => Vec<Vec<u8>>;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Price = BalanceOf<T>,
    {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimTransferred(AccountId, Vec<u8>, AccountId),
        ClaimPriceSet(AccountId, Vec<u8>, Price),
        ClaimSold(AccountId, Vec<u8>, Price, AccountId),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        ProofTooLong,
        ClaimAlreadyOwned,
        ClaimNotForSale,
        BidPriceTooLow,
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
        pub fn create_claim(origin, claim: Vec<u8>, note: Option<Vec<u8>>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            match note.clone() {
                None => (),
                Some(text) => ensure!(T::MaxNoteLength::get() >= text.len() as u32, Error::<T>::NoteTooLong),
            }

            // 附加题答案
            ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

            Proofs::<T>::insert(&claim, (sender.clone(), note.clone(), system::Module::<T>::block_number()));

            if Claims::<T>::contains_key(&sender) {
                let mut vec = Claims::<T>::get(&sender);
                match vec.binary_search(&claim) {
                    // If the search succeeds, the caller is already a member, so just return
                    Ok(_) => (),
                    Err(index) => vec.insert(index, claim.clone()),
                };
                Claims::<T>::insert(&sender, vec);
            }
            else {
                let mut vec = Vec::<Vec<u8>>::new();
                vec.push(claim.clone());
                Claims::<T>::insert(&sender, vec);
            }

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

            Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _notes, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            let mut vec = Claims::<T>::get(&sender);
            match vec.binary_search(&claim) {
                Ok(index) => vec.remove(index),
                Err(_) => [0].to_vec(),
            };
            Claims::<T>::insert(&sender, vec);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        // 第二题答案
        #[weight = 0]
        pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, note, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            let dest = T::Lookup::lookup(dest)?;

            Proofs::<T>::insert(&claim, (dest.clone(), note, system::Module::<T>::block_number()));

            let mut vec = Claims::<T>::get(&sender);
            match vec.binary_search(&claim) {
                Ok(index) => vec.remove(index),
                Err(_) => [0].to_vec(),
            };
            Claims::<T>::insert(&sender, vec);

            if Claims::<T>::contains_key(&dest) {
                let mut vec = Claims::<T>::get(&dest);
                match vec.binary_search(&claim) {
                    // If the search succeeds, the caller is already a member, so just return
                    Ok(_) => (),
                    Err(index) => vec.insert(index, claim.clone()),
                };
                Claims::<T>::insert(&dest, vec);
            }
            else {
                let mut vec = Vec::<Vec<u8>>::new();
                vec.push(claim.clone());
                Claims::<T>::insert(&dest, vec);
            }

            Self::deposit_event(RawEvent::ClaimTransferred(sender, claim, dest));

            Ok(())
        }

        #[weight = 1000]
        pub fn set_claim_price(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _notes, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Prices::<T>::insert(&claim, (price, system::Module::<T>::block_number()));

            Self::deposit_event(RawEvent::ClaimPriceSet(sender, claim, price));

            Ok(())
        }

        #[weight = 1000]
        pub fn buy_claim(origin, claim: Vec<u8>, bid_price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, note, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner != sender, Error::<T>::ClaimAlreadyOwned);

            ensure!(Prices::<T>::contains_key(&claim), Error::<T>::ClaimNotForSale);

            let (price, _block_number) = Prices::<T>::get(&claim);

            ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);

            T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::KeepAlive)?;

            Proofs::<T>::insert(&claim, (sender.clone(), note, system::Module::<T>::block_number()));

            Prices::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimSold(owner, claim, price, sender));

            Ok(())
        }
    }
}
