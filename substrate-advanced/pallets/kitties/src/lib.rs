#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, DispatchResult}, traits::Randomness,traits::Currency, traits::ReservableCurrency};
	use frame_system::pallet_prelude::{*, OriginFor};
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, CheckedAdd};

	// type KittyIndex  = u32;

	#[pallet::type_value]
	pub fn GetdefaultValue()-> u32{
		0_u32
	}

	#[derive(Encode,Decode,Clone, PartialEq,Eq,Debug,TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8;16]);

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type KittyIndex: Copy
			+ Member
			+ Default
			+ MaxEncodedLen
			+ Parameter
			+ Bounded
			+ AtLeast32BitUnsigned;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		// kitty最多数量
		type MaxKittyLength: Get<u32>;

		// 定义kitty价格
		type KittyPrice: Get<BalanceOf<Self>>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub(super) type NextKittyId<T:Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T:Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex,Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_onwer)]
	pub type KittyOnwer<T: Config> = StorageMap<_, Blake2_128Concat,T::KittyIndex,T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn all_kts_owned)]
	pub type KittyOnwerHistory<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Kitty, T::MaxKittyLength>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		KittyCreate(T::AccountId, T::KittyIndex, Kitty),
		KittyBreed(T::AccountId, T::KittyIndex, Kitty),
		KittyTransfor(T::AccountId,T::KittyIndex, T::AccountId),
	}
	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		NotOwner,
		KittyIndexOverflow,
		BalanceNotEnough,
		OverLimitOnwerForKitty,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult{
			// 1.判断当前用户是否是有效用户
			let sender = ensure_signed(origin)?;

			// 2.质押token
			T::Currency::reserve(&sender, T::KittyPrice::get())
				.map_err(|_| Error::<T>::BalanceNotEnough)?;

			// 3.获取新的kitty id
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
			
			// 4.计算一个新的随机数
			let dna = Self::random_value(&sender);
			let kitty = Kitty(dna);

			// 5.更新存储
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOnwer::<T>::insert(kitty_id, &sender);
			// 5.1 这里要用到checked_add来保证数据计算的安全性
			let next_kitty_id = 
				kitty_id.checked_add(&(T::KittyIndex::from(1_u8))).ok_or(Error::<T>::KittyIndexOverflow)?;
			NextKittyId::<T>::set(next_kitty_id);

			// 5.2这里是将当前用户说拥有的kitty都保存在一个vec中
			KittyOnwerHistory::<T>::try_mutate(&sender, |kitty_vec| kitty_vec.try_push(kitty.clone()))
				.map_err(|_| <Error<T>>::OverLimitOnwerForKitty)?;

			// 6.发送事件
			Self::deposit_event(Event::<T>::KittyCreate(sender, kitty_id, kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: T::KittyIndex ,kitty_id_2: T::KittyIndex) -> DispatchResult {
			// 1.判断当前用户是否是有效用户
			let sender = ensure_signed(origin)?;

			// 2.质押token
			T::Currency::reserve(&sender, T::KittyPrice::get())
				.map_err(|_| Error::<T>::BalanceNotEnough)?;

			// 3.通过kitty index获取 kitty_1跟kitty_2
			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

			// 4.获取一个新的kitty_id
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			// 5.计算一个新的随机数
			let selector = Self::random_value(&sender);

			// 6.通过&计算来得到一个新的随机数，并且这个随机数与kitty_1和kitty_2相关
			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len(){
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & selector[i]);
			}
			let new_kitty = Kitty(data);

			// 7.更新存储
			Kitties::<T>::insert(kitty_id, &new_kitty);
			KittyOnwer::<T>::insert(kitty_id,&sender);
			// 7.1 这里要用到checked_add来保证数据计算的安全性
			let next_kitty_id = 
				kitty_id.checked_add(&(T::KittyIndex::from(1_u8))).ok_or(Error::<T>::KittyIndexOverflow)?;
			NextKittyId::<T>::set(next_kitty_id);
			// 7.2这里是将当前用户说拥有的kitty都保存在一个vec中
			KittyOnwerHistory::<T>::try_mutate(&sender, |kitty_vec| kitty_vec.try_push(new_kitty.clone()))
				.map_err(|_| <Error<T>>::OverLimitOnwerForKitty)?;

			// 8.发送事件
			Self::deposit_event(Event::<T>::KittyBreed(sender, kitty_id, new_kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfor(origin: OriginFor<T>, kitty_id: T::KittyIndex, to: T::AccountId) -> DispatchResult{
			// 1.判断当前用户是否是有效用户
			let sender = ensure_signed(origin)?;

			// 2.判断kitty id是否存在
			let kitty = Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			// 3.判断当前用户是否为该kitty的拥有者
			ensure!(Self::kitty_onwer(kitty_id) == Some(sender.clone()), Error::<T>::NotOwner);

			// 4.解除当前用户的质押token
			T::Currency::unreserve(&sender, T::KittyPrice::get());

			// 5.质押kitty接收者的token
			T::Currency::reserve(&to, T::KittyPrice::get())
				.map_err(|_| Error::<T>::BalanceNotEnough)?;

			// 6.更改kitty的拥有者
			KittyOnwer::<T>::insert(kitty_id, &to);

			// 7.把当前的kitty从当前用户的kitty列表中删除
			KittyOnwerHistory::<T>::try_mutate(&sender,|kitties_vec| {
				if let Some(index) = kitties_vec.iter().position(|kitty_in_vec| kitty_in_vec == &kitty) {
					kitties_vec.remove(index);
					return Ok(());
				}
				Err(())
					
			}).map_err(|_| Error::<T>::NotOwner)?;


			// 8.把当前的kitty添加到接收者的kitty列表中
			KittyOnwerHistory::<T>::try_mutate(&to, |kitties_vec| kitties_vec.try_push(kitty.clone()))
				.map_err(|_| <Error<T>>::OverLimitOnwerForKitty)?;

			Self::deposit_event(Event::<T>::KittyTransfor(sender, kitty_id, to));
			Ok(())
		}

	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16]{
			let payload = (
				T::Randomness::random_seed(),
				sender, 
				<frame_system::Pallet::<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		fn get_next_id() -> Result<T::KittyIndex,()>	{
			let kitty_id = Self::next_kitty_id();
			if kitty_id == T::KittyIndex::max_value() {
				return Err(());
			}
			Ok(kitty_id)
		}

		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()>{
			match Self::kitties(kitty_id){
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}
	}
}
