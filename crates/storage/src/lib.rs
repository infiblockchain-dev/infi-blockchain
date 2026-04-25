use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

use infi_primitives::{Account, Address, Amount, Block, BlockHeader, Hash, Transaction};

#[derive(Debug)]
pub enum StorageError {
    AccountMissing(Address),
    InsufficientBalance {
        address: Address,
        balance: Amount,
        required: Amount,
    },
    BadNonce {
        address: Address,
        expected: u64,
        actual: u64,
    },
}

pub trait ChainStorage {
    fn account(&self, address: &Address) -> Option<&Account>;
    fn upsert_account(&mut self, account: Account);
    fn push_block(&mut self, block: Block);
    fn latest_block(&self) -> Option<&Block>;
}

#[derive(Clone, Debug)]
pub struct TransactionReceipt {
    pub transaction_hash: Hash,
    pub transaction_index: u64,
    pub block_number: u64,
    pub block_hash: Hash,
    pub from: Address,
    pub to: Option<Address>,
    pub gas_used: u64,
    pub cumulative_gas_used: u64,
    pub status: bool,
}

#[derive(Default)]
pub struct MemoryStorage {
    accounts: BTreeMap<Address, Account>,
    blocks: Vec<Block>,
    receipts: BTreeMap<Hash, TransactionReceipt>,
    transactions: BTreeMap<Hash, Transaction>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_dir(data_dir: impl AsRef<Path>) -> io::Result<Self> {
        let data_dir = data_dir.as_ref();
        fs::create_dir_all(data_dir)?;

        let mut storage = Self::new();
        storage.load_accounts(data_dir)?;
        storage.load_blocks(data_dir)?;
        storage.rebuild_indexes();
        Ok(storage)
    }

    pub fn save_to_dir(&self, data_dir: impl AsRef<Path>) -> io::Result<()> {
        let data_dir = data_dir.as_ref();
        fs::create_dir_all(data_dir)?;
        write_atomic(&data_dir.join("accounts.tsv"), &self.accounts_tsv())?;
        write_atomic(&data_dir.join("blocks.tsv"), &self.blocks_tsv())?;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty() && self.blocks.is_empty()
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn receipt_count(&self) -> usize {
        self.receipts.len()
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

    pub fn receipt(&self, transaction_hash: &Hash) -> Option<&TransactionReceipt> {
        self.receipts.get(transaction_hash)
    }

    pub fn transaction(&self, transaction_hash: &Hash) -> Option<&Transaction> {
        self.transactions.get(transaction_hash)
    }

    pub fn block_by_number(&self, number: u64) -> Option<&Block> {
        self.blocks
            .iter()
            .find(|block| block.header.number == number)
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }

    fn load_accounts(&mut self, data_dir: &Path) -> io::Result<()> {
        let path = data_dir.join("accounts.tsv");
        let Ok(contents) = fs::read_to_string(&path) else {
            return Ok(());
        };

        for (line_index, line) in contents.lines().enumerate() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() != 3 {
                return invalid_data(&path, line_index, "account row must have 3 fields");
            }
            let address = parse_field::<Address>(&path, line_index, fields[0], "address")?;
            let balance = parse_field::<u128>(&path, line_index, fields[1], "balance")?;
            let nonce = parse_field::<u64>(&path, line_index, fields[2], "nonce")?;
            self.accounts.insert(
                address,
                Account {
                    address,
                    balance: Amount(balance),
                    nonce,
                },
            );
        }

        Ok(())
    }

    fn load_blocks(&mut self, data_dir: &Path) -> io::Result<()> {
        let path = data_dir.join("blocks.tsv");
        let Ok(contents) = fs::read_to_string(&path) else {
            return Ok(());
        };

        let mut blocks: BTreeMap<u64, Block> = BTreeMap::new();
        for (line_index, line) in contents.lines().enumerate() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }

            let fields: Vec<&str> = line.split('\t').collect();
            match fields.first().copied() {
                Some("block") => {
                    if fields.len() != 7 {
                        return invalid_data(&path, line_index, "block row must have 7 fields");
                    }
                    let number = parse_field::<u64>(&path, line_index, fields[1], "number")?;
                    let header = BlockHeader {
                        number,
                        parent_hash: parse_field::<Hash>(
                            &path,
                            line_index,
                            fields[2],
                            "parent_hash",
                        )?,
                        state_root: parse_field::<Hash>(
                            &path,
                            line_index,
                            fields[3],
                            "state_root",
                        )?,
                        tx_root: parse_field::<Hash>(&path, line_index, fields[4], "tx_root")?,
                        proposer: parse_field::<Address>(&path, line_index, fields[5], "proposer")?,
                        timestamp_ms: parse_field::<u64>(
                            &path,
                            line_index,
                            fields[6],
                            "timestamp_ms",
                        )?,
                    };
                    blocks.insert(
                        number,
                        Block {
                            header,
                            transactions: Vec::new(),
                        },
                    );
                }
                Some("tx") => {
                    if fields.len() != 9 {
                        return invalid_data(&path, line_index, "tx row must have 9 fields");
                    }
                    let block_number =
                        parse_field::<u64>(&path, line_index, fields[1], "block_number")?;
                    let Some(block) = blocks.get_mut(&block_number) else {
                        return invalid_data(&path, line_index, "tx row references missing block");
                    };
                    let to = if fields[3] == "null" {
                        None
                    } else {
                        Some(parse_field::<Address>(&path, line_index, fields[3], "to")?)
                    };
                    let input = decode_hex(fields[8]).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{}:{} invalid input hex", path.display(), line_index + 1),
                        )
                    })?;
                    block.transactions.push(Transaction {
                        from: parse_field::<Address>(&path, line_index, fields[2], "from")?,
                        to,
                        value: Amount(parse_field::<u128>(&path, line_index, fields[4], "value")?),
                        gas_limit: parse_field::<u64>(&path, line_index, fields[5], "gas_limit")?,
                        gas_price: Amount(parse_field::<u128>(
                            &path,
                            line_index,
                            fields[6],
                            "gas_price",
                        )?),
                        nonce: parse_field::<u64>(&path, line_index, fields[7], "nonce")?,
                        input,
                    });
                }
                _ => return invalid_data(&path, line_index, "unknown row type"),
            }
        }

        self.blocks = blocks.into_values().collect();
        Ok(())
    }

    fn accounts_tsv(&self) -> String {
        let mut output = String::from("# address\tbalance\tnonce\n");
        for account in self.accounts.values() {
            output.push_str(&format!(
                "{}\t{}\t{}\n",
                account.address, account.balance.0, account.nonce
            ));
        }
        output
    }

    fn blocks_tsv(&self) -> String {
        let mut output = String::from(
            "# block\tnumber\tparent_hash\tstate_root\ttx_root\tproposer\ttimestamp_ms\n",
        );
        output.push_str(
            "# tx\tblock_number\tfrom\tto\tvalue\tgas_limit\tgas_price\tnonce\tinput_hex\n",
        );
        for block in &self.blocks {
            output.push_str(&format!(
                "block\t{}\t{}\t{}\t{}\t{}\t{}\n",
                block.header.number,
                block.header.parent_hash,
                block.header.state_root,
                block.header.tx_root,
                block.header.proposer,
                block.header.timestamp_ms
            ));
            for transaction in &block.transactions {
                let to = transaction
                    .to
                    .map(|address| address.to_string())
                    .unwrap_or_else(|| "null".to_string());
                output.push_str(&format!(
                    "tx\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    block.header.number,
                    transaction.from,
                    to,
                    transaction.value.0,
                    transaction.gas_limit,
                    transaction.gas_price.0,
                    transaction.nonce,
                    encode_hex(&transaction.input)
                ));
            }
        }
        output
    }

    fn rebuild_indexes(&mut self) {
        self.receipts.clear();
        self.transactions.clear();

        for block in &self.blocks {
            let block_number = block.header.number;
            let block_hash = Hash::from_bytes(format!("{:?}", block.header).as_bytes());
            let mut cumulative_gas_used = 0_u64;

            for (index, transaction) in block.transactions.iter().enumerate() {
                let transaction_hash = transaction.hash();
                cumulative_gas_used += transaction.gas_limit;
                self.transactions
                    .insert(transaction_hash, transaction.clone());
                self.receipts.insert(
                    transaction_hash,
                    TransactionReceipt {
                        transaction_hash,
                        transaction_index: index as u64,
                        block_number,
                        block_hash,
                        from: transaction.from,
                        to: transaction.to,
                        gas_used: transaction.gas_limit,
                        cumulative_gas_used,
                        status: true,
                    },
                );
            }
        }
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
        self.rebuild_indexes();
    }

    fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

fn parse_field<T>(path: &Path, line_index: usize, value: &str, field: &str) -> io::Result<T>
where
    T: FromStr,
{
    value.parse::<T>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}:{} invalid {field}", path.display(), line_index + 1),
        )
    })
}

fn invalid_data<T>(path: &Path, line_index: usize, message: &str) -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{}:{} {message}", path.display(), line_index + 1),
    ))
}

fn write_atomic(path: &Path, contents: &str) -> io::Result<()> {
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, contents)?;
    fs::rename(temp_path, path)?;
    Ok(())
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if value.len() % 2 != 0 {
        return None;
    }

    let mut output = Vec::with_capacity(value.len() / 2);
    for index in (0..value.len()).step_by(2) {
        output.push(u8::from_str_radix(&value[index..index + 2], 16).ok()?);
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn persists_accounts_blocks_transactions_and_receipts() {
        let dir = std::env::temp_dir().join(format!(
            "infi-storage-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        let alice = Address::repeat(0x11);
        let bob = Address::repeat(0x22);
        let transaction = Transaction::simple_transfer(alice, bob, Amount(5), 0);
        let transaction_hash = transaction.hash();

        let mut storage = MemoryStorage::new();
        storage.credit(alice, Amount(100_000));
        storage
            .transfer(
                alice,
                bob,
                transaction.value,
                transaction.fee(),
                transaction.nonce,
            )
            .unwrap();
        storage.push_block(Block {
            header: BlockHeader {
                parent_hash: Hash::ZERO,
                number: 1,
                state_root: Hash::ZERO,
                tx_root: transaction_hash,
                proposer: Address::repeat(0xaa),
                timestamp_ms: 1,
            },
            transactions: vec![transaction],
        });
        storage.save_to_dir(&dir).unwrap();

        let loaded = MemoryStorage::load_from_dir(&dir).unwrap();
        assert_eq!(loaded.block_count(), 1);
        assert_eq!(loaded.receipt_count(), 1);
        assert_eq!(loaded.account(&alice).unwrap().balance, Amount(78_995));
        assert_eq!(loaded.account(&bob).unwrap().balance, Amount(5));
        assert!(loaded.receipt(&transaction_hash).is_some());
        assert!(loaded.transaction(&transaction_hash).is_some());

        fs::remove_dir_all(dir).ok();
    }
}
