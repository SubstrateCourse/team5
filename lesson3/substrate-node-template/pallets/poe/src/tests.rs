// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
#[test]
fn test_crate_claim() {
 new_test_ext().execute_with(||{
     let _new_claim = vec![0,1];
     assert_ok!(PoeModule::create_claim(Origin::signed(1), _new_claim.clone()));
     assert_eq!(Proofs::<Test>::get(&_new_claim),(1,system::Module::<Test>::block_number()));


 })    
}


#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(||{
    let claim = vec![0,1];
    let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
    assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
    })

}


#[test]
fn when_proof_too_long() {
    new_test_ext().execute_with(||{
        let claim = vec![1,2,3,4,5,6,7];
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofTooLong);


    })  
}


#[test]
fn revoke_test_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));

    })
}
#[test]
fn revoek_a_not_exist_claim() {
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_noop!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()), Error::<Test>::ClaimNotExist);
    })
}
#[test]
fn not_the_claim_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_noop!(PoeModule::revoke_claim(Origin::signed(2),claim.clone()), Error::<Test>::NotClaimOwner);
    
        
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let dest : <Test as system::Trait>::AccountId = 123456789;
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), dest));
        assert_eq!(Proofs::<Test>::get(&claim),(dest,system::Module::<Test>::block_number()));

    })
}

#[test]
fn transfer_not_exist_claim() {
    new_test_ext().execute_with(||{
    let dest :<Test as system::Trait>::AccountId = 123456789;
    let claim = vec![0,1];
    assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(),dest), Error::<Test>::ClaimNotExist);

    })
}

#[test]
fn not_the_owner_want_to_transfer() {
    new_test_ext().execute_with(||{
        let dest :<Test as system::Trait>::AccountId = 123456789;
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(PoeModule::transfer_claim(Origin::signed(2), claim.clone(), dest), Error::<Test>::NotClaimOwner);

    })
}