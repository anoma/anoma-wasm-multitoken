# anoma-wasm-multitoken

> :warning: This repo is work in progress, experimental and shouldn't be relied upon!

Validity predicate and transactions for a variable supply multitoken account. This example is geared around how the validity predicate for [the Ethereum bridge](https://specs.anoma.net/master/architecture/interoperability/ethereum-bridge.html) will work, in that minting of tokens must happen in correspondence to the state of some other account, and burning of tokens will take place on demand by the owner of those tokens.