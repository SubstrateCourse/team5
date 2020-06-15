// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// test cases for create_claim
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let proof_info = ProofInfo {
            claimer: 1,
            owner: 1,
            price: 10,
            claim_block: system::Module::<Test>::block_number(),
        }; 
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone(), 10));
        assert_eq!(Proofs::<Test>::get(&claim), Some(proof_info));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 10);

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone(), 10),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3, 4, 5, 6];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone(), 10),
            Error::<Test>::ProofTooLong
        );
    })
}

// test cases for revoke_claim
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 10);

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 10);

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

// test cases for transfer_claim
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 10);

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        let proof_info = ProofInfo {
            claimer: 1,
            owner: 2,
            price: 10,
            claim_block: system::Module::<Test>::block_number(),
        }; 
        assert_eq!(Proofs::<Test>::get(&claim), Some(proof_info))
    })
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 10);

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 2),
            Error::<Test>::NotProofOwner
        );
    })
}

// test cases for buy_claim
#[test]
fn buy_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 10);

        let proof_info_1 = ProofInfo {
            claimer: 1,
            owner: 1,
            price: 10,
            claim_block: system::Module::<Test>::block_number(),
        };

        assert_eq!(Proofs::<Test>::get(&claim), Some(proof_info_1));
        let _ = PoeModule::buy_claim(Origin::signed(2), claim.clone(), 12);
        let proof_info_2 = ProofInfo {
            claimer: 1,
            owner: 1,
            price: 10,
            claim_block: system::Module::<Test>::block_number(),
        };

        assert_eq!(Proofs::<Test>::get(&claim), Some(proof_info_2));
    })
}

#[test]
fn buy_claim_failed_with_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::buy_claim(Origin::signed(2), claim.clone(), 12),
            Error::<Test>::NoSuchProof
        );
    })
}


#[test]
fn buy_claim_failed_with_insufficient_price() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 10);

        assert_noop!(
            PoeModule::buy_claim(Origin::signed(2), claim.clone(), 8),
            Error::<Test>::InsufficientPrice
        );
    })
}

