use std::collections::VecDeque;

use infi_primitives::Transaction;

#[derive(Default)]
pub struct Mempool {
    pending: VecDeque<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn submit(&mut self, transaction: Transaction) {
        self.pending.push_back(transaction);
    }

    pub fn drain_for_block(&mut self, max_transactions: usize) -> Vec<Transaction> {
        let count = max_transactions.min(self.pending.len());
        self.pending.drain(..count).collect()
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

