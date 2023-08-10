use tracing::info;

use spin_primitives::{AccountId, ExecutionOutcome};
use spin_runtime::context::ExecutionContext;
use spin_runtime::executor;

use playgrounds::install_tracing;

use std::sync::{Arc, RwLock};

fn main() {
    install_tracing();

    init_evm_accounts();
    info!("EVM accounts initialized");

    let alice = AccountId::new("alice.spin".to_string());
    let alice_evm_address = ExecutionContext::get_account_evm_address(alice.clone());

    let erc_abi_path = String::from("./etc/evm_contracts/erc20.abi");
    let erc_bytecode_path = String::from("./etc/evm_contracts/erc20_bytecode");
    let erc_abi = ethabi::Contract::load(std::fs::read(erc_abi_path).unwrap().as_slice()).unwrap();
    let token_address = deploy_evm_contract(&erc_abi, erc_bytecode_path, &alice);
    info!(?token_address, "Token deployed");

    let factory_abi_path = String::from("./etc/evm_contracts/factory.abi");
    let factory_bytecode_path = String::from("./etc/evm_contracts/factory_bytecode");
    let factory_abi = ethabi::Contract::load(std::fs::read(factory_abi_path).unwrap().as_slice()).unwrap();
    let factory_address = deploy_evm_contract(&factory_abi, factory_bytecode_path, &alice);
    info!(?factory_address, "Factory deployed");

    let factory_owner = call_evm_contract(
        &factory_abi,
        factory_address,
        String::from("owner"),
        &[],
        &alice
    )[0].clone().into_address().unwrap().0;
    assert_eq!(factory_owner, alice_evm_address.to_fixed_bytes());

    info!("Starting main");
    let vault_abi_path = String::from("./etc/evm_contracts/vault.abi");
    let vault_abi = ethabi::Contract::load(std::fs::read(vault_abi_path).unwrap().as_slice()).unwrap();

    for _ in 0..3 {
        let vault_address = call_evm_contract(
            &factory_abi,
            factory_address,
            String::from("deployVault"),
            &[
                ethabi::Token::String("Vault name".to_string()),
                ethabi::Token::String("VLT".to_string()),
                ethabi::Token::Address(token_address),
                ethabi::Token::Uint(ethabi::Uint::from(18)),
                ethabi::Token::Address(alice_evm_address),
            ],
            &alice,
        )[0].clone().into_address().unwrap();
        info!("Vault {}", vault_address);

        let deposit_token = call_evm_contract(
            &vault_abi,
            vault_address,
            String::from("depositToken"),
            &[],
            &alice
        )[0].clone().into_address().unwrap();
        info!("Deposit token: {}", deposit_token);

        let decimals = call_evm_contract(
            &vault_abi,
            vault_address,
            String::from("decimals"),
            &[],
            &alice
        )[0].clone();
        info!("Decimals: 0x{}", decimals);
    }

    let vault_count = call_evm_contract(
        &factory_abi,
        factory_address,
        String::from("vaultCount"),
        &[],
        &alice
    )[0].clone();
    info!("Vault count: {}", vault_count);
}

#[allow(dead_code)]
fn init_evm_accounts() {
    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        spin_primitives::ContractEntrypointContext::new(
            AccountId::new("evm".to_string()),
            "init".into(),
            (),
            100_000_000,
            AccountId::new(String::from("alice.spin")),
            AccountId::new(String::from("alice.spin")),
        ),
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

    let constructor = abi.constructor().expect("no constructor");
    let constructor_input = constructor.encode_input(bytecode, &[]).unwrap();

    let ctx = Arc::new(RwLock::new(ExecutionContext::new(
        spin_primitives::ContractEntrypointContext::new(
            AccountId::new("evm".to_string()),
            "deploy_contract".into(),
            constructor_input,
            100_000_000,
            owner_account_id.clone(),
            owner_account_id.clone(),
        ),
    )));

    let s = executor::execute(ctx.clone()).unwrap();
    let committment: ExecutionOutcome =
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
        spin_primitives::ContractEntrypointContext::new(
            AccountId::new("evm".to_string()),
            "call_contract".into(),
            (contract_address.to_fixed_bytes(), input),
            100_000_000,
            account_id.clone(),
            account_id.clone(),
        ),
    )));

    let s = executor::execute(ctx.clone()).unwrap();
    let committment: ExecutionOutcome =
        borsh::BorshDeserialize::deserialize(&mut s.journal.as_slice()).unwrap();
    let output: Vec<u8> = committment.try_deserialize_output().unwrap();
    function
        .decode_output(output.as_slice())
        .expect("Can't decode output")
}
