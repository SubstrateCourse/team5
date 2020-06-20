// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_claim_with_memo_ok() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let memo = vec![0,1,3,4,5,6,7,5,1];
        assert_ok!(PoeModule::create_claim_with_memo(Origin::signed(1), claim.clone(), memo.clone()));
        assert_eq!(ProofsWithMemo::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number(), memo));
    })
}