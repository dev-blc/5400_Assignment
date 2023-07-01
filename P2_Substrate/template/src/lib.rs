#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;


#[frame_support::pallet]
pub mod pallet {
	

	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// use rand::Rng;
	use sp_std::vec::Vec;
	use const_random::const_random;
	// use sp_std::convert::TryInto;
	// extern crate collections;
	// use rand::thread_rng;
	// use std::collections::hash_map::HashMap;
	// use sp_std::collections::hash_map::DefaultHasher;
	// use codec::alloc::collections::hash_map::DefaultHasher;
	// use sp_std::hash::Hasher;
	// use blake2::{Blake2b, Digest};
	// use std::collections::HashMap;
	// use std::hash::BuildHasherDefault;

	// use hashers::fx_hash::FxHasher;

	
	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	// #[pallet::storage]
	// #[pallet::getter(fn something)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	// pub type Something<T> = StorageValue<_, u128>;
	// #[pallet::storage]
	// #[pallet::getter(fn sc_count)]
	// pub type SC_Count<T> = StorageValue<_, u32>;
	#[pallet::storage]
	pub type SC<T> = StorageMap<_,Twox64Concat, u32, sp_std::vec::Vec<u8>>;//change to vec after 

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		SCUpload {address: u32, who: T::AccountId},
		SCRetrived {contract: Vec<u8>, arg: u32},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn upload(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;
			// let mut hasher = DefaultHasher::new();
			// hasher.write(&code);
			// let address:u128 = hasher.finish().into();
			// let hash = Blake2b::digest(&code);
			let address: u32 = const_random!(u32);
			SC::<T>::insert(address,code); 
			Self::deposit_event(Event::SCUpload { address, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn retrive(origin: OriginFor<T>, arg: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let contract = SC::<T>::get(arg).ok_or(Error::<T>::NoneValue)?;
			Self::deposit_event(Event::SCRetrived { contract, arg });
			Ok(())
		}
	}
}
