#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]

// 1. Imports
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap, 
    traits::{Get, Currency, ExistenceRequirement::AllowDeath},
};
use system::{self as system, ensure_signed};
use sp_std::{
    cmp::{Eq, PartialEq},
    vec::Vec,
};
use sp_runtime::{
    RuntimeDebug,
    traits::StaticLookup,
};
use codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

// 2. Pallet Configuration
// proofinfo
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct ProofInfo<AccountId,BlockNumber,Balance> {
    pub claimer: AccountId,
    pub owner: AccountId,
    pub price: Balance,
    pub claim_block: BlockNumber,
}

pub trait Trait: system::Trait { 
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    
    type MaxClaimLength: Get<u32>;

    type Currency: Currency<Self::AccountId>;
}

// 3. Pallet Storage Items
decl_storage! { 
    trait Store for Module<T: Trait> as Poe {
        /// The storage item for our proofs.
        /// It maps a proof to the user who made the claim and when they made it.
        pub Proofs: map hasher(blake2_128_concat) Vec<u8> => Option<ProofInfo<T::AccountId, T::BlockNumber, BalanceOf<T>>>;
        
    }
 }

// 4. Pallet Events
decl_event! { 
    pub enum Event<T> where 
    AccountId = <T as system::Trait>::AccountId,
    Balance = BalanceOf<T>  
    {
        /// Event emitted when a proof has been claimed.
        ClaimCreated(AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner.
        ClaimRevoked(AccountId, Vec<u8>),
        /// Event emitted when a claim is transferred by the owner.
        ClaimTransfered(AccountId, AccountId, Vec<u8>),
        /// Event emitted when a claim is bought.
        ClaimBought(AccountId, Vec<u8>, Balance),
    }
}

// 5. Pallet Errors
decl_error! { 
    pub enum Error for Module<T: Trait> {
        /// This proof has already been claimed
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it
        NotProofOwner,
        /// caller can't transfer the proof to self
        CannotTransferProofToSelf,
        /// The proof is claimed is too long, so caller can't revoke it
        ProofTooLong,

        InsufficientPrice,

        AccountBalanceNotEnough,
    }
 }

// 6. Callable Pallet Functions
decl_module! { 
     /// The module declaration.
     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // A default function for depositing events
        fn deposit_event() = default;

        /// Allow a user to claim ownership of an unclaimed proof
        #[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>, price: BalanceOf<T>) {
            // we limit proof length should not longer than MaxClaimLength
            ensure!(T::MaxClaimLength::get() >= proof.len() as u32, Error::<T>::ProofTooLong);
            
            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not been claimed yet or error with the message
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();

            // new proof info
            let new_proof_info = ProofInfo {
                claimer: sender.clone(),
                owner: sender.clone(),
                price: price.clone(),
                claim_block: current_block,
            };

            // Store the proof_info with the sender and the current block number
            Proofs::<T>::insert(&proof, new_proof_info);

            // Emit an event that the claim was created
            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }

        /// Allow the owner to revoke their claim
        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {
            // Determine who is calling the function
            let sender = ensure_signed(origin)?;

            // Get owner of the claim
            let proof_info = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            // Verify that sender of the current call is the claim owner
            ensure!(sender == proof_info.owner, Error::<T>::NotProofOwner);

            // Remove claim from storage
            Proofs::<T>::remove(&proof);
            
            // Emit an event that the claim was erased
            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }

        /// Allow the owner to transfer their claim
        #[weight = 0]
		fn transfer_claim(origin, proof: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) {
			let sender = ensure_signed(origin)?;
            
            // Get owner of the claim
            let mut proof_info = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            // Verify that sender of the current call is the claim owner
			ensure!(sender == proof_info.owner, Error::<T>::NotProofOwner);

            
            // change owner
            // Store the proof_info with the new owner
            let dest = T::Lookup::lookup(dest)?;

            // Verify that receiver of the current call is not the claim owner
            ensure!(proof_info.owner != dest, Error::<T>::CannotTransferProofToSelf);

            proof_info.owner = dest.clone();
            Proofs::<T>::insert(&proof, proof_info);

            // Emit an event that the claim was transferred
			Self::deposit_event(RawEvent::ClaimTransfered(sender, dest, proof));
        }
        
        #[weight = 0]
		pub fn buy_claim(origin, proof: Vec<u8>, amount: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;
            
            let mut proof_info = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            ensure!(amount >= proof_info.price, Error::<T>::InsufficientPrice);

            T::Currency::transfer(&sender, &proof_info.owner, amount, AllowDeath)
                .map_err(|_| Error::<T>::AccountBalanceNotEnough)?;

            proof_info.owner = sender.clone();
            proof_info.price = amount.clone();
            Proofs::<T>::insert(&proof, &proof_info);

            Self::deposit_event(RawEvent::ClaimBought(sender, proof, proof_info.price));
            
		}
    }
 }