# Spin ZKP Blockchain Node MVP

Spin ZKP Blockchain Node is our blockchain prototype, which utilizes zero-knowledge proofs (via [risc0](https://github.com/risc0/risc0)) for all state transitions.

Architecture description, playground cases, tech details and other documentation could be found on [Spin Node Wiki](https://github.com/spin-fi/spin-node/wiki)

## Structure

- `spin_core` - the core of the node, currently contains prototype of the runtime.
- `spin_sdk` - SDK for writing contracts.
- `example_contracts` - example contracts written using the SDK.

## Playgrounds

### Run

Build example contracts and copy them to the state directory.
```sh
cd example_contracts
cargo +nightly-2023-03-06 build --release
cp target/riscv-guest/riscv32im-risc0-zkvm-elf/release/token_contract ../spin_core/state/contracts/token.spin
cp target/riscv-guest/riscv32im-risc0-zkvm-elf/release/demo_ccc_contract ../spin_core/state/contracts/demo_ccc.spin
cd ..
```

Run the playground.
```sh
cd spin_core

cargo +nightly-2023-03-06 run --release --bin erc20
# or
cargo +nightly-2023-03-06 run --release --bin example_token
```
