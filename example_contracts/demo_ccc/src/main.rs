#![no_main]

spin_sdk::entrypoint!(entrypoint);

pub fn entrypoint(call: spin_sdk::FunctionCall) {
    match call.method.as_str() {
        "hello" => {
            hello(call.try_deserialize_args().unwrap());
        }
        "fibonacci_and_multiply" => {
            fibonacci_and_multiply(call.try_deserialize_args().unwrap());
        }
        _ => {
            panic!("Unknown method name");
        }
    }
}

pub fn fibonacci_and_multiply(input: (u32, u64)) {
    let n = input.0;
    let multiplier = input.1;

    let result: u64 = spin_sdk::cross_contract_call(
        "fibonacci.spin".to_string(),
        "entypoint".to_string(),
        n as u32,
    );

    let result = result * multiplier;

    spin_sdk::commit(result);
}

pub fn hello(name: String) {
    let result = format!("Hello, {}!", name);
    spin_sdk::commit(result);
}
