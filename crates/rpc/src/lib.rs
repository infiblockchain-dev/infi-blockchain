use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use infi_primitives::{Address, Amount, Block, BlockHeader, ChainConfig, Hash, Transaction};
use infi_storage::{ChainStorage, MemoryStorage, TransactionReceipt};

const INVERTX_UNIT: u128 = 1_000_000_000_000_000_000;
const FAUCET_MONTHLY_LIMIT: u128 = 100_000 * INVERTX_UNIT;
const FAUCET_MAX_CLAIM: u128 = 10_000 * INVERTX_UNIT;
const FAUCET_WARNING: &str =
    "test InvertX is non-tradable testnet gas with no redeemable real-world value.";

pub struct RpcInfo {
    pub chain_id_hex: String,
    pub client_version: String,
}

pub fn devnet_rpc_info(config: &ChainConfig) -> RpcInfo {
    RpcInfo {
        chain_id_hex: format!("0x{:x}", config.chain_id),
        client_version: format!("infi-devnet/{}", env!("CARGO_PKG_VERSION")),
    }
}

pub struct RpcServer {
    config: ChainConfig,
    storage: Arc<Mutex<MemoryStorage>>,
    faucet: Arc<Mutex<FaucetState>>,
    data_dir: Option<PathBuf>,
    info: RpcInfo,
}

impl RpcServer {
    pub fn new(config: ChainConfig, storage: Arc<Mutex<MemoryStorage>>) -> Self {
        Self::with_data_dir(config, storage, None)
    }

    pub fn with_data_dir(
        config: ChainConfig,
        storage: Arc<Mutex<MemoryStorage>>,
        data_dir: Option<PathBuf>,
    ) -> Self {
        let info = devnet_rpc_info(&config);
        let faucet = data_dir
            .as_ref()
            .and_then(|data_dir| match FaucetState::load_from_dir(data_dir) {
                Ok(faucet) => Some(faucet),
                Err(error) => {
                    println!("Failed to load faucet state: {error}");
                    None
                }
            })
            .unwrap_or_else(FaucetState::new);
        Self {
            config,
            storage,
            faucet: Arc::new(Mutex::new(faucet)),
            data_dir,
            info,
        }
    }

    pub fn serve(self, bind_address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(bind_address)?;
        println!("INFI JSON-RPC listening on http://{bind_address}");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(error) = self.handle_connection(stream) {
                        println!("RPC connection error: {error}");
                    }
                }
                Err(error) => {
                    println!("RPC accept error: {error}");
                }
            }
        }

        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let request = read_http_request(&mut stream)?;
        if request.is_empty() {
            return Ok(());
        }

        let target = request_target(&request);
        let response_body = if request.starts_with("OPTIONS ") {
            String::new()
        } else if matches!(target, Some(("GET", "/health")) | Some(("GET", "/"))) {
            self.health_json()
        } else if let Some(("GET", path)) =
            target.filter(|(_, path)| path.starts_with("/faucet/status"))
        {
            self.handle_faucet_status(path)
        } else if let Some(("GET", path)) =
            target.filter(|(_, path)| path.starts_with("/faucet/history"))
        {
            self.handle_faucet_history(path)
        } else if matches!(target, Some(("POST", "/faucet/claim"))) {
            match http_body(&request) {
                Some(body) => self.handle_faucet_claim(body),
                None => json_error("null", -32700, "Parse error"),
            }
        } else if let Some(body) = http_body(&request) {
            self.handle_rpc_body(body)
        } else {
            json_error("null", -32700, "Parse error")
        };

        let status = if response_body.is_empty() {
            "204 No Content"
        } else {
            "200 OK"
        };

        let response = format!(
            "HTTP/1.1 {status}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             Access-Control-Allow-Headers: content-type\r\n\
             Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            response_body.len(),
            response_body
        );

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn health_json(&self) -> String {
        format!(
            "{{\"status\":\"ok\",\"chain\":\"{}\",\"chainId\":\"{}\",\"clientVersion\":\"{}\"}}",
            escape_json(&self.config.chain_name),
            self.info.chain_id_hex,
            escape_json(&self.info.client_version)
        )
    }

    fn handle_rpc_body(&self, body: &str) -> String {
        let id = json_id(body).unwrap_or_else(|| "null".to_string());
        let Some(method) = json_string_field(body, "method") else {
            return json_error(&id, -32600, "Invalid request");
        };

        match method.as_str() {
            "web3_clientVersion" => json_result_string(&id, &self.info.client_version),
            "eth_chainId" => json_result_string(&id, &self.info.chain_id_hex),
            "net_version" => json_result_string(&id, &self.config.chain_id.to_string()),
            "eth_syncing" => json_result_raw(&id, "false"),
            "eth_accounts" => json_result_raw(&id, "[]"),
            "eth_gasPrice" => json_result_string(&id, "0x1"),
            "eth_maxPriorityFeePerGas" => json_result_string(&id, "0x1"),
            "eth_estimateGas" => json_result_string(&id, "0x5208"),
            "eth_call" => json_result_string(&id, "0x"),
            "eth_getCode" => json_result_string(&id, "0x"),
            "eth_getStorageAt" => json_result_string(&id, &format!("0x{}", "0".repeat(64))),
            "eth_getLogs" => json_result_raw(&id, "[]"),
            "eth_feeHistory" => self.handle_fee_history(&id, body),
            "eth_blockNumber" => {
                let storage = match self.storage.lock() {
                    Ok(storage) => storage,
                    Err(_) => return json_error(&id, -32000, "Storage unavailable"),
                };
                let block_number = storage
                    .latest_block()
                    .map(|block| block.header.number)
                    .unwrap_or(0);
                json_result_string(&id, &format!("0x{block_number:x}"))
            }
            "eth_getBalance" => self.handle_get_balance(&id, body),
            "eth_getBlockByNumber" => self.handle_get_block_by_number(&id, body),
            "eth_getBlockByHash" => self.handle_get_block_by_hash(&id, body),
            "eth_getBlockTransactionCountByNumber" => {
                self.handle_get_block_transaction_count_by_number(&id, body)
            }
            "eth_getBlockTransactionCountByHash" => {
                self.handle_get_block_transaction_count_by_hash(&id, body)
            }
            "eth_getTransactionCount" => self.handle_get_transaction_count(&id, body),
            "eth_getTransactionByHash" => self.handle_get_transaction_by_hash(&id, body),
            "eth_getTransactionByBlockNumberAndIndex" => {
                self.handle_get_transaction_by_block_number_and_index(&id, body)
            }
            "eth_getTransactionByBlockHashAndIndex" => {
                self.handle_get_transaction_by_block_hash_and_index(&id, body)
            }
            "eth_getTransactionReceipt" => self.handle_get_transaction_receipt(&id, body),
            "eth_sendRawTransaction" => self.handle_send_raw_transaction(&id, body),
            _ => json_error(&id, -32601, "Method not found"),
        }
    }

    fn handle_fee_history(&self, id: &str, body: &str) -> String {
        let block_count = params_string_at(body, 0)
            .as_deref()
            .and_then(parse_quantity)
            .unwrap_or(1)
            .clamp(1, 16);
        let oldest_block = {
            let storage = match self.storage.lock() {
                Ok(storage) => storage,
                Err(_) => return json_error(id, -32000, "Storage unavailable"),
            };
            storage
                .latest_block()
                .map(|block| block.header.number.saturating_sub(block_count - 1))
                .unwrap_or(0)
        };

        let base_fees = std::iter::repeat("\"0x1\"")
            .take(block_count as usize + 1)
            .collect::<Vec<_>>()
            .join(",");
        let gas_used_ratio = std::iter::repeat("0")
            .take(block_count as usize)
            .collect::<Vec<_>>()
            .join(",");
        let rewards = std::iter::repeat("[\"0x1\"]")
            .take(block_count as usize)
            .collect::<Vec<_>>()
            .join(",");

        json_result_raw(
            id,
            &format!(
                "{{\"oldestBlock\":\"0x{oldest_block:x}\",\"baseFeePerGas\":[{base_fees}],\"gasUsedRatio\":[{gas_used_ratio}],\"reward\":[{rewards}]}}"
            ),
        )
    }

    fn handle_get_balance(&self, id: &str, body: &str) -> String {
        let Some(address_text) = first_params_string(body) else {
            return json_error(id, -32602, "eth_getBalance requires an address parameter");
        };

        let address = match Address::from_str(&address_text) {
            Ok(address) => address,
            Err(_) => return json_error(id, -32602, "Invalid address"),
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let balance = storage
            .account(&address)
            .map(|account| account.balance.0)
            .unwrap_or(0);

        json_result_string(id, &format!("0x{balance:x}"))
    }

    fn handle_get_transaction_count(&self, id: &str, body: &str) -> String {
        let Some(address_text) = first_params_string(body) else {
            return json_error(
                id,
                -32602,
                "eth_getTransactionCount requires an address parameter",
            );
        };

        let address = match Address::from_str(&address_text) {
            Ok(address) => address,
            Err(_) => return json_error(id, -32602, "Invalid address"),
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let nonce = storage
            .account(&address)
            .map(|account| account.nonce)
            .unwrap_or(0);

        json_result_string(id, &format!("0x{nonce:x}"))
    }

    fn handle_get_block_by_number(&self, id: &str, body: &str) -> String {
        let Some(block_tag) = params_string_at(body, 0) else {
            return json_error(
                id,
                -32602,
                "eth_getBlockByNumber requires a block parameter",
            );
        };
        let full_transactions = params_bool_at(body, 1).unwrap_or(false);

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(block) = block_by_tag(&storage, &block_tag) else {
            return json_result_raw(id, "null");
        };

        json_result_raw(
            id,
            &block_json(block, &storage, &self.info.chain_id_hex, full_transactions),
        )
    }

    fn handle_get_block_by_hash(&self, id: &str, body: &str) -> String {
        let Some(hash_text) = params_string_at(body, 0) else {
            return json_error(
                id,
                -32602,
                "eth_getBlockByHash requires a block hash parameter",
            );
        };
        let Some(hash) = parse_hash(&hash_text) else {
            return json_error(id, -32602, "Invalid block hash");
        };
        let full_transactions = params_bool_at(body, 1).unwrap_or(false);

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(block) = storage.blocks().find(|block| block_hash(block) == hash) else {
            return json_result_raw(id, "null");
        };

        json_result_raw(
            id,
            &block_json(block, &storage, &self.info.chain_id_hex, full_transactions),
        )
    }

    fn handle_get_block_transaction_count_by_number(&self, id: &str, body: &str) -> String {
        let Some(block_tag) = params_string_at(body, 0) else {
            return json_error(
                id,
                -32602,
                "eth_getBlockTransactionCountByNumber requires a block parameter",
            );
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(block) = block_by_tag(&storage, &block_tag) else {
            return json_result_raw(id, "null");
        };

        json_result_string(id, &format!("0x{:x}", block.transactions.len()))
    }

    fn handle_get_block_transaction_count_by_hash(&self, id: &str, body: &str) -> String {
        let Some(hash_text) = params_string_at(body, 0) else {
            return json_error(
                id,
                -32602,
                "eth_getBlockTransactionCountByHash requires a block hash parameter",
            );
        };
        let Some(hash) = parse_hash(&hash_text) else {
            return json_error(id, -32602, "Invalid block hash");
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(block) = storage.blocks().find(|block| block_hash(block) == hash) else {
            return json_result_raw(id, "null");
        };

        json_result_string(id, &format!("0x{:x}", block.transactions.len()))
    }

    fn handle_get_transaction_by_hash(&self, id: &str, body: &str) -> String {
        let Some(hash_text) = first_params_string(body) else {
            return json_error(
                id,
                -32602,
                "eth_getTransactionByHash requires a transaction hash parameter",
            );
        };

        let transaction_hash = match parse_hash(&hash_text) {
            Some(hash) => hash,
            None => return json_error(id, -32602, "Invalid transaction hash"),
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(transaction) = storage.transaction(&transaction_hash) else {
            return json_result_raw(id, "null");
        };
        let Some(receipt) = storage.receipt(&transaction_hash) else {
            return json_result_raw(id, "null");
        };

        json_result_raw(
            id,
            &transaction_json(transaction, receipt, &self.info.chain_id_hex),
        )
    }

    fn handle_get_transaction_by_block_number_and_index(&self, id: &str, body: &str) -> String {
        let Some(block_tag) = params_string_at(body, 0) else {
            return json_error(
                id,
                -32602,
                "eth_getTransactionByBlockNumberAndIndex requires a block parameter",
            );
        };
        let Some(index_text) = params_string_at(body, 1) else {
            return json_error(id, -32602, "Missing transaction index");
        };
        let Some(index) = parse_quantity(&index_text) else {
            return json_error(id, -32602, "Invalid transaction index");
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(block) = block_by_tag(&storage, &block_tag) else {
            return json_result_raw(id, "null");
        };

        transaction_by_block_index_json(id, &storage, block, index, &self.info.chain_id_hex)
    }

    fn handle_get_transaction_by_block_hash_and_index(&self, id: &str, body: &str) -> String {
        let Some(hash_text) = params_string_at(body, 0) else {
            return json_error(
                id,
                -32602,
                "eth_getTransactionByBlockHashAndIndex requires a block hash parameter",
            );
        };
        let Some(hash) = parse_hash(&hash_text) else {
            return json_error(id, -32602, "Invalid block hash");
        };
        let Some(index_text) = params_string_at(body, 1) else {
            return json_error(id, -32602, "Missing transaction index");
        };
        let Some(index) = parse_quantity(&index_text) else {
            return json_error(id, -32602, "Invalid transaction index");
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(block) = storage.blocks().find(|block| block_hash(block) == hash) else {
            return json_result_raw(id, "null");
        };

        transaction_by_block_index_json(id, &storage, block, index, &self.info.chain_id_hex)
    }

    fn handle_get_transaction_receipt(&self, id: &str, body: &str) -> String {
        let Some(hash_text) = first_params_string(body) else {
            return json_error(
                id,
                -32602,
                "eth_getTransactionReceipt requires a transaction hash parameter",
            );
        };

        let transaction_hash = match parse_hash(&hash_text) {
            Some(hash) => hash,
            None => return json_error(id, -32602, "Invalid transaction hash"),
        };

        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        let Some(receipt) = storage.receipt(&transaction_hash) else {
            return json_result_raw(id, "null");
        };

        json_result_raw(id, &receipt_json(receipt))
    }

    fn handle_send_raw_transaction(&self, id: &str, body: &str) -> String {
        let Some(raw_transaction) = first_params_string(body) else {
            return json_error(
                id,
                -32602,
                "eth_sendRawTransaction requires a raw transaction parameter",
            );
        };

        let transaction = match decode_dev_transfer_transaction(&raw_transaction) {
            Ok(transaction) => transaction,
            Err(message) => return json_error(id, -32602, message),
        };

        let Some(to) = transaction.to else {
            return json_error(id, -32602, "Dev transfer requires a recipient");
        };
        let mut storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return json_error(id, -32000, "Storage unavailable"),
        };
        if let Err(error) = storage.transfer(
            transaction.from,
            to,
            transaction.value,
            transaction.fee(),
            transaction.nonce,
        ) {
            return json_error(id, -32000, &format!("Transaction rejected: {error:?}"));
        }

        let transaction_hash = mine_transaction(&mut storage, transaction);
        if let Err(error) = self.persist_chain_storage(&storage) {
            return json_error(id, -32000, &error);
        }

        json_result_string(id, &transaction_hash.to_string())
    }

    fn handle_faucet_status(&self, path: &str) -> String {
        let Some(address_text) = query_value(path, "address") else {
            return faucet_error("faucet_status requires an address query parameter");
        };
        let address = match Address::from_str(&address_text) {
            Ok(address) => address,
            Err(_) => return faucet_error("Invalid address"),
        };

        let month_key = current_month_key();
        let faucet = match self.faucet.lock() {
            Ok(faucet) => faucet,
            Err(_) => return faucet_error("Faucet unavailable"),
        };
        let claimed = faucet.claimed_this_month(address, &month_key);
        faucet_status_json(address, &month_key, claimed)
    }

    fn handle_faucet_history(&self, path: &str) -> String {
        let limit = query_value(path, "limit")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(100)
            .clamp(1, 500);
        let address_filter = match query_value(path, "address") {
            Some(value) if !value.trim().is_empty() => match Address::from_str(&value) {
                Ok(address) => Some(address),
                Err(_) => return faucet_error("Invalid history filter address"),
            },
            _ => None,
        };
        let storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return faucet_error("Storage unavailable"),
        };
        faucet_history_json(&storage, limit, address_filter)
    }

    fn handle_faucet_claim(&self, body: &str) -> String {
        let Some(address_text) = json_string_field(body, "address") else {
            return faucet_error("faucet_claim requires an address");
        };
        let Some(amount_text) = json_string_field(body, "amount") else {
            return faucet_error("faucet_claim requires an amount");
        };

        let recipient = match Address::from_str(&address_text) {
            Ok(address) => address,
            Err(_) => return faucet_error("Invalid address"),
        };
        let amount = match amount_text.parse::<u128>() {
            Ok(amount) if amount > 0 => Amount::from_invertx_units(amount),
            _ => return faucet_error("Invalid amount"),
        };

        if amount.0 > FAUCET_MAX_CLAIM {
            return faucet_error("Claim amount exceeds the 10,000 test InvertX per-claim limit");
        }

        let month_key = current_month_key();
        let mut faucet = match self.faucet.lock() {
            Ok(faucet) => faucet,
            Err(_) => return faucet_error("Faucet unavailable"),
        };
        let claimed = faucet.claimed_this_month(recipient, &month_key);
        let remaining = FAUCET_MONTHLY_LIMIT.saturating_sub(claimed);
        if amount.0 > remaining {
            return faucet_error("Claim exceeds the remaining monthly faucet allowance");
        }

        let faucet_address = Address::repeat(0x22);
        let mut storage = match self.storage.lock() {
            Ok(storage) => storage,
            Err(_) => return faucet_error("Storage unavailable"),
        };
        let nonce = storage
            .account(&faucet_address)
            .map(|account| account.nonce)
            .unwrap_or(0);
        let transaction = Transaction::simple_transfer(faucet_address, recipient, amount, nonce);

        if let Err(error) = storage.transfer(
            faucet_address,
            recipient,
            amount,
            Amount::ZERO,
            transaction.nonce,
        ) {
            return faucet_error(&format!("Faucet transfer rejected: {error:?}"));
        }

        let transaction_hash = mine_transaction(&mut storage, transaction);
        let (block_number, block_hash, timestamp_ms) = storage
            .receipt(&transaction_hash)
            .and_then(|receipt| {
                storage.block_by_number(receipt.block_number).map(|block| {
                    (
                        receipt.block_number,
                        receipt.block_hash,
                        block.header.timestamp_ms,
                    )
                })
            })
            .unwrap_or((0, Hash::ZERO, now_ms()));
        let claimed_after = faucet.record_claim(recipient, &month_key, amount.0);
        if let Err(error) = self.persist_chain_storage(&storage) {
            return faucet_error(&error);
        }
        if let Err(error) = self.persist_faucet_state(&faucet) {
            return faucet_error(&error);
        }
        faucet_claim_json(
            recipient,
            &transaction_hash,
            amount.0,
            &month_key,
            claimed_after,
            block_number,
            block_hash,
            timestamp_ms,
        )
    }

    fn persist_chain_storage(&self, storage: &MemoryStorage) -> Result<(), String> {
        let Some(data_dir) = &self.data_dir else {
            return Ok(());
        };
        storage
            .save_to_dir(data_dir)
            .map_err(|error| format!("Failed to persist chain state: {error}"))
    }

    fn persist_faucet_state(&self, faucet: &FaucetState) -> Result<(), String> {
        let Some(data_dir) = &self.data_dir else {
            return Ok(());
        };
        faucet
            .save_to_dir(data_dir)
            .map_err(|error| format!("Failed to persist faucet state: {error}"))
    }
}

#[derive(Default)]
struct FaucetState {
    claims: BTreeMap<Address, FaucetClaim>,
}

#[derive(Clone, Debug)]
struct FaucetClaim {
    month_key: String,
    claimed: u128,
}

impl FaucetState {
    fn new() -> Self {
        Self::default()
    }

    fn load_from_dir(data_dir: &Path) -> std::io::Result<Self> {
        fs::create_dir_all(data_dir)?;
        let path = data_dir.join("faucet_claims.tsv");
        let Ok(contents) = fs::read_to_string(&path) else {
            return Ok(Self::new());
        };

        let mut faucet = Self::new();
        for (line_index, line) in contents.lines().enumerate() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() != 3 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "{}:{} faucet row must have 3 fields",
                        path.display(),
                        line_index + 1
                    ),
                ));
            }
            let address = Address::from_str(fields[0]).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{}:{} invalid address", path.display(), line_index + 1),
                )
            })?;
            let claimed = fields[2].parse::<u128>().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "{}:{} invalid claimed amount",
                        path.display(),
                        line_index + 1
                    ),
                )
            })?;
            faucet.claims.insert(
                address,
                FaucetClaim {
                    month_key: fields[1].to_string(),
                    claimed,
                },
            );
        }
        Ok(faucet)
    }

    fn save_to_dir(&self, data_dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(data_dir)?;
        let path = data_dir.join("faucet_claims.tsv");
        let mut output = String::from("# address\tmonth\tclaimed\n");
        for (address, claim) in &self.claims {
            output.push_str(&format!(
                "{}\t{}\t{}\n",
                address, claim.month_key, claim.claimed
            ));
        }
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, output)?;
        fs::rename(temp_path, path)?;
        Ok(())
    }

    fn claimed_this_month(&self, address: Address, month_key: &str) -> u128 {
        self.claims
            .get(&address)
            .filter(|claim| claim.month_key == month_key)
            .map(|claim| claim.claimed)
            .unwrap_or(0)
    }

    fn record_claim(&mut self, address: Address, month_key: &str, amount: u128) -> u128 {
        let claim = self.claims.entry(address).or_insert(FaucetClaim {
            month_key: month_key.to_string(),
            claimed: 0,
        });
        if claim.month_key != month_key {
            claim.month_key = month_key.to_string();
            claim.claimed = 0;
        }
        claim.claimed += amount;
        claim.claimed
    }
}

fn http_body(request: &str) -> Option<&str> {
    request.split("\r\n\r\n").nth(1)
}

fn read_http_request(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut request = Vec::new();
    let mut buffer = [0_u8; 8 * 1024];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        request.extend_from_slice(&buffer[..bytes_read]);

        if request.len() > 1024 * 1024 {
            break;
        }

        let Some(header_end) = find_header_end(&request) else {
            continue;
        };
        let headers = String::from_utf8_lossy(&request[..header_end]);
        let body_start = header_end + 4;
        let content_length = content_length(&headers).unwrap_or(0);
        if request.len() >= body_start + content_length {
            break;
        }
    }

    Ok(String::from_utf8_lossy(&request).to_string())
}

fn find_header_end(request: &[u8]) -> Option<usize> {
    request.windows(4).position(|window| window == b"\r\n\r\n")
}

fn content_length(headers: &str) -> Option<usize> {
    for line in headers.lines() {
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                return value.trim().parse().ok();
            }
        }
    }
    None
}

fn request_target(request: &str) -> Option<(&str, &str)> {
    let mut parts = request.lines().next()?.split_whitespace();
    let method = parts.next()?;
    let target = parts.next()?;
    Some((method, target))
}

fn query_value(path: &str, key: &str) -> Option<String> {
    let query = path.split_once('?')?.1;
    for pair in query.split('&') {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        if name == key {
            return Some(percent_decode(value));
        }
    }
    None
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&value[index + 1..index + 3], 16) {
                output.push(byte);
                index += 3;
                continue;
            }
        }
        output.push(if bytes[index] == b'+' {
            b' '
        } else {
            bytes[index]
        });
        index += 1;
    }
    String::from_utf8_lossy(&output).to_string()
}

fn json_string_field(body: &str, field: &str) -> Option<String> {
    let marker = format!("\"{field}\"");
    let field_start = body.find(&marker)?;
    let after_field = &body[field_start + marker.len()..];
    let colon = after_field.find(':')?;
    let after_colon = after_field[colon + 1..].trim_start();
    json_string_value(after_colon)
}

fn first_params_string(body: &str) -> Option<String> {
    params_string_at(body, 0)
}

fn params_string_at(body: &str, index: usize) -> Option<String> {
    let values = params_values(body)?;
    let value = values.get(index)?.trim();
    json_string_value(value)
}

fn params_bool_at(body: &str, index: usize) -> Option<bool> {
    let values = params_values(body)?;
    match values.get(index)?.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn params_values(body: &str) -> Option<Vec<String>> {
    let marker = "\"params\"";
    let params_start = body.find(marker)?;
    let after_params = &body[params_start + marker.len()..];
    let open_bracket = after_params.find('[')?;
    let mut values = Vec::new();
    let mut current = String::new();
    let mut array_depth = 0_i32;
    let mut object_depth = 0_i32;
    let mut in_string = false;
    let mut escaped = false;

    for character in after_params[open_bracket + 1..].chars() {
        if in_string {
            current.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        match character {
            '"' => {
                in_string = true;
                current.push(character);
            }
            '[' => {
                array_depth += 1;
                current.push(character);
            }
            ']' if array_depth > 0 => {
                array_depth -= 1;
                current.push(character);
            }
            ']' if object_depth == 0 => {
                let value = current.trim();
                if !value.is_empty() {
                    values.push(value.to_string());
                }
                return Some(values);
            }
            '{' => {
                object_depth += 1;
                current.push(character);
            }
            '}' => {
                object_depth -= 1;
                current.push(character);
            }
            ',' if array_depth == 0 && object_depth == 0 => {
                values.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(character),
        }
    }

    None
}

fn json_string_value(input: &str) -> Option<String> {
    let rest = input.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn json_id(body: &str) -> Option<String> {
    let marker = "\"id\"";
    let id_start = body.find(marker)?;
    let after_id = &body[id_start + marker.len()..];
    let colon = after_id.find(':')?;
    let after_colon = after_id[colon + 1..].trim_start();

    if let Some(value) = json_string_value(after_colon) {
        return Some(format!("\"{}\"", escape_json(&value)));
    }

    let end = after_colon
        .find(|character: char| character == ',' || character == '}')
        .unwrap_or(after_colon.len());
    let raw = after_colon[..end].trim();
    if raw.is_empty() {
        None
    } else {
        Some(raw.to_string())
    }
}

fn json_result_string(id: &str, result: &str) -> String {
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":\"{}\"}}",
        id,
        escape_json(result)
    )
}

fn json_result_raw(id: &str, result: &str) -> String {
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}",
        id, result
    )
}

fn json_error(id: &str, code: i64, message: &str) -> String {
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
        id,
        code,
        escape_json(message)
    )
}

fn faucet_error(message: &str) -> String {
    format!(
        "{{\"status\":\"error\",\"message\":\"{}\",\"warning\":\"{}\"}}",
        escape_json(message),
        escape_json(FAUCET_WARNING)
    )
}

fn faucet_status_json(address: Address, month_key: &str, claimed: u128) -> String {
    let remaining = FAUCET_MONTHLY_LIMIT.saturating_sub(claimed);
    format!(
        "{{\
         \"status\":\"ok\",\
         \"address\":\"{}\",\
         \"month\":\"{}\",\
         \"monthlyLimit\":\"{}\",\
         \"maxClaim\":\"{}\",\
         \"claimedThisMonth\":\"{}\",\
         \"remainingThisMonth\":\"{}\",\
         \"symbol\":\"tINVX\",\
         \"warning\":\"{}\"\
         }}",
        address,
        escape_json(month_key),
        FAUCET_MONTHLY_LIMIT,
        FAUCET_MAX_CLAIM,
        claimed,
        remaining,
        escape_json(FAUCET_WARNING)
    )
}

fn faucet_history_json(
    storage: &MemoryStorage,
    limit: usize,
    address_filter: Option<Address>,
) -> String {
    let faucet_address = Address::repeat(0x22);
    let mut entries = Vec::new();

    for block in storage.blocks() {
        let block_hash_value = block_hash(block);
        for transaction in &block.transactions {
            if transaction.from != faucet_address {
                continue;
            }
            let Some(wallet_address) = transaction.to else {
                continue;
            };
            if address_filter.is_some_and(|address| address != wallet_address) {
                continue;
            }
            entries.push(faucet_history_entry_json(
                &transaction.hash(),
                wallet_address,
                transaction.value.0,
                block.header.number,
                &block_hash_value,
                block.header.timestamp_ms,
            ));
        }
    }

    entries.reverse();
    entries.truncate(limit);

    format!(
        "{{\
         \"status\":\"ok\",\
         \"transactions\":[{}],\
         \"count\":{},\
         \"filteredAddress\":{},\
         \"symbol\":\"tINVX\",\
         \"warning\":\"{}\"\
         }}",
        entries.join(","),
        entries.len(),
        address_filter
            .map(|address| format!("\"{}\"", address))
            .unwrap_or_else(|| "null".to_string()),
        escape_json(FAUCET_WARNING)
    )
}

fn faucet_history_entry_json(
    transaction_hash: &Hash,
    wallet_address: Address,
    amount: u128,
    block_number: u64,
    block_hash: &Hash,
    timestamp_ms: u64,
) -> String {
    format!(
        "{{\
         \"transactionHash\":\"{}\",\
         \"walletAddress\":\"{}\",\
         \"amount\":\"{}\",\
         \"blockNumber\":\"0x{:x}\",\
         \"blockNumberDecimal\":{},\
         \"blockHash\":\"{}\",\
         \"timestampMs\":{},\
         \"symbol\":\"tINVX\"\
         }}",
        transaction_hash,
        wallet_address,
        amount,
        block_number,
        block_number,
        block_hash,
        timestamp_ms
    )
}

fn faucet_claim_json(
    address: Address,
    transaction_hash: &Hash,
    amount: u128,
    month_key: &str,
    claimed: u128,
    block_number: u64,
    block_hash: Hash,
    timestamp_ms: u64,
) -> String {
    let remaining = FAUCET_MONTHLY_LIMIT.saturating_sub(claimed);
    format!(
        "{{\
         \"status\":\"ok\",\
         \"address\":\"{}\",\
         \"transactionHash\":\"{}\",\
         \"amount\":\"{}\",\
         \"blockNumber\":\"0x{:x}\",\
         \"blockNumberDecimal\":{},\
         \"blockHash\":\"{}\",\
         \"timestampMs\":{},\
         \"month\":\"{}\",\
         \"monthlyLimit\":\"{}\",\
         \"claimedThisMonth\":\"{}\",\
         \"remainingThisMonth\":\"{}\",\
         \"symbol\":\"tINVX\",\
         \"warning\":\"{}\"\
         }}",
        address,
        transaction_hash,
        amount,
        block_number,
        block_number,
        block_hash,
        timestamp_ms,
        escape_json(month_key),
        FAUCET_MONTHLY_LIMIT,
        claimed,
        remaining,
        escape_json(FAUCET_WARNING)
    )
}

fn mine_transaction(storage: &mut MemoryStorage, transaction: Transaction) -> Hash {
    let transaction_hash = transaction.hash();
    let next_block_number = storage
        .latest_block()
        .map(|block| block.header.number + 1)
        .unwrap_or(1);
    let block = Block {
        header: BlockHeader {
            parent_hash: Hash::ZERO,
            number: next_block_number,
            state_root: Hash::ZERO,
            tx_root: transaction_hash,
            proposer: Address::repeat(0xaa),
            timestamp_ms: now_ms(),
        },
        transactions: vec![transaction],
    };
    storage.push_block(block);
    transaction_hash
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn parse_hash(value: &str) -> Option<Hash> {
    let hex = value.strip_prefix("0x").unwrap_or(value);
    if hex.len() != 64 {
        return None;
    }

    let mut bytes = [0_u8; 32];
    for index in 0..32 {
        let start = index * 2;
        let end = start + 2;
        bytes[index] = u8::from_str_radix(&hex[start..end], 16).ok()?;
    }

    Some(Hash(bytes))
}

fn parse_quantity(value: &str) -> Option<u64> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix("0x") {
        return u64::from_str_radix(hex, 16).ok();
    }
    value.parse::<u64>().ok()
}

fn block_by_tag<'a>(storage: &'a MemoryStorage, tag: &str) -> Option<&'a Block> {
    match tag {
        "latest" | "pending" | "safe" | "finalized" => storage.latest_block(),
        "earliest" => storage.blocks().next(),
        value => parse_quantity(value).and_then(|number| storage.block_by_number(number)),
    }
}

fn block_hash(block: &Block) -> Hash {
    Hash::from_bytes(format!("{:?}", block.header).as_bytes())
}

fn block_json(
    block: &Block,
    storage: &MemoryStorage,
    chain_id_hex: &str,
    full_transactions: bool,
) -> String {
    let hash = block_hash(block);
    let gas_used: u64 = block
        .transactions
        .iter()
        .map(|transaction| transaction.gas_limit)
        .sum();
    let transactions = if full_transactions {
        block
            .transactions
            .iter()
            .filter_map(|transaction| {
                let hash = transaction.hash();
                let receipt = storage.receipt(&hash)?;
                Some(transaction_json(transaction, receipt, chain_id_hex))
            })
            .collect::<Vec<_>>()
            .join(",")
    } else {
        block
            .transactions
            .iter()
            .map(|transaction| format!("\"{}\"", transaction.hash()))
            .collect::<Vec<_>>()
            .join(",")
    };

    format!(
        "{{\
         \"number\":\"0x{:x}\",\
         \"hash\":\"{}\",\
         \"parentHash\":\"{}\",\
         \"nonce\":null,\
         \"sha3Uncles\":\"{}\",\
         \"logsBloom\":\"0x{}\",\
         \"transactionsRoot\":\"{}\",\
         \"stateRoot\":\"{}\",\
         \"receiptsRoot\":\"{}\",\
         \"miner\":\"{}\",\
         \"difficulty\":\"0x0\",\
         \"totalDifficulty\":\"0x0\",\
         \"extraData\":\"0x\",\
         \"size\":\"0x0\",\
         \"gasLimit\":\"0x1c9c380\",\
         \"gasUsed\":\"0x{:x}\",\
         \"timestamp\":\"0x{:x}\",\
         \"transactions\":[{}],\
         \"uncles\":[],\
         \"baseFeePerGas\":\"0x1\"\
         }}",
        block.header.number,
        hash,
        block.header.parent_hash,
        Hash::ZERO,
        "0".repeat(512),
        block.header.tx_root,
        block.header.state_root,
        Hash::ZERO,
        block.header.proposer,
        gas_used,
        block.header.timestamp_ms / 1000,
        transactions
    )
}

fn transaction_by_block_index_json(
    id: &str,
    storage: &MemoryStorage,
    block: &Block,
    index: u64,
    chain_id_hex: &str,
) -> String {
    let Some(transaction) = block.transactions.get(index as usize) else {
        return json_result_raw(id, "null");
    };
    let transaction_hash = transaction.hash();
    let Some(receipt) = storage.receipt(&transaction_hash) else {
        return json_result_raw(id, "null");
    };
    json_result_raw(id, &transaction_json(transaction, receipt, chain_id_hex))
}

fn transaction_json(
    transaction: &Transaction,
    receipt: &TransactionReceipt,
    chain_id_hex: &str,
) -> String {
    let to = transaction
        .to
        .map(|address| format!("\"{}\"", address))
        .unwrap_or_else(|| "null".to_string());

    format!(
        "{{\
         \"hash\":\"{}\",\
         \"nonce\":\"0x{:x}\",\
         \"blockHash\":\"{}\",\
         \"blockNumber\":\"0x{:x}\",\
         \"transactionIndex\":\"0x{:x}\",\
         \"from\":\"{}\",\
         \"to\":{},\
         \"value\":\"0x{:x}\",\
         \"gas\":\"0x{:x}\",\
         \"gasPrice\":\"0x{:x}\",\
         \"input\":\"0x{}\",\
         \"type\":\"0x0\",\
         \"chainId\":\"{}\",\
         \"v\":\"0x0\",\
         \"r\":\"0x0\",\
         \"s\":\"0x0\"\
         }}",
        transaction.hash(),
        transaction.nonce,
        receipt.block_hash,
        receipt.block_number,
        receipt.transaction_index,
        transaction.from,
        to,
        transaction.value.0,
        transaction.gas_limit,
        transaction.gas_price.0,
        encode_hex(&transaction.input),
        chain_id_hex
    )
}

fn receipt_json(receipt: &TransactionReceipt) -> String {
    let to = receipt
        .to
        .map(|address| format!("\"{}\"", address))
        .unwrap_or_else(|| "null".to_string());
    let status = if receipt.status { "0x1" } else { "0x0" };

    format!(
        "{{\
         \"transactionHash\":\"{}\",\
         \"transactionIndex\":\"0x{:x}\",\
         \"blockNumber\":\"0x{:x}\",\
         \"blockHash\":\"{}\",\
         \"from\":\"{}\",\
         \"to\":{},\
         \"cumulativeGasUsed\":\"0x{:x}\",\
         \"gasUsed\":\"0x{:x}\",\
         \"contractAddress\":null,\
         \"logs\":[],\
         \"logsBloom\":\"0x{}\",\
         \"status\":\"{}\"\
         }}",
        receipt.transaction_hash,
        receipt.transaction_index,
        receipt.block_number,
        receipt.block_hash,
        receipt.from,
        to,
        receipt.cumulative_gas_used,
        receipt.gas_used,
        "0".repeat(512),
        status
    )
}

fn decode_dev_transfer_transaction(raw_transaction: &str) -> Result<Transaction, &'static str> {
    let bytes = decode_hex(raw_transaction).ok_or("Raw transaction must be valid hex")?;
    let payload = String::from_utf8(bytes).map_err(|_| "Raw transaction must be UTF-8")?;
    let parts: Vec<&str> = payload.split(':').collect();

    if parts.len() != 6 || parts[0] != "infi" || parts[1] != "transfer" {
        return Err("Unsupported dev raw transaction format");
    }

    let from = Address::from_str(parts[2]).map_err(|_| "Invalid sender address")?;
    let to = Address::from_str(parts[3]).map_err(|_| "Invalid recipient address")?;
    let value = parts[4]
        .parse::<u128>()
        .map_err(|_| "Invalid transfer value")?;
    let nonce = parts[5].parse::<u64>().map_err(|_| "Invalid nonce")?;

    Ok(Transaction::simple_transfer(
        from,
        to,
        Amount::from_invertx_units(value),
        nonce,
    ))
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    let hex = value.strip_prefix("0x").unwrap_or(value);
    if hex.len() % 2 != 0 {
        return None;
    }

    let mut output = Vec::with_capacity(hex.len() / 2);
    for index in (0..hex.len()).step_by(2) {
        output.push(u8::from_str_radix(&hex[index..index + 2], 16).ok()?);
    }

    Some(output)
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

fn current_month_key() -> String {
    let days_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| (duration.as_secs() / 86_400) as i64)
        .unwrap_or(0);
    let (year, month, _) = civil_from_days(days_since_epoch);
    format!("{year:04}-{month:02}")
}

fn civil_from_days(days_since_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };
    (year as i32, month as u32, day as u32)
}
