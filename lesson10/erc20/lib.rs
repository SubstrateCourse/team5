#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;

    #[ink(storage)]
    struct Erc20 {
        total_supply: storage::Value<Balance>,

        balances: storage::HashMap<AccountId, Balance>,

        allowance: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(supply);
            self.balances.insert(caller, supply);
            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: supply,
            });
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply.get()
        }

        #[ink(message)]
        fn balance_of(&self, account: AccountId) -> Balance {
            self.get_balance_or_zero(&account)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.get_balance_or_zero(&from);
            if from_balance < value {
                return false;
            }

            let to_balance = self.get_balance_or_zero(&to);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowance.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            true
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let spender = self.env().caller();
            let allowance = self.get_allowance_or_zero(&from, &spender);
            if allowance < value {
                return false;
            }

            self.allowance.insert((from, spender), allowance - value);
            self.transfer_from_to(from, to, value);

            true
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.get_allowance_or_zero(&owner, &spender)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_balance = self.get_balance_or_zero(&from);
            if from_balance < value {
                return false;
            }
            let to_balance = self.get_balance_or_zero(&to);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }

        fn get_balance_or_zero(&self, account: &AccountId) -> Balance {
            *self.balances.get(account).unwrap_or(&0)
        }

        fn get_allowance_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            *self.allowance.get(&(*owner, *spender)).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_core::env;
        use ink_core::env::call::CallData;

        fn run_test<F>(test_fn: F)
            where
                F: FnOnce(),
        {
            env::test::run_test::<env::DefaultEnvTypes, _>(|_| {
                test_fn();
                Ok(())
            })
                 .unwrap()
        }

        #[test]
        fn new_works() {
            let erc20 = Erc20::new(444);
            assert_eq!(erc20.total_supply(), 444);
        }

        #[test]
        fn balance_of_works() {
            let erc20 = Erc20::new(1000);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 1000);
            assert_eq!(erc20.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[test]
        fn transfer_works() {
            let mut erc20 = Erc20::new(1000);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 1000);

            let to_addr = AccountId::from([0x0; 32]);
            erc20.transfer(to_addr, 400);

            assert_eq!(erc20.balance_of(AccountId::from([0x0; 32])), 400);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 600);
        }

        #[test]
        fn approve_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);

            contract.approve(AccountId::from([0x0; 32]), 20);
            assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x0; 32])), 20);
        }

        #[test]
        fn transfer_from_works() {
            let mut erc20 = Erc20::new(100);
            assert_eq!(env::test::recorded_events().count(), 1);
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
                .expect("Cannot get accounts");

            assert_eq!(erc20.transfer_from(accounts.alice, accounts.eve, 10), false);
            assert_eq!(erc20.approve(accounts.bob, 10), true);

            assert_eq!(env::test::recorded_events().count(), 2);

            let callee =
                env::account_id::<env::DefaultEnvTypes>().unwrap_or([0x0; 32].into());
            let mut data =
                CallData::new(env::call::Selector::new([0x00; 4]));
            data.push_arg(&accounts.bob);

            assert_eq!(
                env::test::push_execution_context::<env::DefaultEnvTypes>(
                    accounts.bob,
                    callee,
                    1000000,
                    1000000,
                    data
                ),
                ()
            );

            assert_eq!(erc20.transfer_from(accounts.alice, accounts.eve, 10), true);
            assert_eq!(env::test::recorded_events().count(), 3);
            assert_eq!(erc20.balance_of(accounts.eve), 10);
        }
    }
}
