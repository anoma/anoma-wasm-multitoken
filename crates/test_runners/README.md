# test_runners

End-to-end tests as Rust binaries which can be run against any Anoma chain.

The binaries need the following in the environment

- `$ANOMA_CHAIN_ID` is the chain ID
- `$ANOMA_LEDGER_ADDRESS` is the address of a Tendermint RPC endpoint for the chain
- `$ANOMA_NETWORK_CONFIGS_SERVER` is set to the URL of the network configs server (if not the default)

Also, `anoma`, `anomac`, and `anomaw` must be on `$PATH`.
