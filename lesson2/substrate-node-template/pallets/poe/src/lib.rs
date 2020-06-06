#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as PoeModule {
        /// The first field tells the owner of the claim.
        /// The owner defaults to be the creater and can be mutated if claim is transfered
        /// The second field tells the block in which the last transcation was made (create/transfer)
        Claims: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Event emitted when a claim has been created.
        ClaimCreated(AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner.
        ClaimRevoked(AccountId, Vec<u8>),
        /// Event emitted when a claim is transfered from the owner to a receiver.
        ClaimTransfered(AccountId, AccountId, Vec<u8>),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This claim has already been created
        ClaimAlreadyCreated,
        /// The claim has not been created, so it can't be revoked/transfered
        NoSuchClaim,
        /// The claim is own by another account, so caller can't revoke/transfer it
        NotClaimOwner,
        /// The hash of the claim's content is out of bound
        ContentHashOutOfBound,
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        type Error = Error<T>;

        // A default function for depositing events
        fn deposit_event() = default;

        #[weight = 10_000]
        fn create_claim(origin, claim: Vec<u8>) {
            // For bonus question:
            // The hash of claim's content should not be longer than two [u8]
            // i.e. 0xabcd is valid, but 0xabcde is out of bound
            ensure!(claim.len() <= 2, Error::<T>::ContentHashOutOfBound);

            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;

            // Verify that the specified claim has not been claimed yet or error with the message
            ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::ClaimAlreadyCreated);

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();

            // Store the claim with the sender and the current block number
            Claims::<T>::insert(&claim, (&sender, current_block));

            // Emit an event that the claim was created
            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
        }

        #[weight = 10_000]
        fn revoke_claim(origin, claim: Vec<u8>) {
            // Determine who is calling the function
            let sender = ensure_signed(origin)?;

            // Verify that the specified claim has been claimed
            ensure!(Claims::<T>::contains_key(&claim), Error::<T>::NoSuchClaim);

            // Get owner of the claim
            let (owner, _) = Claims::<T>::get(&claim);

            // Verify that sender of the current call is the claim owner
            ensure!(sender == owner, Error::<T>::NotClaimOwner);

            // Remove claim from storage
            Claims::<T>::remove(&claim);

            // Emit an event that the claim was erased
            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
        }

        #[weight = 10_000]
        fn transfer_claim(origin, receiver: T::AccountId, claim: Vec<u8>) {
            // Determine who is calling the function
            let sender = ensure_signed(origin)?;

            // Verify that the specified claim has been claimed
            ensure!(Claims::<T>::contains_key(&claim), Error::<T>::NoSuchClaim);

            // Get owner of the claim
            let (owner, _) = Claims::<T>::get(&claim);

            // Verify that sender of the current call is the claim owner
            ensure!(sender == owner, Error::<T>::NotClaimOwner);

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();

            // Store the claim with the receiver and the current block number
            Claims::<T>::insert(&claim, (&receiver, current_block));

            // Emit an event that the claim was erased
            Self::deposit_event(RawEvent::ClaimTransfered(sender, receiver, claim));
        }
    }
}
