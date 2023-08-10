run:
	cd example_contracts && cargo +nightly-2023-03-06 build --release
	cp example_contracts/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/token_contract spin_core/state/contracts/token.spin
	cp example_contracts/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/fibonacci_contract spin_core/state/contracts/fibonacci.spin
	cd spin_core && cargo +nightly-2023-03-06 run --release --bin erc20