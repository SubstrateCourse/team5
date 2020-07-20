#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

//noinspection ALL
#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;

    #[ink(storage)]
    struct Erc20 {
        total_supply: storage::Value<Balance>,
        balances: storage::HashMap<AccountId, Balance>,
        allowance: storage::HashMap<(AccountId,AccountId), Balance>,
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
    struct Approve {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, init_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(init_supply);
            self.balances.insert(caller, init_supply);
            self.env().emit_event(Transfer {
                from: None, to: Some(caller), value: init_supply,
            });
        }

        #[ink(constructor)]
        fn default(&mut self) {
            self.new(1_000_000_000_000 * 10_000)
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.get_balance(&owner)
        }

        fn get_balance(&self, who: &AccountId) -> Balance {
            *self.balances.get(who).unwrap_or(&0)
        }
        fn get_allowance(&self, from: &AccountId, to: &AccountId) -> Balance {
            *self.allowance.get(&(from.clone(), to.clone())).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.get_balance(&from);
            if from_balance < value {
                return false;
            }
            let to_balance = self.get_balance(&to);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from), to: Some(to), value,
            });
            true
        }

        #[ink(message)]
        fn approve(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let approval = self.get_allowance(&from, &to);
            self.allowance.insert((from,to), approval + value);
            self.env().emit_event(Approve { from, to, value });
            true
        }

        #[ink(message)]
        fn approval(&self, to: AccountId) -> Balance {
            let from = self.env().caller();
            self.get_allowance(&from, &to)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        //noinspection ALL
        #[test]
        fn total_supply() {
            let erc20 = Erc20::new(666);
            assert_eq!(erc20.total_supply(), 666);
        }
    }
}
