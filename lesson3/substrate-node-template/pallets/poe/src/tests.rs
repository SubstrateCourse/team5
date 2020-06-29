// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};

// test for create_claim
#[test]
fn create_claim_works () {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));

    })
}
#[test]
fn create_claim_fail_when_claim_allready_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
    })
}

#[test]
fn create_claim_fail_when_claim_toolong() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 3, 4, 5, 6, 7];
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofTooLong);
    })
}

#[test]
fn revoke_claim_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}
#[test]
fn revoke_claim_fail_then_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotClaimOwner);
    })
}
#[test]
fn revoke_claim_fail_when_claim_not_exit() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let claim1 = vec![0, 2];
        PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim1.clone()), Error::<Test>::ClaimNotExist);
    })
}
#[test]
fn revoke_claim_fail_when_claim_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotClaimOwner);
    })
}
#[test]
fn transfer_claim_ok() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let dest = 2;
        PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), dest));
    })
}
#[test]
fn transfer_claim_fail_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let dest = 2;
        assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), dest), Error::<Test>::ClaimNotExist);
    })
}