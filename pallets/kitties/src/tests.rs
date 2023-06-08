use crate::{mock::*, Error, Event, Kitty, KittyId, KittyName, NextKittyId};
use frame_support::{assert_noop, assert_ok, pallet_prelude::DispatchResultWithPostInfo};

const ACCOUNT_ID_1: AccountId = 1;
const ACCOUNT_ID_2: AccountId = 2;
const KITTY_ID_0: KittyId = 0;
const KITTY_NAME: KittyName = *b"12345678";

fn init_balance(account: AccountId, new_free: Balance) -> DispatchResultWithPostInfo {
	Balances::force_set_balance(RuntimeOrigin::root(), account, new_free)
}
// #[test]
// fn it_works_for_mock() {
// 	new_test_ext().execute_with(|| {

// 		assert_eq!(1, 1);
// 	});
// }

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ACCOUNT_ID_1, 10_000_000));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 检查初始状态
		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID_0);

		// create kitty
		assert_ok!(KittiesModule::create(signer.clone(), KITTY_NAME));
		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID_0 + 1);
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_1));
		assert_eq!(KittiesModule::kitty_parents(KITTY_ID_0), None);
		let kitty = Kitty { name: KITTY_NAME, dna: KittiesModule::random_value(&ACCOUNT_ID_1) };
		assert_eq!(KittiesModule::kitties(KITTY_ID_0), Some(kitty.clone()));
		System::assert_last_event(
			Event::KittyCreated { who: ACCOUNT_ID_1, kitty_id: KITTY_ID_0, kitty }.into(),
		);

		// KittyId 溢出
		NextKittyId::<Test>::set(KittyId::max_value());
		assert_noop!(KittiesModule::create(signer, KITTY_NAME), Error::<Test>::InvalidKittyId,);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ACCOUNT_ID_1, 10_000_000));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		let parent_id_0 = KITTY_ID_0;
		let parent_id_1 = KITTY_ID_0 + 1;
		let child_id = KITTY_ID_0 + 2;

		// parent 相同
		assert_noop!(
			KittiesModule::breed(signer.clone(), parent_id_0, parent_id_0, KITTY_NAME),
			Error::<Test>::SameKittyId
		);
		// parent 不存在
		assert_noop!(
			KittiesModule::breed(signer.clone(), parent_id_0, parent_id_1, KITTY_NAME),
			Error::<Test>::InvalidKittyId
		);

		// 创建两只Kitty作为parent
		assert_ok!(KittiesModule::create(signer.clone(), KITTY_NAME));
		assert_ok!(KittiesModule::create(signer.clone(), KITTY_NAME));
		assert_eq!(KittiesModule::next_kitty_id(), child_id);
		let parent_1 = Kitty { name: KITTY_NAME, dna: KittiesModule::random_value(&ACCOUNT_ID_1) };
		let parent_2 = Kitty { name: KITTY_NAME, dna: KittiesModule::random_value(&ACCOUNT_ID_1) };

		// bred kitty
		assert_ok!(KittiesModule::breed(signer, parent_id_0, parent_id_1, KITTY_NAME));
		assert_eq!(KittiesModule::next_kitty_id(), child_id + 1);
		assert_eq!(KittiesModule::kitty_owner(child_id), Some(ACCOUNT_ID_1));
		assert_eq!(KittiesModule::kitty_parents(child_id), Some((parent_id_0, parent_id_1)));
		let child = Kitty {
			name: KITTY_NAME,
			dna: KittiesModule::child_kitty_dna(&ACCOUNT_ID_1, &parent_1, &parent_2),
		};
		assert_eq!(KittiesModule::kitties(child_id), Some(child.clone()));
		System::assert_last_event(
			Event::KittyBreed { who: ACCOUNT_ID_1, kitty_id: child_id, kitty: child }.into(),
		);
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ACCOUNT_ID_1, 10_000_000));
		assert_ok!(init_balance(ACCOUNT_ID_2, 10_000_000));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);
		let signer_2 = RuntimeOrigin::signed(ACCOUNT_ID_2);

		// transfer 不存在的 kitty
		assert_noop!(
			KittiesModule::transfer(signer.clone(), ACCOUNT_ID_2, KITTY_ID_0),
			Error::<Test>::NoOwner
		);

		// create kitty
		assert_ok!(KittiesModule::create(signer.clone(), KITTY_NAME));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_1));

		// 非ower进行transfer
		assert_noop!(
			KittiesModule::transfer(signer_2.clone(), ACCOUNT_ID_1, KITTY_ID_0),
			Error::<Test>::NotOwner
		);

		// transfer 给 ower
		assert_noop!(
			KittiesModule::transfer(signer.clone(), ACCOUNT_ID_1, KITTY_ID_0),
			Error::<Test>::TransferKittyToOwner
		);

		// transfer 1 -> 2
		assert_ok!(KittiesModule::transfer(signer, ACCOUNT_ID_2, KITTY_ID_0));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_2));
		System::assert_last_event(
			Event::KittyTransferred {
				who: ACCOUNT_ID_1,
				recipient: ACCOUNT_ID_2,
				kitty_id: KITTY_ID_0,
			}
			.into(),
		);

		// transfer 2 -> 1
		assert_ok!(KittiesModule::transfer(signer_2, ACCOUNT_ID_1, KITTY_ID_0));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_1));
		System::assert_last_event(
			Event::KittyTransferred {
				who: ACCOUNT_ID_2,
				recipient: ACCOUNT_ID_1,
				kitty_id: KITTY_ID_0,
			}
			.into(),
		);
	});
}
