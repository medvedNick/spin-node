use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub mod syscalls;

#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
pub struct ContractCall {
    pub account: AccountId,
    pub function_call: FunctionCall,
    pub attached_gas: u64,
}

impl ContractCall {
    pub fn new<T>(account: AccountId, method: String, args: T, attached_gas: u64) -> Self
    where
        T: BorshSerialize,
    {
        Self {
            account,
            function_call: FunctionCall::new(method, args),
            attached_gas,
        }
    }

    pub fn try_from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> {
        borsh::BorshDeserialize::deserialize(&mut bytes.as_slice())
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(&self).expect("Expected to serialize")
    }
}

#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
pub struct FunctionCall {
    pub method: String,
    pub args: Vec<u8>,
}

impl FunctionCall {
    pub fn new<T>(method: String, args: T) -> Self
    where
        T: BorshSerialize,
    {
        Self {
            method,
            args: args.try_to_vec().expect("Expected to serialize"),
        }
    }

    pub fn try_from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> {
        borsh::BorshDeserialize::deserialize(&mut bytes.as_slice())
    }

    pub fn try_deserialize_args<T: BorshDeserialize>(&self) -> std::io::Result<T> {
        borsh::BorshDeserialize::deserialize(&mut self.args.as_slice())
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(&self).expect("Expected to serialize")
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

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
pub struct CallEnv {
    pub signer: AccountId,
    pub caller: AccountId,
    pub contract: AccountId,
    pub attached_gas: u64,
}

impl CallEnv {
    pub fn into_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(&self).expect("Expected to serialize")
    }
}
