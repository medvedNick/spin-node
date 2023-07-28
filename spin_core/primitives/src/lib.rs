use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub mod syscalls;

#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
pub struct ContractCall {
    pub account: AccountId,
    pub method: String,
    pub args: Vec<u8>,
    pub attached_gas: u64,
    pub sender: AccountId,
    pub signer: AccountId,
}

impl ContractCall {
    pub fn new<T: BorshSerialize>(
        account: AccountId,
        method: String,
        args: T,
        attached_gas: u64,
        sender: AccountId,
        signer: AccountId,
    ) -> Self
    where
        T: BorshSerialize,
    {
        Self {
            account,
            method,
            args: BorshSerialize::try_to_vec(&args).expect("Expected to serialize"),
            attached_gas,
            sender,
            signer,
        }
    }

    pub fn try_from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> {
        borsh::BorshDeserialize::deserialize(&mut bytes.as_slice())
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(&self).expect("Expected to serialize")
    }

    pub fn function_call(&self) -> FunctionCall {
        FunctionCall {
            method: self.method.clone(),
            args: self.args.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
pub struct FunctionCall {
    pub method: String,
    pub args: Vec<u8>,
}

impl FunctionCall {
    pub fn try_deserialize_args<T: BorshDeserialize>(&self) -> std::io::Result<T> {
        borsh::BorshDeserialize::deserialize(&mut self.args.as_slice())
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    BorshSerialize,
    BorshDeserialize,
    Clone,
    PartialEq,
    Hash,
    PartialOrd,
    Eq,
)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn new_evm(address: eth_primitive_types::H160) -> Self {
        Self(format!("{:?}.evm", address))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<eth_primitive_types::H160> for AccountId {
    fn from(address: eth_primitive_types::H160) -> Self {
        Self::new_evm(address)
    }
}

pub type Digest = [u8; 32];

pub type StorageKey = String;

/// Execution outcome of a contract call.
#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
pub struct ExecutionOutcome {
    pub call_hash: Digest,
    pub output: Vec<u8>,
    pub storage_reads: HashMap<StorageKey, Digest>,
    pub storage_writes: HashMap<StorageKey, Digest>,
    pub cross_calls_hashes: Vec<(Digest, Digest)>, // hashes of call and output of cross calls
}

impl ExecutionOutcome {
    pub fn try_from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> {
        borsh::BorshDeserialize::deserialize(&mut bytes.as_slice())
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(&self).expect("Expected to serialize")
    }

    pub fn try_deserialize_output<T: BorshDeserialize>(&self) -> std::io::Result<T> {
        borsh::BorshDeserialize::deserialize(&mut self.output.as_slice())
    }
}
