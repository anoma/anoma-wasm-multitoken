# test_runners

Rust binaries which can be run against any Anoma ledger, that will submit a series of transactions and assert results.

The binaries assume the following:

- `$ANOMA_LEDGER_ADDRESS` is set in the environment to the address of the Tendermint RPC
- `.anoma/<chain_id>/` already exists and the chain is already joined
- `anoma`, `anomac`, and `anomaw` are on `$PATH`
