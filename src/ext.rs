use near_sdk::json_types::U128;
use near_sdk::{ext_contract, near, AccountId, PromiseOrValue};

#[near(serializers = [json])]
pub struct Action {
    pool_id: u64,
    token_in: AccountId,
    token_out: AccountId,
    amount_in: U128,
    min_amount_out: U128,
}

pub fn create_ref_message(
    pool_id: u64,
    token_in: AccountId,
    token_out: AccountId,
    amount_in: u128,
    min_amount_out: u128,
) -> Vec<Action> {
    // Create the RefInnerMsg instance
    let action = Action {
        pool_id,
        token_in,
        token_out,
        amount_in: U128(amount_in),
        min_amount_out: U128(min_amount_out),
    };

    vec![action]
}

// FT transfer interface
#[allow(dead_code)]
#[ext_contract(ft_contract)]
trait FT {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128);

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

#[allow(dead_code)]
#[ext_contract(ref_contract)]
trait Ref {
    fn swap(&mut self, actions: Vec<Action>) -> U128;

    fn withdraw(&mut self, token_id: AccountId, amount: U128) -> U128;
}
