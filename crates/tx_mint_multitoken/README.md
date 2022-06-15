# tx_mint_multitoken

This transaction attempts to mint tokens, where the token is part of a multitoken account. The tokens are minted directly to a specified Anoma account's balance.

It does the following change transactionally to storage:

```
#multitoken_address/$multitoken_key/$token_id/balance/$owner_address += $amount
```

See the `MintMultitoken` struct in [src/data.rs](./src/data.rs) for details of the parameters to be passed in the transaction data.

Whether the attempted mint is accepted or not is up to the specific multitoken validity predicate. e.g. mints may only be allowed if the transaction is signed by a certain key, or if there is some other associated state change under some other account's storage space in the same transaction.
