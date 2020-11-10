#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use orml_nft;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


pub type CID = sp_std::vec::Vec<u8>;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait +  orml_nft::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		MyNftClassId get(fn nft_class_id): map hasher(twox_64_concat) T::AccountId => T::ClassId;
		MyNftTokenMetaData get(fn user_nft_token_metadata):  map hasher(twox_64_concat) T::AccountId => CID;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
		MyIssuedBy(AccountId),
		MyMinted(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_mynft(origin, metadata: CID, data: <T as orml_nft::Trait>::ClassData) -> dispatch::DispatchResult{
			let who = ensure_signed(origin)?;
			let res = <orml_nft::Module<T>>::create_class(&who,metadata.clone(),data);
			<MyNftClassId<T>>::insert(&who,res.unwrap());
			<MyNftTokenMetaData<T>>::insert(&who,metadata);
			Self::deposit_event(RawEvent::MyIssuedBy(who));
			Ok(())
		}
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn mint_mynft(origin, data: <T as orml_nft::Trait>::TokenData) -> dispatch::DispatchResult{
			let who = ensure_signed(origin)?;
			let metadata = <MyNftTokenMetaData<T>>::get(&who);
			let classid = <MyNftClassId<T>>::get(&who);
			
			let _res = <orml_nft::Module<T>>::mint(&who,classid,metadata,data);
			
			Self::deposit_event(RawEvent::MyMinted(who));
			Ok(())
		}
		
		
	}
}