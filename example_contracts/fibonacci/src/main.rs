#![no_main]

spin_sdk::entrypoint!(entrypoint);

pub fn fibonacci(n: u32) {
    let (mut a, mut b) = (0u64, 1);
    for _ in 0..n {
        let c = a;
        a = b;
        b += c;
    }
    spin_sdk::commit(a);
}

pub fn entrypoint(call: spin_sdk::FunctionCall) {
    fibonacci(call.try_deserialize_args().unwrap());
}
