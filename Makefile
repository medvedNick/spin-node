run:
	cd example_contracts/token && cargo build
	cp example_contracts/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/token_contract spin_core/known_contracts/token.spin
	cp example_contracts/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/demo_ccc_contract spin_core/known_contracts/demo_ccc.spin
	cd spin_core && cargo run