use anyhow::Result;
use risc0_zkvm::Session;

use std::sync::{Arc, RwLock};

use spin_primitives::{AccountId, CallEnv, ContractCall, FunctionCall};

pub struct ExecutionContext {
    signer: AccountId,
    caller: AccountId,
    contract: AccountId,
    attached_gas: u64,
    used_gas: u64,
    call: FunctionCall,

    cross_contract_calls: Vec<Arc<RwLock<ExecutionContext>>>,
    session: Option<Session>,
}

impl ExecutionContext {
    pub fn new(
        signer: AccountId,
        caller: AccountId,
        contract: AccountId,
        attached_gas: u64,
        call: FunctionCall,
    ) -> Self {
        Self {
            signer,
            caller,
            contract,
            attached_gas,
            used_gas: 0,
            call,
            cross_contract_calls: Vec::new(),
            session: None,
        }
    }

    pub fn cross_contract_call(
        &mut self,
        call: ContractCall,
        attached_gas: u64,
    ) -> Result<Arc<RwLock<ExecutionContext>>> {
        if self.available_gas() < attached_gas {
            return Err(anyhow::anyhow!("Not enough gas"));
        }
        let context = Arc::new(RwLock::new(ExecutionContext {
            signer: self.signer().clone(),
            caller: self.contract().clone(),
            contract: call.account.clone(),
            attached_gas,
            used_gas: 0,
            call: call.function_call,
            cross_contract_calls: Vec::new(),
            session: None,
        }));

        self.cross_contract_calls.push(context.clone());

        Ok(context)
    }

    pub fn signer(&self) -> &AccountId {
        &self.signer
    }

    pub fn caller(&self) -> &AccountId {
        &self.caller
    }

    pub fn contract(&self) -> &AccountId {
        &self.contract
    }

    pub fn attached_gas(&self) -> u64 {
        self.attached_gas
    }

    pub fn used_gas(&self) -> u64 {
        self.used_gas
    }

    pub fn call(&self) -> &FunctionCall {
        &self.call
    }

    pub fn available_gas(&self) -> u64 {
        self.attached_gas - self.used_gas
    }

    pub fn call_env(&self) -> CallEnv {
        CallEnv {
            signer: self.signer().clone(),
            caller: self.caller().clone(),
            contract: self.contract().clone(),
            attached_gas: self.attached_gas(),
        }
    }

    pub fn execution_session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    pub fn set_execution_session(&mut self, session: Session) {
        self.session = Some(session);
    }
}
