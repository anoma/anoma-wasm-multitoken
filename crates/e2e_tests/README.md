# e2e_tests

End-to-end tests as Rust binaries which can be run against any Anoma chain.

The binaries need the following in the environment

- `$NAMADA_CHAIN_ID` is the chain ID (or the chain ID can be in a file named `chain-id`)
- `$NAMADA_LEDGER_ADDRESS` is the address of a Tendermint RPC endpoint for the chain
- `$NAMADA_NETWORK_CONFIGS_SERVER` is set to the URL of the network configs server (if not the default)

Also, `namada`, `namadac`, and `namadaw` must be on `$PATH`.
