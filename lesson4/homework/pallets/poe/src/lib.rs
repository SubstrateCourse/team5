#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]

// 1. Imports
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap, 
    traits::{Get, Currency, ExistenceRequirement::AllowDeath, Time},
};
use system::{self as system, ensure_signed};
use sp_std::{
    cmp::{Eq, PartialEq},
    vec::Vec,
};
use sp_runtime::{
    RuntimeDebug,
    traits::StaticLookup,
    DispatchResult,
};
use codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type MomentOf<T> = <<T as Trait>::Time as Time>::Moment;

// 2. Pallet Configuration
// proofinfo
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct ProofInfo<AccountId, BlockNumber, Balance, Moment> {
    pub claimer: AccountId,
    pub owner: AccountId,
    pub comment: Vec<u8>,
    pub price: Balance,
    pub created_on: Moment,
    pub claim_block: BlockNumber,
}

pub trait Trait: system::Trait { 

    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type Time: Time;
    
    type MaxClaimLength: Get<u32>;

    type MaxCommentLength: Get<u32>;

    type Currency: Currency<Self::AccountId>;
}

// 3. Pallet Storage Items
decl_storage! { 
    trait Store for Module<T: Trait> as Poe {

        pub Proofs: map hasher(blake2_128_concat) Vec<u8> => Option<ProofInfo<T::AccountId, T::BlockNumber, BalanceOf<T>, MomentOf<T>>>;

        pub OwnedProofs get(fn owned_proofs): map hasher(blake2_128_concat) T::AccountId => Vec<Vec<u8>>;
    }
 }

// 4. Pallet Events
decl_event! { 
    pub enum Event<T> where 
    AccountId = <T as system::Trait>::AccountId,
    Balance = BalanceOf<T>  
    {

        ClaimCreated(AccountId, Vec<u8>),

        ClaimRevoked(AccountId, Vec<u8>),
 
        ClaimTransfered(AccountId, AccountId, Vec<u8>),

        ClaimBought(AccountId, Vec<u8>, Balance),
    }
}

// 5. Pallet Errors
decl_error! { 
    pub enum Error for Module<T: Trait> {

        ProofAlreadyClaimed,

        NoSuchProof,

        NotProofOwner,

        CannotTransferProofToSelf,

        ProofTooLong,

        InsufficientPrice,

        AccountBalanceNotEnough,

        CommentTooLong,

        ProofAlreadyOwned,

        ProofNotOwned,
    }
 }

// 6. Callable Pallet Functions
decl_module! { 

     pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>, comment: Vec<u8>, price: BalanceOf<T>) {

            ensure!(T::MaxClaimLength::get() >= proof.len() as u32, Error::<T>::ProofTooLong);

            ensure!(T::MaxCommentLength::get() >= comment.len() as u32, Error::<T>::CommentTooLong);
            
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <system::Module<T>>::block_number();

            let now = T::Time::now();

            let new_proof_info = ProofInfo {
                claimer: sender.clone(),
                owner: sender.clone(),
                comment: comment.clone(),
                price: price.clone(),
                created_on: now.clone(),
                claim_block: current_block,
            };

            Proofs::<T>::insert(&proof, new_proof_info);

            Self::_add_proof_to_owned_proofs(sender.clone(), proof.clone())?;

            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }

        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {

            let sender = ensure_signed(origin)?;

            let proof_info = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            ensure!(sender == proof_info.owner, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::_add_proof_to_owned_proofs(sender.clone(), proof.clone())?;

            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }

        #[weight = 0]
		fn transfer_claim(origin, proof: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) {
			let sender = ensure_signed(origin)?;

            let mut proof_info = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

			ensure!(sender == proof_info.owner, Error::<T>::NotProofOwner);

            let dest = T::Lookup::lookup(dest)?;

            ensure!(proof_info.owner != dest, Error::<T>::CannotTransferProofToSelf);

            proof_info.owner = dest.clone();

            Proofs::<T>::insert(&proof, proof_info);

            Self::_remove_proof_from_owned_proofs(sender.clone(), proof.clone())?;

            Self::_add_proof_to_owned_proofs(dest.clone(), proof.clone())?;

			Self::deposit_event(RawEvent::ClaimTransfered(sender, dest, proof));
        }
        
        #[weight = 0]
		pub fn buy_claim(origin, proof: Vec<u8>, amount: BalanceOf<T>) {

            let sender = ensure_signed(origin)?;
            
            let mut proof_info = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            ensure!(amount >= proof_info.price, Error::<T>::InsufficientPrice);

            T::Currency::transfer(&sender, &proof_info.owner, amount, AllowDeath)
                .map_err(|_| Error::<T>::AccountBalanceNotEnough)?;

            let seller = proof_info.owner;

            proof_info.owner = sender.clone();

            proof_info.price = amount.clone();

            Proofs::<T>::insert(&proof, &proof_info);

            
            Self::_remove_proof_from_owned_proofs(seller.clone(), proof.clone())?;

            Self::_add_proof_to_owned_proofs(sender.clone(), proof.clone())?;

            Self::deposit_event(RawEvent::ClaimBought(sender, proof, proof_info.price));
            
		}
    }
 }
 impl<T: Trait> Module<T> {

	pub fn _add_proof_to_owned_proofs(owner: T::AccountId, proof: Vec<u8>) -> DispatchResult {

        ensure!(!Self::owned_proofs(owner.clone()).contains(&proof), Error::<T>::ProofAlreadyOwned);

        let mut owned_proofs = Self::owned_proofs(owner.clone());
        
        owned_proofs.push(proof.clone());
        
        <OwnedProofs<T>>::insert(owner, owned_proofs);
        
        Ok(())
    }
    
	pub fn _remove_proof_from_owned_proofs(owner: T::AccountId, proof: Vec<u8>) -> DispatchResult {

        ensure!(Self::owned_proofs(owner.clone()).contains(&proof), Error::<T>::ProofNotOwned);

        let mut owned_proofs = Self::owned_proofs(owner.clone());
		
        let mut j = 0;
        
		for i in &owned_proofs{

			if *i == proof.clone() {

                owned_proofs.remove(j);
                
				break;
            }
            	
            j +=1; 
            
        }	
        
        <OwnedProofs<T>>::insert(owner, owned_proofs);

		Ok(())
	}
}
 