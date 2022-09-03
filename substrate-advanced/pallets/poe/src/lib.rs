#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet] // 定义功能模块
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config] // 定义配置接口
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MaxClaimLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage] // 定义存储单元
	pub(super) type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
		OptionQuery,
	>;

	#[pallet::event] // 定义事件回调
	#[pallet::generate_deposit(pub (super) fn deposit_event)] //系统的事件，用于更方便触发事件
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimTrans(T::AccountId,T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
	}

	#[pallet::error] // 定义错误信息
	pub enum Error<T> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
	}


	#[pallet::call] // 包含可调用函数
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)] // 设置权重，可转换成交易费用，以此来防止类似拒绝服务的攻击
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxClaimLength>,
		) -> DispatchResult {

			// 做必要检查，检查内容： 1，交易发送方是不是一个签名的用户 2，存证是否被别人创建过，创建过就抛出错误
			// 存证拥有人是交易发送方，只有拥有人才可以调用存证，sender即当前交易发送方
			let sender = ensure_signed(origin)?;

			// 使用ensure!宏检查是否存证存证
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Pallet<T>>::block_number();

			// 不存在执行插入操作，key是存证的hash值，value是当前的发送方跟当前交易所在的区块高度，使用block_number这个系统函数进行查询
			Proofs::<T>::insert(&proof, (&sender, current_block));

			// 发送事件
			Self::deposit_event(Event::ClaimCreated(sender, proof));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxClaimLength>, //
		) -> DispatchResult {

			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");

			ensure!(sender == owner, Error::<T>::NotProofOwner);

			Proofs::<T>::remove(&proof);

			Self::deposit_event(Event::ClaimRevoked(sender, proof));
			Ok(())
		}

		#[pallet::weight(100_000)]
		pub fn trans_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxClaimLength>,
			receiver: T::AccountId,
		) -> DispatchResult {

			// 检查交易发送方是不是一个签名的用户
			let sender = ensure_signed(origin)?;

			// 检查存证是否存在
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			// 检查发送方是否为存证的owner
			let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			// 直接将原来的覆盖
			let current_block = <frame_system::Pallet<T>>::block_number();
			Proofs::<T>::insert(&proof, (&receiver, current_block));

			Self::deposit_event(Event::ClaimTrans(sender, receiver,proof));
			Ok(())
		}
	}
}
