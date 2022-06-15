# tx_mint_multitoken

This transaction attempts to mint tokens, where the token is part of a multitoken account. The tokens are minted directly to a specified Anoma address' balance.

It does the following change transactionally to storage:

```
multitoken/$token_id/balance/$owner_address += $amount
```

$token_id - the ID of the token which we are minting in the multitoken account
$owner_address - the Anoma address which we are minting the tokens for
$amount - the amount of token to mint

Whether the attempted mint is accepted or not is up to the specific multitoken validity predicate. e.g. mints may only be allowed if the transaction is signed by a certain key, or if there is some other associated state change under some other account's storage space in the same transaction.
