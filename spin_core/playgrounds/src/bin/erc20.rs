use tracing::info;

use spin_primitives::{AccountId, ExecutionCommittment};
use spin_runtime::context::ExecutionContext;
use spin_runtime::executor;

use playgrounds::install_tracing;

use std::sync::{Arc, RwLock};

fn main() {
    install_tracing();

    let abi_path = String::from("./etc/evm_contracts/erc20.abi");
    let bytecode_path = String::from("./etc/evm_contracts/erc20_bytecode");

    let alice = AccountId::new("alice.spin".to_string());
    let alice_evm_address = ExecutionContext::get_account_evm_address(alice.clone());

    let abi = ethabi::Contract::load(std::fs::read(abi_path).unwrap().as_slice()).unwrap();

    // init_evm_accounts();
    // info!("EVM accounts initialized");

    let token_address = deploy_evm_contract(&abi, bytecode_path, &alice);
    info!(?token_address, "token deployed");

    let token_owner = call_evm_contract(&abi, token_address, String::from("owner"), &[], &alice);

    assert!(token_owner[0].clone().into_address().unwrap().0 == alice_evm_address.to_fixed_bytes());
    info!("Token owner is alice");

    let alice_balance = call_evm_contract(
        &abi,
        token_address,
        String::from("balanceOf"),
        &[ethabi::Token::Address(alice_evm_address)],
        &alice,
    );
    info!(?alice_balance, "Alice balance");

    call_evm_contract(
        &abi,
        token_address,
        String::from("mint"),
        &[
            ethabi::Token::Address(alice_evm_address),
            ethabi::Token::Uint(100.into()),
        ],
        &alice,
    );
    info!("Alice minted 100 tokens");

    let alice_balance = call_evm_contract(
        &abi,
        token_address,
        String::from("balanceOf"),
        &[ethabi::Token::Address(alice_evm_address)],
        &alice,
    );
    info!(?alice_balance, "Alice balance");
    assert!(alice_balance[0].clone().into_uint().unwrap() == 100.into());
}

#[allow(dead_code)]
fn init_evm_accounts() {
    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        AccountId::new(String::from("alice.spin")),
        AccountId::new(String::from("alice.spin")),
        AccountId::new("evm".to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new("init".into(), ()),
    )));

    executor::execute(ctx.clone()).unwrap();
}

/// Deploy EVM contract and return its address
fn deploy_evm_contract(
    abi: &ethabi::Contract,
    hex_bytecode_path: String,
    owner_account_id: &AccountId,
) -> eth_primitive_types::H160 {
    let bytecode_file = std::fs::read(hex_bytecode_path).expect("Can't read bytecode");
    let bytecode = hex::decode(bytecode_file).expect("Can't decode bytecode");

    let constructor = abi.constructor().unwrap();
    let constructor_input = constructor.encode_input(bytecode, &[]).unwrap();

    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        owner_account_id.clone(),
        owner_account_id.clone(),
        AccountId::new("evm".to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new("deploy_contract".into(), constructor_input),
    )));

    let s = executor::execute(ctx.clone()).unwrap();
    let committment: ExecutionCommittment =
        borsh::BorshDeserialize::deserialize(&mut s.journal.as_slice()).unwrap();

    let result: ([u8; 20], Vec<u8>) = committment.try_deserialize_output().unwrap();
    let address = eth_primitive_types::H160::from_slice(&result.0);
    info!(address = ?address, "Contract deployed");
    address
}

fn call_evm_contract(
    abi: &ethabi::Contract,
    contract_address: eth_primitive_types::H160,
    function: String,
    args: &[ethabi::Token],
    account_id: &AccountId,
) -> Vec<ethabi::Token> {
    let function = abi.function(&function).unwrap();
    let input = function.encode_input(args).unwrap();

    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        account_id.clone(),
        account_id.clone(),
        AccountId::new("evm".to_string()),
        100_000_000,
        spin_primitives::FunctionCall::new(
            "call_contract".into(),
            (contract_address.to_fixed_bytes(), input),
        ),
    )));

    let s = executor::execute(ctx.clone()).unwrap();
    let committment: ExecutionCommittment =
        borsh::BorshDeserialize::deserialize(&mut s.journal.as_slice()).unwrap();

    let output: Vec<u8> = committment.try_deserialize_output().unwrap();
    function
        .decode_output(output.as_slice())
        .expect("Can't decode output")
}
