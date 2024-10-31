# ref-swap-example

> [!WARNING]  
> This is just a basic example and needs modifying to be used on mainnet.

This is a basic example of how to make a swap in a ref finance pool in a contract that outputs the amount of tokens received. You can make a swap in one call, however, you will not get the amount of tokens received. 

If you want to get a certain amount of tokens out you cannot reliably do this, you can make a call to predict how many tokens though but someone can swap in the pool inbetween the callback. 

Remember to register the account in the tokens, the account in ref finance and to register the tokens in the exchange for the account the tokens if the tokens are not whitelisted. You can find more info here https://guide.ref.finance/developers-1/cli-trading. The contract account must also have some amount of `ft_contract_1` to make the swap, in this example the user does not deposit any tokens.

Ref finance testnet contract is `ref-finance-101.testnet`.

Deploy and init

```bash
cargo near deploy --no-docker <contractId> with-init-call init json-args '{"ref_contract":"ref-finance-101.testnet","pool_id":2197,"ft_contract_1":"usdc.betvex.testnet","ft_contract_2":"token.betvex.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-legacy-keychain send
```

Call function

```bash
near contract call-function as-transaction alike-talk.testnet swap_in_ref_pool json-args '{"amount": "1000"}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as pivortex.testnet network-config testnet sign-with-legacy-keychain send
```