#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

// 依赖
use frame_support::{decl_module, decl_storage, decl_event, decl_error, 
	dispatch, ensure,traits::{Get}};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*; // 使用Vec
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
/// 主程序
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
		// 定义存储单元
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber)
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId,Vec<u8>),
		ClaimRevoked(AccountId,Vec<u8>),
		ClaimTransfer(AccountId,Vec<u8>),

	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
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
		pub fn create_claim(origin,claim: Vec<u8>) -> dispatch::DispatchResult{
		    let sender = ensure_signed(origin)?;

			// Check
		    ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

		    Proofs::<T>::insert(&claim,(sender.clone(),system::Module::<T>::block_number()));

		    Self::deposit_event(RawEvent::ClaimCreated(sender,claim));

		    Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult{
		    let sender = ensure_signed(origin)?;

			// 不存在抛出错误
		    ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

		    let (owner,_block_number) = Proofs::<T>::get(&claim);
		    ensure!(owner == sender, Error::<T>::NotClaimOwner);

		    Proofs::<T>::remove(&claim);

		    // 删除存证
		    Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));
		    Ok(())
		}

		#[weight = 0]
			pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			// 用户不存在或者不匹配则抛出错误
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			
			let (owner,_block_number) = Proofs::<T>::get(&claim);
			// 用户不匹配
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			
			let dest = T::Lookup::lookup(dest)?;
			Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number()));
			Ok(())
		}

	}
}
