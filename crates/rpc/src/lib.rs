use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use infi_primitives::{Address, ChainConfig, Hash};
use infi_storage::{ChainStorage, MemoryStorage, TransactionReceipt};

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
    info: RpcInfo,
}

impl RpcServer {
    pub fn new(config: ChainConfig, storage: Arc<Mutex<MemoryStorage>>) -> Self {
        let info = devnet_rpc_info(&config);
        Self {
            config,
            storage,
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
        let response_body = if request.starts_with("OPTIONS ") {
            String::new()
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
             Access-Control-Allow-Methods: POST, OPTIONS\r\n\
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
                let block_number = self
                    .storage
                    .lock()
                    .expect("RPC storage mutex poisoned")
                    .latest_block()
                    .map(|block| block.header.number)
                    .unwrap_or(0);
                json_result_string(&id, &format!("0x{block_number:x}"))
            }
            "eth_getBalance" => self.handle_get_balance(&id, body),
            "eth_getTransactionCount" => self.handle_get_transaction_count(&id, body),
            "eth_getTransactionReceipt" => self.handle_get_transaction_receipt(&id, body),
            "eth_sendRawTransaction" => json_error(
                &id,
                -32000,
                "Raw Ethereum transaction decoding is not implemented yet",
            ),
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

        let balance = self
            .storage
            .lock()
            .expect("RPC storage mutex poisoned")
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

        let nonce = self
            .storage
            .lock()
            .expect("RPC storage mutex poisoned")
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

        let storage = self.storage.lock().expect("RPC storage mutex poisoned");
        let Some(receipt) = storage.receipt(&transaction_hash) else {
            return json_result_raw(id, "null");
        };

        json_result_raw(id, &receipt_json(receipt))
    }
}

fn http_body(request: &str) -> Option<&str> {
    request.split("\r\n\r\n").nth(1)
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
    format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}", id, result)
}

fn json_error(id: &str, code: i64, message: &str) -> String {
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
        id,
        code,
        escape_json(message)
    )
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
