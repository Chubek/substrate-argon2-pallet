#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


pub mod argon2_hash {
	use argon2::{self, Config};
	use std::hash::{Hash, Hasher};
	use fasthash::{metro, MetroHasher};
	use rand::Rng;

	fn hash_func_id<T: Hash>(t: &T) -> u64 {
		let mut s: MetroHasher = Default::default();
		t.hash(&mut s);
		s.finish()
	}
	

	pub fn hash_pass(pass: &str, salt: &str) -> (u64, String) {
		let config = Config::default();
		let mut rng = rand::thread_rng();

		let hash = argon2::hash_encoded(pass.as_bytes(), salt.as_bytes(), &config).unwrap();

		let rand_val: u16 = rng.gen();

		let id_hash = hash_func_id(format!("{} {} {}", pass, hash, rand_val));

		return (id_hash, hash)


	}

	pub fn verify_pass(hash: String, password: &str) -> bool {
		return argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
	}

}



#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn something)]


	pub type Something<T> = StorageValue<_, u32>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


	
	}
}
