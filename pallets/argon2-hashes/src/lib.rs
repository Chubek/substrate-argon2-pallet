#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod argon2_hash;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use crate::argon2_hash;
	use frame_support::sp_runtime::traits::Hash;
	use scale_info::TypeInfo;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct PassHash<T: Config> {
		pub id: u64,
		pub hash: [u8; 64],
		pub owner: AccountOf<T>
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn hashdata)]
	pub type HashData<T: Config> =  StorageMap<
		_,
		Twox64Concat,
		T::Hash,
		PassHash<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn hash_cnt)]
	pub(super) type HashCnt<T: Config> = StorageValue<_, u64, ValueQuery>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	
		Success(T::Hash, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		HashPwdOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1)]
		pub fn add_to_chain(
			origin: OriginFor<T>,
			password: String,
			salt: String
		) -> DispatchResult {
			let owner_id = ensure_signed(origin)?;



			let (hash_id, hash_string) = argon2_hash::hash_pwd(password.as_str(), salt.as_str());

			let hash_array: [u8; 64] = hash_string.as_bytes().try_into().unwrap();

			let p = PassHash::<T> {
				id: hash_id,
				hash: hash_array,
				owner: owner_id
			};
		
			let hpwd_id = T::Hashing::hash_of(&p);
		
			let new_cnt = Self::hash_cnt().checked_add(1)
				.ok_or(<Error<T>>::HashPwdOverflow)?;
			<HashData<T>>::insert(hpwd_id, p);
			<HashCnt<T>>::put(new_cnt);
	
			Self::deposit_event(Event::Success(hpwd_id , new_cnt));

			Ok(())
		}
	
	}
}
