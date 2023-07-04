#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
#[derive(Debug,Clone)]
enum opcodes{
	ILOAD_1,
	ICONST_2,
	IREM,
	IFEQ, 
	LDC,
	GOTO, 
	INVOKEVIRTUAL, 
	BLK_NO,
}
#[frame_support::pallet]
pub mod pallet {
	

	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// use rand::Rng;
	use sp_std::vec::Vec;
	// use sp_std::string::String;
	use const_random::const_random;
	use scale_info::prelude::string::String;
	use codec::alloc::string::ToString;
	// use ring::digest::{Context, SHA256};
	use sha2::{Sha256, Sha512, Digest};
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
	// pub type SC_Count<T> = StorageValue<_, u8>;
	static mut count: u8 = 0;
	#[pallet::storage]
	pub type SC<T> = StorageMap<_,Twox64Concat, Vec<u8>, String>;//change to vec after 

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: Vec<u8>, who: T::AccountId },
		SCUpload {address: Vec<u8>, who: T::AccountId},
		SCRetrived {contract: String},
		SCExecuted {output: String, bno: Option<u32>},
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
		
		// #[pallet::call_index(0)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn upload(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://docs.substrate.io/main-docs/build/origins/
		// 	let who = ensure_signed(origin)?;
		// 	// let mut hasher = DefaultHasher::new();
		// 	// hasher.write(&code);
		// 	// let address:u128 = hasher.finish().into();
		// 	// let hash = Blake2b::digest(&code);
		// 	let address: u32 = const_random!(u32);
		// 	SC::<T>::insert(address,code); 
		// 	Self::deposit_event(Event::SCUpload { address, who });
		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn upload(origin: OriginFor<T>, bytecode: String) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;
			// let mut hasher = DefaultHasher::new();
			// hasher.write(&code);
			// let address:u128 = hasher.finish().into();
			// let hash = Blake2b::digest(&code);
			let mut hasher = Sha256::new();
			// // let hash = hasher.finish();
			hasher.update(&bytecode);
			// hasher.update(into);
			let address: Vec<u8> = hasher.finalize().to_vec();
			// let mut address: u32 = const_random!(u32);
			SC::<T>::insert(&address,bytecode); 
			//Self::deposit_event(Event::SCExecuted { bytetemp });
			Self::deposit_event(Event::SCUpload { address, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn retrive(origin: OriginFor<T>, address: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let contract = SC::<T>::get(address).ok_or(Error::<T>::NoneValue)?;
			Self::deposit_event(Event::SCRetrived { contract });
			Ok(())
		}
		
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn execute(origin: OriginFor<T>, address: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let bytecode = SC::<T>::get(address).ok_or(Error::<T>::NoneValue)?;
			let mut output:String = "".to_string();
			// let binding = bytecode.replace(" \"","*");
			let opcodes: Vec<&str> = bytecode.split(",").collect();
			let mut stack: Vec<String> = Vec::new();
			let mut bno: Option<u32> = Some(0);
			let mut ifeq: u32 = 0;
			for opcode in opcodes {
				match opcode {
					"BLK_NO" => {
						let current_block_number = <frame_system::Pallet<T>>::block_number();
						// ;
						bno = TryInto::<u32>::try_into(current_block_number).ok();
						stack.push((bno).expect("REASON").to_string());
						// stack.push(<<T as frame_system::Config>::BlockNumber as Into<T>>::into(current_block_number).to_string());
					},
					"ICONST_2" => {
						stack.push(2.to_string());
					}
					"IREM" => {
						let a = (stack.pop()).expect("REASON").parse::<i32>().unwrap();
						let b = (stack.pop()).expect("REASON").parse::<i32>().unwrap();
						stack.push((b%a).to_string());
					},
					"IF_ICMPNE " => {
						let rem = (stack.pop()).expect("REASON").parse::<i32>().unwrap();
						if rem == 0{
							ifeq = 1;
						}
						else {
							continue;
						}
					},
					"EMIT \"You Win\"" => {
						if ifeq == 1{
							stack.push("You Win".to_string());
							// output = "You Win".to_string();
							break;
						}
						else{
							continue;
						}
					},
					"EMIT \"You Lose\"" => {
						stack.push("You Lose".to_string());
						// output = "You Lose".to_string();
						break;
					},
					
					
					&_ => {continue;}
				}
			}
			output = stack.pop().expect("REASON").to_string();
			Self::deposit_event(Event::SCExecuted { output: output.to_string(), bno });

			Ok(())
		}
	}
}
