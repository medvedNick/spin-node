use borsh::{BorshDeserialize, BorshSerialize};

use crate::{Digest, StorageKey};

pub const CROSS_CONTRACT_CALL: risc0_zkvm_platform::syscall::SyscallName = unsafe {
    risc0_zkvm_platform::syscall::SyscallName::from_bytes_with_nul(
        concat!("spinvm", "::", "CROSS_CONTRACT_CALL", "\0").as_ptr(),
    )
};

pub const GET_STORAGE_CALL: risc0_zkvm_platform::syscall::SyscallName = unsafe {
    risc0_zkvm_platform::syscall::SyscallName::from_bytes_with_nul(
        concat!("spinvm", "::", "GET_STORAGE", "\0").as_ptr(),
    )
};

pub const SET_STORAGE_CALL: risc0_zkvm_platform::syscall::SyscallName = unsafe {
    risc0_zkvm_platform::syscall::SyscallName::from_bytes_with_nul(
        concat!("spinvm", "::", "SET_STORAGE", "\0").as_ptr(),
    )
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct GetStorageRequest {
    pub key: StorageKey,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct GetStorageResponse {
    pub storage: Option<Vec<u8>>,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct SetStorageRequest {
    pub key: StorageKey,
    pub hash: Digest,
    pub storage: Vec<u8>,
}

pub const GET_ACCOUNT_MAPPING: risc0_zkvm_platform::syscall::SyscallName = unsafe {
    risc0_zkvm_platform::syscall::SyscallName::from_bytes_with_nul(
        concat!("spinvm", "::", "GET_ACCOUNT_MAPPING", "\0").as_ptr(),
    )
};
