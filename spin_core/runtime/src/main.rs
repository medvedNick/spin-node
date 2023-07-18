use std::sync::{Arc, RwLock};

use spin_primitives::AccountId;
use tracing::info;

pub mod context;
pub mod executor;
pub mod syscalls;

fn install_tracing() {
    use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "warn,spin_runtime,spin_primitives=debug".to_owned());
    println!("RUST_LOG={}", filter);

    let main_layer = fmt::layer()
        .event_format(fmt::format().with_ansi(true))
        .with_filter(EnvFilter::from(filter));

    let registry = registry().with(main_layer);

    registry.init();
}

fn main() {
    install_tracing();

    token_init("token.spin", "owner.spin", "SPIN", 100);
    let owner_balance = token_balance_of("token.spin", "owner.spin");
    info!(address = "owner.spin", balance = owner_balance);

    let alice_balance = token_balance_of("token.spin", "alice.spin");
    info!(address = "alice.spin", balance = alice_balance,);

    transfer("token.spin", "owner.spin", "alice.spin", 10);
    let owner_balance = token_balance_of("token.spin", "owner.spin");
    info!(address = "owner.spin", balance = owner_balance,);
    let alice_balance = token_balance_of("token.spin", "alice.spin");
    info!(address = "alice.spin", balance = alice_balance,);

    transfer("token.spin", "alice.spin", "demo_ccc.spin", 7);

    let ctx = Arc::new(RwLock::new(context::ExecutionContext::new(
        AccountId::new("owner.spin".to_string()),
        AccountId::new("alice.spin".to_string()),
        AccountId::new("demo_ccc.spin".to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new(
            "transfer_token".into(),
            ("token.spin", "bob.spin", 5u128),
        ),
    )));

    executor::execute(ctx.clone()).unwrap();
}

fn token_init(token: &str, signer: &str, ticker: &str, initial_supply: u128) {
    info!(
        token,
        owner = signer.to_string(),
        ticker,
        initial_supply,
        "Creating token"
    );
    let ctx = Arc::new(RwLock::new(context::ExecutionContext::new(
        AccountId::new(signer.to_string()),
        AccountId::new(signer.to_string()),
        AccountId::new(token.to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new("init".into(), (ticker, initial_supply)),
    )));

    executor::execute(ctx).unwrap();
}

// fn mint(token: &str, address: &str, amount: u128) {
//     let s = exec_contract::<u64>(ContractCall::new(
//         token.to_string(),
//         "mint".to_string(),
//         address.to_string(),
//         amount,
//     ))
//     .unwrap();
// }

fn transfer(token: &str, from: &str, to: &str, amount: u128) {
    info!(amount, to, from, "Transfering");
    let ctx = Arc::new(RwLock::new(context::ExecutionContext::new(
        AccountId::new(from.to_string()),
        AccountId::new(from.to_string()),
        AccountId::new(token.to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new("transfer".into(), (to, amount)),
    )));

    executor::execute(ctx).unwrap();
}

fn token_balance_of(token: &str, account: &str) -> u64 {
    let ctx = Arc::new(RwLock::new(context::ExecutionContext::new(
        AccountId::new(account.to_string()),
        AccountId::new(account.to_string()),
        AccountId::new(token.to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new("balance_of".into(), account),
    )));

    let s = executor::execute(ctx).unwrap();

    risc0_zkvm::serde::from_slice(&mut s.journal.as_slice()).unwrap()
}
