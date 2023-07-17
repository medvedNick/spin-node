use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize)]
pub struct ContractCall {
    pub address: String,
    pub function_call: FunctionCall,
}

impl ContractCall {
    pub fn new<T>(address: String, method: String, args: T) -> Self
    where
        T: BorshSerialize,
    {
        Self {
            address,
            function_call: FunctionCall::new(method, args),
        }
    }

    pub fn try_from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> {
        borsh::BorshDeserialize::deserialize(&mut bytes.as_slice())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
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

    pub fn to_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(&self).expect("Expected to serialize")
    }
}

const CROSS_CONTRACT_CALL: risc0_zkvm_platform::syscall::SyscallName = unsafe {
    risc0_zkvm_platform::syscall::SyscallName::from_bytes_with_nul(
        concat!("spinvm", "::", "CROSS_CONTRACT_CALL", "\0").as_ptr(),
    )
};

pub fn cross_contract_call<T: borsh::BorshSerialize, O: borsh::BorshDeserialize>(
    address: String,
    method: String,
    args: T,
) -> O {
    let call = ContractCall::new(address, method, args);
    let bytes = call.to_bytes();

    let mut response = [0u32; 32];

    risc0_zkvm::guest::env::syscall(CROSS_CONTRACT_CALL, bytes.as_slice(), &mut response);

    println!("response: {:?}", response);
    let response: Vec<u8> =
        risc0_zkvm::serde::from_slice(&response).expect("Expected to deserialize");
    borsh::BorshDeserialize::deserialize(&mut response.as_slice()).expect("Expected to deserialize")
}

pub fn commit<T: borsh::BorshSerialize>(data: T) {
    risc0_zkvm::guest::env::commit_slice(
        &borsh::BorshSerialize::try_to_vec(&data).expect("Expected to serialize"),
    )
}

#[macro_export]
macro_rules! entrypoint {
    ($path:path) => {
        // Type check the given path
        const ZKVM_ENTRY: fn(spin_sdk::FunctionCall) = $path;

        // Include generated main in a module so we don't conflict
        // with any other definitions of "main" in this file.
        mod zkvm_generated_main {
            #[no_mangle]
            fn main() {
                let call = spin_sdk::FunctionCall::try_from_bytes(risc0_zkvm::guest::env::read())
                    .expect("Expected to deserialize");
                super::ZKVM_ENTRY(call)
            }
        }
    };
}
