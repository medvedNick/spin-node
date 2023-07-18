use anyhow::Result;
use risc0_zkvm::{serde::to_vec, Syscall};
use tracing::debug;

use std::sync::{Arc, RwLock};

use spin_primitives::ContractCall;

use crate::{context::ExecutionContext, executor};

pub struct CrossContractCallHandler {
    context: Arc<RwLock<ExecutionContext>>,
}

impl CrossContractCallHandler {
    pub fn new(context: Arc<RwLock<ExecutionContext>>) -> Self {
        Self { context }
    }
}

impl Syscall for CrossContractCallHandler {
    fn syscall(
        &mut self,
        _syscall: &str,
        ctx: &mut dyn risc0_zkvm::SyscallContext,
        to_guest: &mut [u32],
    ) -> Result<(u32, u32)> {
        let mut context = self.context.write().unwrap();
        debug!(from_contract=?context.contract(), "handling syscall for cross contract call");

        let buf_ptr = ctx.load_register(risc0_zkvm_platform::syscall::reg_abi::REG_A3);
        let buf_len = ctx.load_register(risc0_zkvm_platform::syscall::reg_abi::REG_A4);
        let from_guest = ctx.load_region(buf_ptr, buf_len);

        let call = ContractCall::try_from_bytes(from_guest).expect("Invalid contract call");

        let ccc_context = context.cross_contract_call(call, 0).unwrap();

        let session = executor::execute(ccc_context).unwrap(); // TODO: handle error

        let output: Vec<u32> = to_vec(&session.journal).unwrap();

        context.set_execution_session(session);

        to_guest[0..output.len()].copy_from_slice(&output);
        Ok((0, 0))
    }
}
