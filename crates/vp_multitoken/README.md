# vp_multitoken

This validity predicate keeps track of a multitoken structured like:

```
# $owner: Address
# $multitoken_id: String

/multitoken/$multitoken_id/balance/$owner : Amount
```

Other storage key changes are not allowed.

It maintains the following invariants:

- every `$multitoken_id/balance/$owner` >= 0, always
- if a `$multitoken_id/balance/$owner_a` changes by `n: Amount` (n != 0):
  - if n > 0, exactly one of the following:
    - some `$multitoken_id/balance/$owner_b` changes by `-n` (`$owner_a` != `$owner_b`), the transaction is signed by `$owner_b`
  - if n < 0, exactly one of the following:
    - some `$multitoken_id/balance/$owner_b` changes by `-n` (`$owner_a` != `$owner_b`), the transaction is signed by `$owner_a`