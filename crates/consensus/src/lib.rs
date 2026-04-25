use infi_primitives::{Address, Block, BlockHeader, Hash, Transaction};

#[derive(Clone, Debug)]
pub struct Validator {
    pub address: Address,
    pub power: u64,
}

#[derive(Clone, Debug)]
pub struct ConsensusConfig {
    pub validators: Vec<Validator>,
    pub max_transactions_per_block: usize,
}

impl ConsensusConfig {
    pub fn single_validator_devnet() -> Self {
        Self {
            validators: vec![Validator {
                address: Address::repeat(0xaa),
                power: 1,
            }],
            max_transactions_per_block: 1_000,
        }
    }
}

pub struct DevnetConsensus {
    config: ConsensusConfig,
}

impl DevnetConsensus {
    pub fn new(config: ConsensusConfig) -> Self {
        Self { config }
    }

    pub fn propose_block(
        &self,
        parent_hash: Hash,
        number: u64,
        timestamp_ms: u64,
        transactions: Vec<Transaction>,
    ) -> Block {
        let proposer = self.config.validators[0].address;
        Block {
            header: BlockHeader {
                parent_hash,
                number,
                state_root: Hash::ZERO,
                tx_root: Hash::ZERO,
                proposer,
                timestamp_ms,
            },
            transactions,
        }
    }

    pub fn max_transactions_per_block(&self) -> usize {
        self.config.max_transactions_per_block
    }
}

