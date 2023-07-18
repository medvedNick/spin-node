use anyhow::Result;
use risc0_zkvm::{serde::to_vec, Syscall};
use tracing::debug;

use std::sync::{Arc, RwLock};

use crate::context::ExecutionContext;

pub struct GetEnvCallHandler {
    context: Arc<RwLock<ExecutionContext>>,
}

impl GetEnvCallHandler {
    pub fn new(context: Arc<RwLock<ExecutionContext>>) -> Self {
        Self { context }
    }
}

impl Syscall for GetEnvCallHandler {
    fn syscall(
        &mut self,
        _syscall: &str,
        _ctx: &mut dyn risc0_zkvm::SyscallContext,
        to_guest: &mut [u32],
    ) -> Result<(u32, u32)> {
        let context = self.context.write().unwrap();
        debug!(from_contract=?context.contract(), "handling syscall for env loading");

        let env = context.call_env();

        let output = to_vec(&env.into_bytes()).unwrap();
        to_guest[0..output.len()].copy_from_slice(&output);

        Ok((0, 0))
    }
}
