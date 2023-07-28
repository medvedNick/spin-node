pub mod env;
pub use spin_primitives;
pub use spin_sdk_macros::generate_payload;

#[macro_export]
macro_rules! entrypoint {
    ($path:path) => {
        // Type check the given path
        const ZKVM_ENTRY: fn(spin_sdk::spin_primitives::FunctionCall) = $path;

        // Include generated main in a module so we don't conflict
        // with any other definitions of "main" in this file.
        mod zkvm_generated_main {
            #[no_mangle]
            fn main() {
                let contract_call = spin_sdk::spin_primitives::ContractCall::try_from_bytes(
                    risc0_zkvm::guest::env::read(),
                )
                .expect("Corrupted ContractCall");
                spin_sdk::env::setup_env(&contract_call);
                super::ZKVM_ENTRY(contract_call.function_call())
            }
        }
    };
}
