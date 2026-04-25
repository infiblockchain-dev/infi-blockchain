use std::collections::BTreeMap;

use infi_primitives::{Account, Address, Amount, Block};

#[derive(Debug)]
pub enum StorageError {
    AccountMissing(Address),
    InsufficientBalance { address: Address, balance: Amount, required: Amount },
    BadNonce { address: Address, expected: u64, actual: u64 },
}

pub trait ChainStorage {
    fn account(&self, address: &Address) -> Option<&Account>;
    fn upsert_account(&mut self, account: Account);
    fn push_block(&mut self, block: Block);
    fn latest_block(&self) -> Option<&Block>;
}

#[derive(Default)]
pub struct MemoryStorage {
    accounts: BTreeMap<Address, Account>,
    blocks: Vec<Block>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn credit(&mut self, address: Address, amount: Amount) {
        let account = self.accounts.entry(address).or_insert(Account {
            address,
            balance: Amount::ZERO,
            nonce: 0,
        });
        account.balance.0 += amount.0;
    }

    pub fn transfer(
        &mut self,
        from: Address,
        to: Address,
        amount: Amount,
        fee: Amount,
        nonce: u64,
    ) -> Result<(), StorageError> {
        let required = Amount(amount.0 + fee.0);
        let sender = self
            .accounts
            .get_mut(&from)
            .ok_or(StorageError::AccountMissing(from))?;

        if sender.nonce != nonce {
            return Err(StorageError::BadNonce {
                address: from,
                expected: sender.nonce,
                actual: nonce,
            });
        }

        if sender.balance < required {
            return Err(StorageError::InsufficientBalance {
                address: from,
                balance: sender.balance,
                required,
            });
        }

        sender.balance.0 -= required.0;
        sender.nonce += 1;

        let recipient = self.accounts.entry(to).or_insert(Account {
            address: to,
            balance: Amount::ZERO,
            nonce: 0,
        });
        recipient.balance.0 += amount.0;

        Ok(())
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }
}

impl ChainStorage for MemoryStorage {
    fn account(&self, address: &Address) -> Option<&Account> {
        self.accounts.get(address)
    }

    fn upsert_account(&mut self, account: Account) {
        self.accounts.insert(account.address, account);
    }

    fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

