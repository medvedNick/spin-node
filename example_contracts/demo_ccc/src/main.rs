#![no_main]

use spin_sdk::{env, spin_primitives::AccountId};

spin_sdk::entrypoint!(entrypoint);

pub fn entrypoint(call: spin_sdk::spin_primitives::FunctionCall) {
    match call.method.as_str() {
        "hello" => {
            hello(call.try_deserialize_args().unwrap());
        }
        "fibonacci_and_multiply" => {
            fibonacci_and_multiply(call.try_deserialize_args().unwrap());
        }
        "transfer_token" => {
            transfer_token(call.try_deserialize_args().unwrap());
        }
        _ => {
            panic!("Unknown method name");
        }
    }
}

pub fn fibonacci_and_multiply(input: (u32, u64)) {
    let n = input.0;
    let multiplier = input.1;

    let result: u64 = env::cross_contract_call(
        AccountId::new("fibonacci.spin".to_string()),
        "entypoint".to_string(),
        10_000,
        n as u32,
    );

    let result = result * multiplier;

    env::commit(result);
}

pub fn transfer_token(input: (AccountId, AccountId, u128)) {
    let token_account = input.0;
    let recipient = input.1;
    let amount = input.2;

    let _: () = env::cross_contract_call(
        token_account,
        "transfer".to_string(),
        100_000_000,
        (recipient, amount),
    );
}

pub fn hello(name: String) {
    let result = format!("Hello, {}!", name);
    env::commit(result);

    risc0_zkvm::guest::env::read()
}
