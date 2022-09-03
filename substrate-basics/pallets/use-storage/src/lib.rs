#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::{*, OptionQuery, StorageMap}, Blake2_128Concat};
	use frame_system::{pallet_prelude::*, ensure_signed};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	// Class用于存储班级编号，只有root权限才能操作
	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	pub type Class<T:Config> = StorageValue<_, u32>;

	// StudentsInfo用于存储学号与姓名的对应关系
	#[pallet::storage]
	#[pallet::getter(fn students_info)]
	pub type StudentsInfo<T:Config> = StorageMap<_, Blake2_128Concat, u32, u128, OptionQuery>;

	// DormInfo用于存储寝室号、床号与学号之间的对应关系
	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	pub type DormInfo<T:Config> = StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32,  u32, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetClass(u32),
		SetStudentsInfo(u32,u128),
		SetDormInfo(u32,u32,u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		SetClassDuplicate,
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_class(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo{
			ensure_root(origin)?;
			if Class::<T>::exists() {
				return Err(Error::<T>::SetClassDuplicate.into());
			}
			Class::<T>::put(class);
			Self::deposit_event(Event::SetClass(class));
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_students_info(origin: OriginFor<T>, student_number: u32, student_name: u128) -> DispatchResultWithPostInfo{ 
			ensure_signed(origin)?;
			StudentsInfo::<T>::insert(student_number, student_name);
			Self::deposit_event(Event::SetStudentsInfo(student_number, student_name));
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_dorm_info(origin: OriginFor<T>, dorm_number: u32, bed_number: u32, student_number: u32) -> DispatchResultWithPostInfo{
			ensure_signed(origin)?;
			DormInfo::<T>::insert(dorm_number,bed_number, student_number);
			Self::deposit_event(Event::SetDormInfo(dorm_number, bed_number, student_number));
			Ok(().into())
		}
	}
}