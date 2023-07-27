use borsh::BorshDeserialize;
use once_cell::sync::Lazy;
use risc0_zkvm::{
    serde::from_slice,
    sha::rust_crypto::{Digest, Sha256},
};
use std::sync::Mutex;

use spin_primitives::{
    syscalls::{
        GetStorageResponse, SetStorageRequest, CROSS_CONTRACT_CALL, GET_ACCOUNT_MAPPING,
        GET_ENV_CALL, GET_STORAGE_CALL, SET_STORAGE_CALL,
    },
    AccountId, CallEnv, ContractCall, ExecutionCommittment,
};

static CALL_ENV: Lazy<Mutex<CallEnv>> = Lazy::new(|| Mutex::new(load_env_syscall()));

// TODO: I broke this when I added support for multi-key storage, will fix later
// static INITIAL_STATE_HASH: Lazy<Mutex<Option<[u8; 32]>>> = Lazy::new(|| Mutex::new(None));
// static FINAL_STATE: Lazy<Mutex<Option<Vec<u8>>>> = Lazy::new(|| Mutex::new(None));

static CROSS_CALLS_HASHES: Lazy<Mutex<Vec<[u8; 32]>>> = Lazy::new(|| Mutex::new(Vec::new()));

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

    let algorithm = &mut risc0_zkvm::sha::rust_crypto::Sha256::default();
    algorithm.update(&response);
    let response_hash = algorithm.finalize_reset().as_slice().try_into().unwrap();
    CROSS_CALLS_HASHES.lock().unwrap().push(response_hash);

    ExecutionCommittment::try_from_bytes(response)
        .unwrap()
        .try_deserialize_output()
        .unwrap()
}

pub fn get_state<T: BorshDeserialize>(key: String) -> Option<T> {
    let mut response = [0u32; 65536];

    risc0_zkvm::guest::env::syscall(GET_STORAGE_CALL, &key.into_bytes(), &mut response);

    let response: Vec<u8> =
        risc0_zkvm::serde::from_slice(&response).expect("Expected to deserialize");

    let response: GetStorageResponse =
        BorshDeserialize::try_from_slice(&mut response.as_slice()).unwrap();

    // INITIAL_STATE_HASH
    //     .lock()
    //     .unwrap()
    //     .replace(response.state.clone().try_into().unwrap());

    let algorithm = &mut Sha256::default();
    algorithm.update(&response.state.clone());
    let hash2 = algorithm.finalize_reset();
    assert!(response.hash == hash2.as_slice());

    if response.state.is_empty() {
        return None;
    } else {
        Some(
            borsh::BorshDeserialize::deserialize(&mut response.state.as_slice())
                .expect("Expected to deserialize"),
        )
    }
}

pub fn set_state<T: borsh::BorshSerialize>(key: String, data: T) {
    let state = borsh::BorshSerialize::try_to_vec(&data).expect("Expected to serialize");

    let algorithm = &mut Sha256::default();
    algorithm.update(&state);
    let hash = algorithm.finalize_reset();

    let request = SetStorageRequest {
        key: key.clone(),
        hash: hash.as_slice().try_into().unwrap(),
        state: state.clone(),
    };

    let to_host = borsh::BorshSerialize::try_to_vec(&request).expect("Expected to serialize");

    risc0_zkvm::guest::env::syscall(SET_STORAGE_CALL, &to_host, &mut []);
}

// pub fn set_final_state<T: borsh::BorshSerialize>(state: T) {
//     let mut final_state = FINAL_STATE.lock().unwrap();
//     *final_state = Some(borsh::BorshSerialize::try_to_vec(&state).expect("Expected to serialize"));
// }

pub fn commit<T: borsh::BorshSerialize>(output: T) {
    let output = borsh::BorshSerialize::try_to_vec(&output).expect("Expected to serialize");

    // let final_state_hash = FINAL_STATE.lock().unwrap().clone().map(|state| {
    //     let algorithm = &mut risc0_zkvm::sha::rust_crypto::Sha256::default();
    //     algorithm.update(state);
    //     algorithm.finalize_reset().as_slice().try_into().unwrap()
    // });

    // let initial_state_hash = INITIAL_STATE_HASH.lock().unwrap().clone();

    let cross_calls_hashes = CROSS_CALLS_HASHES.lock().unwrap().clone();

    let committment = ExecutionCommittment {
        output,
        cross_calls_hashes,
        initial_state_hash: None,
        final_state_hash: None,
    };

    risc0_zkvm::guest::env::commit_slice(
        &borsh::BorshSerialize::try_to_vec(&committment).expect("Expected to serialize"),
    )
}

/// Get EVM address by AccountId
pub fn get_evm_address(account_id: AccountId) -> eth_primitive_types::H160 {
    let mut response = [0u32; 20];

    risc0_zkvm::guest::env::syscall(
        GET_ACCOUNT_MAPPING,
        account_id.to_string().as_bytes(),
        &mut response,
    );

    let response: [u8; 20] =
        risc0_zkvm::serde::from_slice(&response).expect("Expected to deserialize");

    eth_primitive_types::H160::from_slice(&response)
}
