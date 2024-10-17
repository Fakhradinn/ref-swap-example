use ext::{create_ref_message, ft_contract, ref_contract};
// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, Gas, NearToken, env, PromiseError};
use near_sdk::json_types::U128;

pub mod ext;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    greeting: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }

    pub fn swap_usdc_for_vex(&mut self, amount: U128) {
        ft_contract::ext("usdc.betvex.testnet".parse().unwrap())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(30))
            .ft_transfer_call("ref-finance-101.testnet".parse().unwrap(), amount, "".to_string())
            .then(
                Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(200))
                .ref_transfer_callback()
            );
    }

    #[private]
    pub fn ref_transfer_callback(&mut self, #[callback_result] call_result: Result<U128, PromiseError>,) {
        let amount = call_result.unwrap();

        let action = create_ref_message(
            2197,
            "usdc.betvex.testnet".parse().unwrap(),
            "token.betvex.testnet".parse().unwrap(),
            amount.0,
            0,
        );

        ref_contract::ext("ref-finance-101.testnet".parse().unwrap())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(30))
            .swap(action)
            .then(
                Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(150))
                .ref_swap_callback()
            );
    }

    #[private]
    pub fn ref_swap_callback(&mut self, #[callback_result] call_result: Result<U128, PromiseError>,) {
        let amount = call_result.unwrap();
        
        ref_contract::ext("ref-finance-101.testnet".parse().unwrap())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(30))
            .withdraw("token.betvex.testnet".parse().unwrap(), amount);
    }



}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
}
