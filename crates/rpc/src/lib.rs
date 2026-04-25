use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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
    info: RpcInfo,
}

impl RpcServer {
    pub fn new(config: ChainConfig, storage: Arc<Mutex<MemoryStorage>>) -> Self {
        let info = devnet_rpc_info(&config);
        Self {
            config,
            storage,
            faucet: Arc::new(Mutex::new(FaucetState::new())),
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
        let mut buffer = [0_u8; 16 * 1024];
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let target = request_target(&request);
        let response_body = if request.starts_with("OPTIONS ") {
            String::new()
        } else if matches!(target, Some(("GET", "/health")) | Some(("GET", "/"))) {
            self.health_json()
        } else if let Some(("GET", path)) =
            target.filter(|(_, path)| path.starts_with("/faucet/status"))
        {
            self.handle_faucet_status(path)
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
            "eth_getTransactionCount" => self.handle_get_transaction_count(&id, body),
            "eth_getTransactionReceipt" => self.handle_get_transaction_receipt(&id, body),
            "eth_sendRawTransaction" => self.handle_send_raw_transaction(&id, body),
            _ => json_error(&id, -32601, "Method not found"),
        }
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
        let claimed_after = faucet.record_claim(recipient, &month_key, amount.0);
        faucet_claim_json(
            recipient,
            &transaction_hash,
            amount.0,
            &month_key,
            claimed_after,
        )
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
    let marker = "\"params\"";
    let params_start = body.find(marker)?;
    let after_params = &body[params_start + marker.len()..];
    let open_bracket = after_params.find('[')?;
    let after_bracket = after_params[open_bracket + 1..].trim_start();
    json_string_value(after_bracket)
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

fn faucet_claim_json(
    address: Address,
    transaction_hash: &Hash,
    amount: u128,
    month_key: &str,
    claimed: u128,
) -> String {
    let remaining = FAUCET_MONTHLY_LIMIT.saturating_sub(claimed);
    format!(
        "{{\
         \"status\":\"ok\",\
         \"address\":\"{}\",\
         \"transactionHash\":\"{}\",\
         \"amount\":\"{}\",\
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
