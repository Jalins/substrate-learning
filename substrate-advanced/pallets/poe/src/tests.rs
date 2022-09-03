use super::*;
use crate::{mock::*};
use frame_support::{assert_ok, BoundedVec, assert_noop};

// ======================================================= 1.创建用例 ==================================================
// 1.1 测试创建存证用例：存证不存在场景
#[test]
fn test_create_claim_work() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 
		
		assert_ok!(PoeModule::create_claim(Origin::signed(1),bounded_claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

// 1.2 测试创建存证用例：存证已存在的场景
#[test]
fn test_create_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 

		let _ = PoeModule::create_claim(Origin::signed(1),bounded_claim.clone());

		// assert_noop 表示操作不会真的执行
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),bounded_claim.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
		
	});
}

// ======================================================= 2.销毁用例 =========================================================

// 2.1 测试销毁存证用例： 存证存证场景
#[test]
fn test_remove_claim_work() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 
		let _ = PoeModule::create_claim(Origin::signed(1),bounded_claim.clone());
		let _ = PoeModule::revoke_claim(Origin::signed(1), bounded_claim.clone());

		assert_eq!(
			Proofs::<Test>::get(bounded_claim.clone()),
			None
		)
			
	})
}


// 2.1 测试销毁存证用例： 存证不存在场景
#[test]
fn test_remove_claim_does_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), bounded_claim.clone()),
			Error::<Test>::NoSuchProof
		);
	})
}

// 2.3 测试销毁存证用例： 存证不属于当前调用者
#[test]
fn test_remove_claim_does_not_belong_to_currentowner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 
		let _ = PoeModule::create_claim(Origin::signed(1),bounded_claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), bounded_claim.clone()),
			Error::<Test>::NotProofOwner
		);
	})
}


// ============================================== 3.转移用例 ============================================================
// 3.1 测试销毁存证用例： 存证存证场景
#[test]
fn test_transform_claim_work() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 
		let _ = PoeModule::create_claim(Origin::signed(1),bounded_claim.clone());

		let _ = PoeModule::trans_claim(Origin::signed(1), bounded_claim.clone(), 2u64);

		assert_eq!(
			Proofs::<Test>::get(bounded_claim.clone()),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		)
			
	})
}


// 3.1 测试销毁存证用例： 存证不存在场景
#[test]
fn test_transform_claim_does_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 
		assert_noop!(
			PoeModule::trans_claim(Origin::signed(1), bounded_claim.clone(), 2u64),
			Error::<Test>::NoSuchProof
		);
	})
}

// 3.3 测试销毁存证用例： 存证不属于当前调用者
#[test]
fn test_transform_claim_does_not_belong_to_currentowner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap(); 
		let _ = PoeModule::create_claim(Origin::signed(1),bounded_claim.clone());

		assert_noop!(
			PoeModule::trans_claim(Origin::signed(2), bounded_claim.clone(), 2u64),
			Error::<Test>::NotProofOwner
		);
	})
}
