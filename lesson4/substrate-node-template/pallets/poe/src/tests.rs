// Tests to be written here

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::{self as system};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];
        let origin = 1;
        assert_ok!(PoeModule::create_claim(
            Origin::signed(origin),
            claim.clone()
        ));
        assert_eq!(
            PoeModule::proofs(claim.clone()),
            (origin, system::Module::<Test>::block_number())
        );
    });
}

#[test]
fn create_claim_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let origin = 1;
        assert_noop!(
            PoeModule::create_claim(Origin::signed(origin), claim.clone()),
            Error::<Test>::ProofTooLong
        );
    });
}

#[test]
fn create_claim_already_exists() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];
        let origin = 1;
        PoeModule::create_claim(Origin::signed(origin), claim.clone()).unwrap();

        assert_noop!(
            PoeModule::create_claim(Origin::signed(origin), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];
        let origin = 1;

        assert_ok!(PoeModule::create_claim(
            Origin::signed(origin),
            claim.clone()
        ));
        assert_eq!(
            PoeModule::proofs(claim.clone()),
            (origin, system::Module::<Test>::block_number())
        );

        assert_ok!(PoeModule::revoke_claim(
            Origin::signed(origin),
            claim.clone()
        ));
        assert_eq!(PoeModule::proofs(claim.clone()), (0, 0));
    });
}

#[test]
fn revoke_claim_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];
        let origin = 1;

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(origin), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}

#[test]
fn revoke_claim_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];
        let origin_before = 1;
        let origin_after = 2;

        assert_ok!(PoeModule::create_claim(
            Origin::signed(origin_before),
            claim.clone()
        ));
        assert_eq!(
            PoeModule::proofs(claim.clone()),
            (origin_before, system::Module::<Test>::block_number())
        );

        assert_ok!(PoeModule::transfer_claim(
            Origin::signed(origin_before),
            claim.clone(),
            origin_after
        ));
        assert_eq!(
            PoeModule::proofs(claim.clone()),
            (origin_after, system::Module::<Test>::block_number())
        );
    });
}

#[test]
fn transfer_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];
        let origin_before = 1;
        let origin_after = 2;

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(origin_before), claim.clone(), origin_after),
            Error::<Test>::ClaimNotExist
        );
    });
}

#[test]
fn transfer_claim_not_claim_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![42];
        let origin_before = 1;
        let origin_after = 2;

        assert_ok!(PoeModule::create_claim(
            Origin::signed(origin_before),
            claim.clone()
        ));

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(3), claim.clone(), origin_after),
            Error::<Test>::NotClaimOwner
        );
    });
}
