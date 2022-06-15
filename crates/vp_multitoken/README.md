# vp_multitoken

This validity predicate acts to maintain a ledger of multiple tokens under a single Anoma account, specifically Anoma tokens corresponding to ERC20 tokens on the Ethereum blockchain.

## Storage structure
```
/erc20/$token_id/balance/$owner : Amount
```

Storage keys other than ones above may not be written to.

### $token_id
This should be the Ethereum address of an ERC20 token.
### $owner
This can be any arbitrary Anoma bech32m address.

## Invariants

- if a `/erc20/$token_id/balance/$owner` exists, it must be >= 0
- if `/erc20/$token_id/balance/$owner_a` changes by `n: Amount` (n != 0):
  - if n > 0, exactly one of the following:
    - some other `/erc20/$token_id/balance/$owner_b` changes by `-n` (`$owner_a` != `$owner_b`), and the transaction is signed by `$owner_b`
    - there are no other storage changes under this account, and the transaction is signed by the protocol
  - if n < 0, exactly one of the following:
    - some other `/erc20/$token_id/balance/$owner_b` changes by `-n` (`$owner_a` != `$owner_b`), the transaction is signed by `$owner_a`
    - there are no other storage changes under this account, and the transaction is signed by `$owner_a`
