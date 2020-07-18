#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;


    #[ink(storage)]
    struct Erc20{
        total_supply: storage::Value<Balance>,
        balances: storage::HashMap<AccountId, Balance>,
        allowence: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transfer{
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value:Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, inital_supply:Balance){
            let caller = self.env().caller();
            self.total_supply.set(inital_supply);
            self.balances.insert(caller,inital_supply);
            self.env().emit_event(
                Transfer{
                    from:None,
                    to:Some(caller),
                    value:inital_supply,
                }
            );
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
           * self.total_supply
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        #[ink(message)]
        fn balance_of(&self, owner:AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        fn approve(&mut self, to:AccountId, value: Balance) -> bool{
            let from_account = self.env().caller();
            let from_balance =self.balance_of_or_zero(&from_account);
            if from_balance < value
            {
                return false;
            }
            self.allowence.insert((from_account,to),value);
            true

        }

        #[ink(message)]
        fn approval(&self, to: AccountId) -> Balance{
            let caller = self.env().caller();
            *self.allowence.get(&(caller, to )).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value:Balance) -> bool{
            let from_account = self.env().caller();
            let from_balance =self.balance_of_or_zero(&from_account);
            if from_balance < value
            {
                return false;
            }
            let recv_balance = self.balance_of_or_zero(&to);
            self.balances.insert(from_account, from_balance - value);
            self.balances.insert(to, recv_balance + value);
            self.env().emit_event(
                Transfer{
                    from:Some(from_account),
                    to:Some(to),
                    value:value,
                }
            );
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_works(){
            let erc20 = Erc20::new(666);
            assert_eq!(erc20.total_supply, 666);
        }

        #[test]
        fn balances_of_works(){
            let erc20 = Erc20::new(1000);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 1000);
        }

        #[test]
        fn tranfer_works() {
            let mut erc20 = Erc20::new(999);
            assert_eq!(erc20.transfer(AccountId::from([0x9; 32]),100), true);
            assert_eq!(erc20.balance_of(AccountId::from([0x9; 32])), 100);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 899);
        }
       
    }





}