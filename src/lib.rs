use ext::{create_ref_message, ft_contract, ref_contract};
use near_sdk::json_types::U128;
use near_sdk::{env, log, near, AccountId, Gas, NearToken, PromiseError, PanicOnDefault};

pub mod ext;

// Define the contract structure
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    ref_contract: AccountId,
    pool_id: u64,
    ft_contract_1: AccountId,
    ft_contract_2: AccountId,
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn init(
        ref_contract: AccountId,
        pool_id: u64,
        ft_contract_1: AccountId,
        ft_contract_2: AccountId,
    ) -> Self {
        Self {
            ref_contract,
            pool_id,
            ft_contract_1,
            ft_contract_2,
        }
    }

    pub fn swap_in_ref_pool(&mut self, amount: U128) {
        // Deposit the amount to ref finance you want to swap
        ft_contract::ext(self.ft_contract_1.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(30))
            .ft_transfer_call(self.ref_contract.clone(), amount, "".to_string())
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(200))
                    .ref_transfer_callback(),
            );
    }

    // If the cross contract call to deposit fails then you need to rollback any state changes in the swap_in_ref_pool method
    #[private]
    pub fn ref_transfer_callback(
        &mut self,
        #[callback_result] call_result: Result<U128, PromiseError>,
    ) {
        if call_result.is_err() {
            // Rollback state here
            log!(
                "Deposit to ref finance failed {:?}",
                call_result.unwrap_err()
            );
            return;
        }

        let amount_deposited = call_result.unwrap();

        // Prepare the action to swap the tokens
        let action = create_ref_message(
            self.pool_id,
            self.ft_contract_1.clone(),
            self.ft_contract_2.clone(),
            amount_deposited,
            0,
        );

        // Call the ref contract to swap the tokens
        ref_contract::ext(self.ref_contract.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(30))
            .swap(action)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(150))
                    .ref_swap_callback(),
            );
    }

    // If the cross contract call to swap fails then you need to rollback any state changes in the ref_transfer_callback method
    // But not rollback state in the swap_in_ref pool method since the deposit has already happened, you should have another
    // method that allows you to proceed with the swap if the deposit is successful
    // but there shouldn't be much to cause an error at the swap stage
    #[private]
    pub fn ref_swap_callback(
        &mut self,
        #[callback_result] call_result: Result<U128, PromiseError>,
    ) {
        if call_result.is_err() {
            // Rollback state here
            log!(
                "Swap in ref finance failed {:?}",
                call_result.unwrap_err()
            );
            return;
        }

        let amount_can_withdraw = call_result.unwrap();

        // Call the ref contract to withdraw the tokens
        ref_contract::ext(self.ref_contract.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(30))
            .withdraw(self.ft_contract_2.clone(), amount_can_withdraw)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(100))
                    .ref_withdraw_callback(),
            );
    }

    // If the cross contract call to withdraw fails then you need to rollback any state changes in the ref_swap_callback method
    // But not rollback state in the previous two methods since the previous cross contract calls were successful
    // you should have another method that allows you to proceed with the withdraw if the swap is successful
    // but there shouldn't be much to cause an error at the withdraw stage
    #[private]
    pub fn ref_withdraw_callback(
        &mut self,
        #[callback_result] call_result: Result<U128, PromiseError>,
    ) -> U128 {
        if call_result.is_err() {
            // Rollback state here
            log!(
                "Withdraw from ref finance failed {:?}",
                call_result.unwrap_err()
            );
            return U128(0);
        }

        let amount_withdrew = call_result.unwrap();

        amount_withdrew
    }
}
