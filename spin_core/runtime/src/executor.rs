use std::sync::{Arc, RwLock};

use anyhow::{Context, Result};
use risc0_zkvm::{serde::to_vec, Executor, ExecutorEnv};
use spin_primitives::{
    syscalls::{CROSS_CONTRACT_CALL, GET_ENV_CALL, GET_STORAGE_CALL, SET_STORAGE_CALL},
    AccountId,
};
use tracing::debug;

use crate::syscalls::{cross_contract::CrossContractCallHandler, env::GetEnvCallHandler};
use crate::{
    context::ExecutionContext,
    syscalls::storage::{GetStorageCallHandler, SetStorageCallHandler},
};

const MAX_MEMORY: u32 = 0x10000000;
const PAGE_SIZE: u32 = 0x400;

fn load_contract(account: AccountId) -> Result<Vec<u8>> {
    std::fs::read(format!("./known_contracts/{}", &account.to_string()))
        .with_context(|| format!("Can't read contract {}", account.to_string()))
}

pub fn execute(context: Arc<RwLock<ExecutionContext>>) -> Result<risc0_zkvm::Session> {
    let mut exec = {
        let ctx = context.read().unwrap();
        debug!(contract = ?ctx.contract(), "Executing contract");

        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&ctx.call().into_bytes())?)
            .session_limit(Some(8192 * 1024 * 1024))
            .syscall(GET_ENV_CALL, GetEnvCallHandler::new(context.clone()))
            .syscall(
                CROSS_CONTRACT_CALL,
                CrossContractCallHandler::new(context.clone()),
            )
            .syscall(
                GET_STORAGE_CALL,
                GetStorageCallHandler::new(context.clone()),
            )
            .syscall(
                SET_STORAGE_CALL,
                SetStorageCallHandler::new(context.clone()),
            )
            .build()?;

        let elf = load_contract(ctx.contract().clone())
            .context(format!("Load contract {:?}", ctx.contract()))?;

        let program = risc0_zkvm::Program::load_elf(&elf, MAX_MEMORY)?;
        let image = risc0_zkvm::MemoryImage::new(&program, PAGE_SIZE)?;
        risc0_zkvm::LocalExecutor::new(env, image, program.entry)
    };

    let session = exec.run()?;

    // debug!("Start proving...");
    // _ = session.prove();
    // debug!("Proved");

    Ok(session)
}
