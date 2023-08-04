use risc0_recursion::{lift, join};
use risc0_zkvm::prove::get_prover;
use tracing::info;

use spin_primitives::{AccountId, ExecutionCommittment};
use spin_runtime::context::ExecutionContext;
use spin_runtime::executor;

use playgrounds::install_tracing;

use std::sync::{Arc, RwLock};

fn main() {
    install_tracing();

    let fib = AccountId::new("fibonacci.spin".to_string());
    let alice = AccountId::new("alice.spin".to_string());

    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        AccountId::new(alice.to_string()),
        AccountId::new(alice.to_string()),
        AccountId::new(fib.to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new("fibonacci".into(), 500_000u32),
    )));
    
    info!("executing...");    
    let session = executor::execute(ctx).unwrap();

    info!("got {} segments...", session.segments.len());
    let verifier_ctx = risc0_zkvm::VerifierContext::default();
    let segments = session.resolve().unwrap();
    let prover = get_prover("$poseidon");

    info!("proving first segment...");
    let first_receipt = prover.prove_segment(&verifier_ctx, &segments[0]).unwrap();
    info!("lifting first segment...");
    let mut rollup = lift(&first_receipt).unwrap();

    for receipt in &segments[1..] {
        info!("proving next segment...");
        let segment_receipt = prover.prove_segment(&verifier_ctx, &receipt).unwrap();
        info!("lifting next segment...");
        let rec_receipt = lift(&segment_receipt).unwrap();
        info!("joining next segment...");
        rollup = join(&rollup, &rec_receipt).unwrap();
    }
    info!("verifying...");
    let result = rollup.verify_with_context(&verifier_ctx);
    info!("verified: {:#?}", result);
    info!("finished!");
}

fn token_init(token: &AccountId, signer: &AccountId, ticker: String, initial_supply: u128) {
    info!(
        ?token,
        owner = ?signer,
        ticker,
        initial_supply,
        "Creating token"
    );
    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        signer.clone(),
        signer.clone(),
        token.clone(),
        100_000_000,
        spin_primitives::FunctionCall::new("init".into(), (ticker, initial_supply)),
    )));

    executor::execute(ctx).unwrap();
}

fn transfer(token: &AccountId, from: &AccountId, to: &AccountId, amount: u128) {
    info!(amount, ?to, ?from, "Transfering");
    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        AccountId::new(from.to_string()),
        AccountId::new(from.to_string()),
        AccountId::new(token.to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new("transfer".into(), (to, amount)),
    )));

    executor::execute(ctx).unwrap();
}

fn token_balance_of(token: &AccountId, account: &AccountId) -> u64 {
    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        account.clone(),
        account.clone(),
        token.clone(),
        100_000_000,
        spin_primitives::FunctionCall::new("balance_of".into(), account),
    )));

    let s = executor::execute(ctx.clone()).unwrap();

    let committment: ExecutionCommittment =
        borsh::BorshDeserialize::deserialize(&mut s.journal.as_slice()).unwrap();

    let balance: u64 = committment.try_deserialize_output().unwrap();
    balance
}
