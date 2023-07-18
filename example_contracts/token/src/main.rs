#![no_main]

use borsh::{BorshDeserialize, BorshSerialize};
use spin_sdk::{
    env,
    spin_primitives::{AccountId, FunctionCall},
};

spin_sdk::entrypoint!(entrypoint);

pub fn entrypoint(call: FunctionCall) {
    match call.method.as_str() {
        "init" => {
            init(call.try_deserialize_args().unwrap());
        }
        "mint" => {
            mint(call.try_deserialize_args().unwrap());
        }
        "burn" => {
            burn(call.try_deserialize_args().unwrap());
        }
        "transfer" => {
            transfer(call.try_deserialize_args().unwrap());
        }
        "balance_of" => {
            balance_of(call.try_deserialize_args().unwrap());
        }
        "set_owner" => {
            set_owner(call.try_deserialize_args().unwrap());
        }
        "get_owner" => {
            get_owner();
        }
        _ => {
            panic!("Unknown method name");
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct TokenState {
    ticker: String,
    owner: AccountId,
    total_supply: u128,
    balances: std::collections::HashMap<AccountId, u128>,
}

pub fn init(input: (String, u128)) {
    let ticker = input.0;
    let initial_supply = input.1;

    let mut state = TokenState {
        ticker,
        owner: env::caller().clone(),
        total_supply: initial_supply,
        balances: std::collections::HashMap::new(),
    };

    state.balances.insert(env::caller(), initial_supply);

    env::set_state(state);
}

pub fn balance_of(address: AccountId) {
    let state: TokenState = env::get_state();
    let balance = state.balances.get(&address).unwrap_or(&0);
    env::commit(balance);
}

pub fn mint(amount: u128) {
    let mut state: TokenState = env::get_state();

    if state.owner != env::caller() {
        panic!("Only owner can mint tokens");
    }

    state.total_supply += amount;
    let balance = state.balances.entry(env::caller()).or_insert(0);
    *balance += amount;

    env::set_state(state);
}

pub fn burn(amount: u128) {
    let mut state: TokenState = env::get_state();

    let balance = state.balances.entry(env::caller()).or_insert(0);

    if *balance < amount {
        panic!("Not enough tokens to burn");
    }

    state.total_supply -= amount;
    *balance -= amount;

    env::set_state(state);
}

pub fn transfer(input: (AccountId, u128)) {
    let recipient = input.0;
    let amount = input.1;

    let mut state: TokenState = env::get_state();

    let sender_balance = state.balances.entry(env::caller()).or_insert(0);
    if *sender_balance < amount {
        panic!("Not enough tokens to transfer");
    }
    *sender_balance -= amount;

    let recipient_balance = state.balances.entry(recipient).or_insert(0);
    *recipient_balance += amount;

    env::set_state(state);
}

pub fn set_owner(new_owner: AccountId) {
    let mut state: TokenState = env::get_state();

    if state.owner != env::caller() {
        panic!("Only owner can set new owner");
    }

    state.owner = new_owner;

    env::set_state(state);
}

pub fn get_owner() {
    let state: TokenState = env::get_state();
    env::commit(state.owner);
}
