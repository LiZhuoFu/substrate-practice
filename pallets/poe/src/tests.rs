use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_eq!(
			Claims::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _=PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::AlreadyClaimed
		);
	})
}
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _=PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
		
	})
}
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::NoSuchClaim
		);
	})
}
#[test]
fn move_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _=PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_ok!(PoeModule::move_claim(RuntimeOrigin::signed(1), 2, claim.clone()));
	})
}