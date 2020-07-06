// Tests to be written here

use crate::{*, mock::*};
use frame_support::{assert_ok, assert_noop};
use codec::{Encode, Decode};

#[test]
fn test_onchain() {
	let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();
	t.execute_with(|| {
		let index = 1;
		let value = 2;
		let account: <Test as system::Trait>::AccountId = Default::default();

		assert_ok!(TemplateModule::save_number(Origin::signed(account), index, value));
		assert_eq!(<Numbers>::get(index), value);

		let expect_event = TestEvent::template(RawEvent::NumberAppended(account, index, value));
		assert!(System::events().iter().any(|er| er.event == expect_event));
	});
}

#[test]
fn test_offchain() {
	let (mut t, pool_state, _offchain_state) = ExtBuilder::build();
    t.execute_with(|| {
		let account: <Test as system::Trait>::AccountId = Default::default();

		TemplateModule::submit_number(0);
		assert_ok!(TemplateModule::save_number(Origin::signed(account), 0, 1));

		TemplateModule::submit_number(1);
		assert_ok!(TemplateModule::save_number(Origin::signed(account), 1, 5));

		TemplateModule::submit_number(2);
		assert_ok!(TemplateModule::save_number(Origin::signed(account), 2, 14));

		TemplateModule::submit_number(3);

		let tx4 = pool_state.write().transactions.pop().unwrap();
		let tx3 = pool_state.write().transactions.pop().unwrap();
		let tx2 = pool_state.write().transactions.pop().unwrap();
		let tx1 = pool_state.write().transactions.pop().unwrap();
        assert!(pool_state.read().transactions.is_empty());

		let tx1decoded = TestExtrinsic::decode(&mut &*tx1).unwrap();
		assert_eq!(tx1decoded.call, Call::save_number(0, 1));

		let tx2decoded = TestExtrinsic::decode(&mut &*tx2).unwrap();
		assert_eq!(tx2decoded.call, Call::save_number(1, 5));

		let tx3decoded = TestExtrinsic::decode(&mut &*tx3).unwrap();
		assert_eq!(tx3decoded.call, Call::save_number(2, 14));

		let tx4decoded = TestExtrinsic::decode(&mut &*tx4).unwrap();
        assert_eq!(tx4decoded.call, Call::save_number(3, 30));
	})
}
