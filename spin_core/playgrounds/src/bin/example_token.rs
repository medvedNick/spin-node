fn main() {}
// use tracing::info;

// use spin_primitives::{AccountId, ExecutionOutcome};
// use spin_runtime::context::ExecutionContext;
// use spin_runtime::executor;

// use playgrounds::install_tracing;

// use std::sync::{Arc, RwLock};

// fn main() {
//     install_tracing();

//     let token = AccountId::new("token.spin".to_string());
//     let alice = AccountId::new("alice.spin".to_string());
//     let bob = AccountId::new("bob.spin".to_string());

//     token_init(&token, &alice, String::from("SPIN"), 100);

//     let alice_balance = token_balance_of(&token, &alice);
//     info!(address = ?alice, balance = alice_balance);

//     let bob_balance = token_balance_of(&token, &bob);
//     info!(address = ?bob, balance = bob_balance);

//     transfer(&token, &alice, &bob, 10);

//     let alice_balance = token_balance_of(&token, &alice);
//     info!(address = ?alice, balance = alice_balance);

//     let bob_balance = token_balance_of(&token, &bob);
//     info!(address = ?bob, balance = bob_balance);
// }

// fn token_init(token: &AccountId, signer: &AccountId, ticker: String, initial_supply: u128) {
//     info!(
//         ?token,
//         owner = ?signer,
//         ticker,
//         initial_supply,
//         "Creating token"
//     );
//     let ctx = Arc::new(RwLock::new(ExecutionContext::new(
//         spin_primitives::ContractCallWithContext::new(
//             token.clone(),
//             "init".into(),
//             (ticker, initial_supply),
//             100_000_000,
//             signer.clone(),
//             signer.clone(),
//         ),
//     )));

//     executor::execute(ctx).unwrap();
// }

// fn transfer(token: &AccountId, from: &AccountId, to: &AccountId, amount: u128) {
//     info!(amount, ?to, ?from, "Transfering");
//     let ctx = Arc::new(RwLock::new(ExecutionContext::new(
//         spin_primitives::ContractCallWithContext::new(
//             token.clone(),
//             "transfer".into(),
//             (to, amount),
//             100_000_000,
//             from.clone(),
//             from.clone(),
//         ),
//     )));

//     executor::execute(ctx).unwrap();
// }

// fn token_balance_of(token: &AccountId, account: &AccountId) -> u64 {
//     let ctx = Arc::new(RwLock::new(ExecutionContext::new(
//         spin_primitives::ContractCallWithContext::new(
//             token.clone(),
//             "balance_of".into(),
//             account,
//             100_000_000,
//             account.clone(),
//             account.clone(),
//         ),
//     )));

//     let s = executor::execute(ctx.clone()).unwrap();

//     let committment: ExecutionOutcome =
//         borsh::BorshDeserialize::deserialize(&mut s.journal.as_slice()).unwrap();

//     let balance: u64 = committment.try_deserialize_output().unwrap();
//     balance
// }
