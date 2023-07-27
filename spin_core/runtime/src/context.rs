use anyhow::Result;
use risc0_zkvm::Session;

use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

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
    ) -> Result<Arc<RwLock<ExecutionContext>>> {
        if self.available_gas() < call.attached_gas {
            return Err(anyhow::anyhow!("Not enough gas"));
        }
        let context = Arc::new(RwLock::new(ExecutionContext {
            signer: self.signer().clone(),
            caller: self.contract().clone(),
            contract: call.account.clone(),
            attached_gas: call.attached_gas,
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
        let cc_gas = self
            .cross_contract_calls
            .iter()
            .map(|call| call.read().unwrap().used_gas())
            .sum::<u64>();

        self.attached_gas
            .saturating_sub(self.used_gas)
            .saturating_sub(cc_gas)
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

    pub fn set_gas_usage(&mut self, used_gas: u64) {
        self.used_gas = used_gas;
    }

    // TODO: remove hardcode, use custom alias system
    pub fn get_account_evm_address(account_id: AccountId) -> eth_primitive_types::H160 {
        let mut hardcoded_mappings = std::collections::HashMap::new();
        hardcoded_mappings.insert(
            AccountId::from(AccountId::new("alice.spin".to_string())),
            eth_primitive_types::H160::from_str("0x0FF1CE0000000000000000000000000000000001")
                .unwrap(),
        );
        hardcoded_mappings.insert(
            AccountId::from(AccountId::new("bob.spin".to_string())),
            eth_primitive_types::H160::from_str("0x0FF1CE0000000000000000000000000000000002")
                .unwrap(),
        );
        hardcoded_mappings.insert(
            AccountId::from(AccountId::new("charlie.spin".to_string())),
            eth_primitive_types::H160::from_str("0x0FF1CE0000000000000000000000000000000003")
                .unwrap(),
        );
        hardcoded_mappings.insert(
            AccountId::from(AccountId::new("eve.spin".to_string())),
            eth_primitive_types::H160::from_str("0x0FF1CE0000000000000000000000000000000004")
                .unwrap(),
        );

        hardcoded_mappings.get(&account_id).unwrap().clone()
    }
}
