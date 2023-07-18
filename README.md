# Spin Blockchain MVP

Simple prototype of cross contract calls in risc0

```sh
cargo +nightly-2023-03-06 -C example_contracts build --release
cp example_contracts/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/demo_ccc_contract spin_core/known_contracts/demo_ccc.spin
cp example_contracts/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/fibonacci_contract spin_core/known_contracts/fibonacci.spin

cargo +nightly-2023-03-06 -C spin_core run --release
```
