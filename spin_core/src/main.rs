use anyhow::{Context, Result};
use borsh::BorshDeserialize;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv,
};
use tracing::{debug, info};
struct CrossContractCallHandler;

impl risc0_zkvm::Syscall for CrossContractCallHandler {
    fn syscall(
        &mut self,
        _syscall: &str,
        ctx: &mut dyn risc0_zkvm::SyscallContext,
        to_guest: &mut [u32],
    ) -> Result<(u32, u32)> {
        let buf_ptr = ctx.load_register(risc0_zkvm_platform::syscall::reg_abi::REG_A3);
        let buf_len = ctx.load_register(risc0_zkvm_platform::syscall::reg_abi::REG_A4);
        let from_guest = ctx.load_region(buf_ptr, buf_len);

        let cc = spin_sdk::ContractCall::try_from_bytes(from_guest).expect("Invalid contract call");

        debug!("Cross Contract Call: {:?}", cc);

        let session = exec_contract::<Vec<u8>>(cc).unwrap();

        let output: Vec<u32> = to_vec(&session.journal).unwrap();

        to_guest[0..output.len()].copy_from_slice(&output);
        Ok((0, 0))
    }
}

const CROSS_CONTRACT_CALL: risc0_zkvm_platform::syscall::SyscallName = unsafe {
    risc0_zkvm_platform::syscall::SyscallName::from_bytes_with_nul(
        concat!("spinvm", "::", "CROSS_CONTRACT_CALL", "\0").as_ptr(),
    )
};

fn load_contract_by_address(address: String) -> Result<Vec<u8>> {
    std::fs::read(format!("./known_contracts/{}", &address))
        .with_context(|| format!("Can't read contract {}", address))
}

fn exec_contract<T: BorshDeserialize>(call: spin_sdk::ContractCall) -> Result<risc0_zkvm::Session> {
    debug!(?call, "Executing contract");

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&call.function_call.to_bytes())?)
        .session_limit(Some(32 * 1024 * 1024))
        .syscall(CROSS_CONTRACT_CALL, CrossContractCallHandler {})
        .build()?;

    let elf = load_contract_by_address(call.address.clone())
        .context(format!("Load contract {}", call.address))?;

    let program = risc0_zkvm::Program::load_elf(&elf, 0x10000000)?;
    let image = risc0_zkvm::MemoryImage::new(&program, 0x400)?;
    let mut exec = risc0_zkvm::LocalExecutor::new(env, image, program.entry);

    let session = exec.run()?;

    Ok(session)
}

fn install_tracing() {
    use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "warn,spin_core=debug".to_owned());

    let main_layer = fmt::layer()
        .event_format(fmt::format().with_ansi(true))
        .with_filter(EnvFilter::from(filter));

    let registry = registry().with(main_layer);

    registry.init();
}

fn main() {
    install_tracing();

    let s = exec_contract::<u64>(spin_sdk::ContractCall::new(
        "demo_ccc.spin".to_string(),
        "fibonacci_and_multiply".to_string(),
        (10u32, 3u64),
    ))
    .unwrap();

    let output: u64 = from_slice(&mut s.journal.as_slice()).unwrap();
    info!(output);
}
