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
	

	pub fn hash_pwd(pass: &str, salt: &str) -> (u64, String) {
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
	use std::time::{SystemTime};

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	struct PassHash {
		id: u64,
		hash: String
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn something)]


	pub type HashData<T> =  StorageMap<
		_,
		Twox64Concat,
		T::Hash,
		PassHash,
	>;

	#[pallet::storage]
	#[pallet::getter(fn hash_cnt)]
	pub(super) type HashCnt<T: Config> = StorageValue<_, u64, ValueQuery>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	
		Success(T::Time, T::Day),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		HashPwdOverflow,
		HashPwdExists
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn hash_pass(
			owner: &T::AccountId,
			password: Option<&str>,
			salt: Option<&str>,
		) -> Result<T::Hash, Error<T>> {
			let (hash_id, hash_string) = argon2_hash::hash_pwd(password, salt);


			let p = PassHash::<T> {
				id: hash_id,
				hash: hash_string
			};
		
			let hpwd_id = T::Hashing::hash_of(&p);
		
			let new_cnt = Self::hash_cnt().checked_add(1)
				.ok_or(<Error<T>>::HashPwdOverflow)?;
		
			ensure!(Self::kitties(&hpwd_id) == None, <Error<T>>::HashPwdExists);
		
			Self::deposit_event(Event::Success(SystemTime::now(), 0));
		
			<Kitties<T>>::insert(hpwd_id, p);
			<KittyCnt<T>>::put(new_cnt);
			Ok(hpwd_i)
		}

	
	}
}
