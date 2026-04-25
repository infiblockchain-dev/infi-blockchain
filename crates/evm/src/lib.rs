use infi_primitives::Transaction;
use infi_storage::{MemoryStorage, StorageError};

#[derive(Debug)]
pub enum ExecutionError {
    ContractExecutionNotImplemented,
    Storage(StorageError),
}

impl From<StorageError> for ExecutionError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionReceipt {
    pub success: bool,
    pub gas_used: u64,
}

pub struct EvmExecutor;

impl EvmExecutor {
    pub fn execute_transaction(
        storage: &mut MemoryStorage,
        transaction: &Transaction,
    ) -> Result<ExecutionReceipt, ExecutionError> {
        if !transaction.input.is_empty() || transaction.to.is_none() {
            return Err(ExecutionError::ContractExecutionNotImplemented);
        }

        storage.transfer(
            transaction.from,
            transaction.to.expect("checked above"),
            transaction.value,
            transaction.fee(),
            transaction.nonce,
        )?;

        Ok(ExecutionReceipt {
            success: true,
            gas_used: transaction.gas_limit,
        })
    }
}

