#![no_main]

spin_sdk::entrypoint!(entrypoint);

use spin_sdk::env;

pub fn fibonacci(n: u32) {
    let (mut a, mut b) = (0u64, 1);
    for _ in 0..n {
        let c = a;
        a = b;
        b += c;
    }
    env::commit(a);
}

pub fn entrypoint(call: spin_sdk::spin_primitives::FunctionCall) {
    fibonacci(call.try_deserialize_args().unwrap());
}
