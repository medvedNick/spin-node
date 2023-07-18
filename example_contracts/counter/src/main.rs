#![no_main]

use borsh::{BorshDeserialize, BorshSerialize};

use spin_sdk::env;

spin_sdk::entrypoint!(entrypoint);

pub fn entrypoint(call: spin_sdk::spin_primitives::FunctionCall) {
    match call.method.as_str() {
        "init" => {
            init();
        }
        "get" => {
            get();
        }
        "add" => {
            add();
        }
        _ => {
            panic!("Unknown method name");
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct State {
    value: u64,
}

pub fn init() {
    let state = State { value: 0 };
    env::set_state(state);
}

pub fn get() {
    let state: State = env::get_state();
    env::commit(state);
}

pub fn add() {
    let mut state: State = env::get_state();
    state.value += 1;

    env::set_state(state);
}
