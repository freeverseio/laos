#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, DefaultNoBound};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{CheckedAdd, One};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(
		Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct CompositeStruct<T: Config> {
		pub(crate) block_number: BlockNumberFor<T>,
	}

	#[pallet::storage]
	pub type Something<T: Config> = StorageValue<_, CompositeStruct<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { block_number: BlockNumberFor<T>, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, bn: u32) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let block_number: BlockNumberFor<T> = bn.into();
			<Something<T>>::put(CompositeStruct { block_number });
			Self::deposit_event(Event::SomethingStored { block_number, who });
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;
			match <Something<T>>::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(mut old) => {
					old.block_number = old
						.block_number
						.checked_add(&One::one())
						.ok_or(Error::<T>::StorageOverflow)?;
					<Something<T>>::put(old);
					Ok(().into())
				},
			}
		}
	}
}
