use std::env;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use infi_consensus::{ConsensusConfig, DevnetConsensus};
use infi_evm::EvmExecutor;
use infi_mempool::Mempool;
use infi_primitives::{Address, Amount, ChainConfig, Hash, Transaction};
use infi_rpc::{devnet_rpc_info, RpcServer};
use infi_storage::{ChainStorage, MemoryStorage};

fn main() {
    let config = ChainConfig::devnet();
    let rpc = devnet_rpc_info(&config);

    println!("Starting {}", config.chain_name);
    println!(
        "Native gas token: {} ({})",
        config.native_token.name, config.native_token.symbol
    );
    println!("Chain ID: {} ({})", config.chain_id, rpc.chain_id_hex);
    println!("Client: {}", rpc.client_version);

    let alice = Address::repeat(0x11);
    let bob = Address::repeat(0x22);

    let mut storage = MemoryStorage::new();
    storage.credit(
        alice,
        Amount::from_invertx_units(1_000_000_000_000_000_000_000),
    );
    storage.credit(
        bob,
        Amount::from_invertx_units(1_000_000_500_000_000_000_000_000_000),
    );

    let mut mempool = Mempool::new();
    mempool.submit(Transaction::simple_transfer(
        alice,
        bob,
        Amount::from_invertx_units(25_000_000_000_000_000_000),
        0,
    ));

    let consensus = DevnetConsensus::new(ConsensusConfig::single_validator_devnet());
    let transactions = mempool.drain_for_block(consensus.max_transactions_per_block());

    for transaction in &transactions {
        match EvmExecutor::execute_transaction(&mut storage, transaction) {
            Ok(receipt) => {
                println!(
                    "Executed tx {} from {} gas_used={}",
                    transaction.hash(),
                    transaction.from,
                    receipt.gas_used
                );
            }
            Err(error) => {
                println!("Rejected tx from {}: {:?}", transaction.from, error);
            }
        }
    }

    let block = consensus.propose_block(Hash::ZERO, 1, now_ms(), transactions);
    storage.push_block(block);

    let latest_block_number = storage
        .latest_block()
        .map(|block| block.header.number)
        .unwrap_or(0);
    println!("Finalized block #{}", latest_block_number);
    println!("Accounts:");
    for account in storage.accounts() {
        println!(
            "  {} balance={} nonce={}",
            account.address, account.balance, account.nonce
        );
    }

    let storage = Arc::new(Mutex::new(storage));
    let rpc_server = RpcServer::new(config, storage);
    let bind_address = rpc_bind_address();
    if let Err(error) = rpc_server.serve(&bind_address) {
        eprintln!("Failed to start INFI JSON-RPC server on {bind_address}: {error}");
        eprintln!("If the port is already in use locally, run: lsof -nP -i :8545");
        std::process::exit(1);
    }
}

fn rpc_bind_address() -> String {
    if let Ok(bind_address) = env::var("INFI_RPC_BIND") {
        let bind_address = bind_address.trim();
        if !bind_address.is_empty() {
            return bind_address.to_string();
        }
    }

    if let Ok(port) = env::var("PORT") {
        let port = port.trim();
        if !port.is_empty() {
            return format!("0.0.0.0:{port}");
        }
    }

    "127.0.0.1:8545".to_string()
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}
