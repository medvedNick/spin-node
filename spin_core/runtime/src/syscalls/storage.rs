use anyhow::Result;
use risc0_zkvm::sha::rust_crypto::{Digest, Sha256};
use risc0_zkvm::{serde::to_vec, Syscall};
use tracing::debug;

use std::sync::{Arc, RwLock};

use crate::context::ExecutionContext;

fn bytes_to_hex_string(slice: &[u8]) -> String {
    slice.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub struct GetStorageCallHandler {
    context: Arc<RwLock<ExecutionContext>>,
}

impl GetStorageCallHandler {
    pub fn new(context: Arc<RwLock<ExecutionContext>>) -> Self {
        Self { context }
    }
}

impl Syscall for GetStorageCallHandler {
    fn syscall(
        &mut self,
        _syscall: &str,
        _ctx: &mut dyn risc0_zkvm::SyscallContext,
        to_guest: &mut [u32],
    ) -> Result<(u32, u32)> {
        let context = self.context.write().unwrap();
        debug!(from_contract=?context.contract(), "handling syscall for storage loading");

        // TODO: don't use files as contract state storage xD
        let state: Vec<u8> = std::fs::read(format!("./state/{}", context.contract().to_string()))
            .unwrap_or_else(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    debug!("No state found for {:?}, creating new", context.contract());
                    Vec::new()
                }
                _ => todo!(),
            });

        let algorithm = &mut Sha256::default();
        algorithm.update(&state);
        let hash = algorithm.finalize_reset();

        debug!(
            hash = bytes_to_hex_string(hash.as_slice()),
            account = ?context.contract(),
            "Loading contract state"
        );

        let output = [hash.to_vec(), state].concat();

        let output = to_vec(&output).unwrap();
        to_guest[0..output.len()].copy_from_slice(&output);

        Ok((0, 0))
    }
}

pub struct SetStorageCallHandler {
    context: Arc<RwLock<ExecutionContext>>,
}

impl SetStorageCallHandler {
    pub fn new(context: Arc<RwLock<ExecutionContext>>) -> Self {
        Self { context }
    }
}

impl risc0_zkvm::Syscall for SetStorageCallHandler {
    fn syscall(
        &mut self,
        _syscall: &str,
        ctx: &mut dyn risc0_zkvm::SyscallContext,
        _to_guest: &mut [u32],
    ) -> Result<(u32, u32)> {
        let context = self.context.write().unwrap();
        debug!(from_contract=?context.contract(), "handling syscall for storage update");

        let buf_ptr = ctx.load_register(risc0_zkvm_platform::syscall::reg_abi::REG_A3);
        let buf_len = ctx.load_register(risc0_zkvm_platform::syscall::reg_abi::REG_A4);
        let from_guest = ctx.load_region(buf_ptr, buf_len);

        let hash = &from_guest[0..32];
        let state = &from_guest[32..];

        let algorithm = &mut Sha256::default();
        algorithm.update(state);
        let hash2 = algorithm.finalize_reset();
        assert_eq!(hash, hash2.as_slice());

        debug!(
            new_hash = bytes_to_hex_string(hash),
            address = ?context.contract(),
            "Updating contract state"
        );

        // TODO: don't use files as contract state storage xD
        std::fs::write(format!("./state/{}", context.contract().to_string()), state).unwrap();

        Ok((0, 0))
    }
}
