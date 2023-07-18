use borsh::BorshDeserialize;
use once_cell::sync::Lazy;
use risc0_zkvm::serde::from_slice;
use std::sync::Mutex;

use spin_primitives::{
    syscalls::{CROSS_CONTRACT_CALL, GET_ENV_CALL, GET_STORAGE_CALL, SET_STORAGE_CALL},
    AccountId, CallEnv, ContractCall,
};

static CALL_ENV: Lazy<Mutex<CallEnv>> = Lazy::new(|| Mutex::new(load_env_syscall()));

/// Loads the call environment from the host.
pub fn load_env_syscall() -> CallEnv {
    let mut response = [0u32; 512]; // TODO: make this dynamic
    risc0_zkvm::guest::env::syscall(GET_ENV_CALL, &[], &mut response);

    let response: Vec<u8> = from_slice(&response).expect("Expected to deserialize");

    BorshDeserialize::deserialize(&mut response.as_slice()).expect("Expected to deserialize")
}

/// Returns the current call signer
pub fn signer() -> AccountId {
    CALL_ENV.lock().unwrap().signer.clone()
}

/// Returns the current call sender
pub fn caller() -> AccountId {
    CALL_ENV.lock().unwrap().caller.clone()
}

/// Returns the current call contract
pub fn contract() -> AccountId {
    CALL_ENV.lock().unwrap().contract.clone()
}

/// Makes a cross-contract call
pub fn cross_contract_call<T: borsh::BorshSerialize, O: borsh::BorshDeserialize>(
    account: AccountId,
    method: String,
    attached_gas: u64,
    args: T,
) -> O {
    let call = ContractCall::new(account, method, args, attached_gas);

    let mut response = [0u32; 32]; // TODO: make this dynamic

    risc0_zkvm::guest::env::syscall(
        CROSS_CONTRACT_CALL,
        call.into_bytes().as_slice(),
        &mut response,
    );

    let response: Vec<u8> =
        risc0_zkvm::serde::from_slice(&response).expect("Expected to deserialize");
    borsh::BorshDeserialize::deserialize(&mut response.as_slice()).expect("Expected to deserialize")
}

pub fn get_state<T: BorshDeserialize>() -> T {
    use risc0_zkvm::sha::rust_crypto::{Digest, Sha256};

    let mut response = [0u32; 1024];

    risc0_zkvm::guest::env::syscall(GET_STORAGE_CALL, &[], &mut response);

    let response: Vec<u8> =
        risc0_zkvm::serde::from_slice(&response).expect("Expected to deserialize");

    let hash = &response[0..32];
    let mut state = &response[32..];

    let algorithm = &mut Sha256::default();
    algorithm.update(&state);
    let hash2 = algorithm.finalize_reset();
    assert!(hash == hash2.as_slice());

    borsh::BorshDeserialize::deserialize(&mut state).expect("Expected to deserialize")
}

pub fn set_state<T: borsh::BorshSerialize>(data: T) {
    use risc0_zkvm::sha::rust_crypto::{Digest, Sha256};

    let state = borsh::BorshSerialize::try_to_vec(&data).expect("Expected to serialize");

    let algorithm = &mut Sha256::default();
    algorithm.update(&state);
    let hash = algorithm.finalize_reset();

    let to_host = [hash.to_vec(), state].concat();

    risc0_zkvm::guest::env::syscall(SET_STORAGE_CALL, &to_host, &mut []);
}

pub fn commit<T: borsh::BorshSerialize>(data: T) {
    risc0_zkvm::guest::env::commit_slice(
        &borsh::BorshSerialize::try_to_vec(&data).expect("Expected to serialize"),
    )
}
