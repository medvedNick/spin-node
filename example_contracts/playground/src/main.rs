#![no_main]

use spin_sdk::env;

spin_sdk::entrypoint!(entrypoint);

pub fn entrypoint(call: spin_sdk::spin_primitives::FunctionCall) {
    match call.method.as_str() {
        "hello" => {
            hello();
        }
        _ => {
            panic!("Unknown method name");
        }
    }
}

pub fn hello() {
    let output = format!("Hello, {}!", env::signer().to_string(),);
    env::commit(output);
}
