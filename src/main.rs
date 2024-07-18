mod compute_block_fee;
mod compute_transaction_fee;
mod mempool;
mod utils;

use compute_block_fee::{block_fee, who_mined_the_block, Miners};
use compute_transaction_fee::high_fee_transaction;
use mempool::inspect_mempool;
use utils::get_block;

use bitcoin::{Block, Network, Txid};
use bitcoincore_rpc::{
    json,
    jsonrpc::{self},
    Auth, Client, RpcApi,
};
use chrono::Duration;
use std::{env, time};
#[macro_use]
extern crate lazy_static;




lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

// static client: Client = Client::new("url", Auth::UserPass("user".to_owned(), "password".to_owned())).unwrap();

// TODO: Task 1
fn time_to_mine(block_height: u64) -> Duration {
    // No way to know how long the first block took... best guess would be 600 seconds
    if block_height == 1 {
        return Duration::seconds(600);
    } else {
        let block_hash = (&*RPC_CLIENT).get_block_hash(block_height).unwrap();
        let block_header = (&*RPC_CLIENT).get_block_header(&block_hash).unwrap();
        let time = block_header.time as i64; // Due to loose rules on what miners can choose to put as their timestamp the difference in the timestamp from one block to the next can be negative so we need i64

        let prev_block_hash = (&*RPC_CLIENT).get_block_hash(block_height - 1).unwrap();
        let prev_block_header = (&*RPC_CLIENT).get_block_header(&prev_block_hash).unwrap();
        let prev_time: i64 = prev_block_header.time as i64;

        Duration::seconds(time - prev_time)
    }
}

// TODO: Task 2
fn number_of_transactions(block_height: u64) -> u16 {
    let block_hash = (&*RPC_CLIENT).get_block_hash(block_height).unwrap();
    println!("{}", block_hash);
    (&*RPC_CLIENT).get_block(&block_hash).unwrap().txdata.len() as u16
}

fn custom_timeout_rpc_client() -> Client {
    const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes
    dotenv::dotenv().ok();
    let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
    let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    let rpc_password: String =
        env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

    let custom_timeout_transport = jsonrpc::simple_http::Builder::new()
        .url(&rpc_url)
        .expect("invalid rpc url")
        .auth(rpc_user, Some(rpc_password))
        .timeout(TIMEOUT_UTXO_SET_SCANS)
        .build();
    let custom_timeout_rpc_client =
        jsonrpc::client::Client::with_transport(custom_timeout_transport);

    Client::from_jsonrpc(custom_timeout_rpc_client)
}

fn get_blockchain_info() -> bitcoincore_rpc::json::GetBlockchainInfoResult {
    let blockchain_info = (&*RPC_CLIENT).get_blockchain_info().unwrap();
    blockchain_info
}

fn main() {
    let mut miners = Vec::new();
    let current_block_height = get_blockchain_info().blocks;
    let past_block_height = current_block_height - 20;
    for block_height in past_block_height.. current_block_height {
        println!("{:?}", block_height);
        let block = get_block(block_height, &*RPC_CLIENT).expect("error fetching block");
        let miner = who_mined_the_block(block);
        miners.push(miner);
    }

    println!("{:?}", miners)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_to_mine_genesis_block() {
        assert_eq!(time_to_mine(1).num_seconds(), 600);
    }

    #[test]
    fn test_number_of_transactions_in_first_ten_blocks() {
        for i in 0..=10 {
            assert_eq!(number_of_transactions(i), 1)
        }
    }

    #[test]
    fn test_number_transactions() {
        assert_eq!(number_of_transactions(1), 1)
    }

    #[test]
    fn test_block_chain_info() {
        let info = get_blockchain_info();
        assert_eq!(info.chain, Network::Bitcoin)
    }

    //#[test]
    fn test_block_fee() {
        let block = get_block(500000, &*RPC_CLIENT).unwrap();
        let block_fee = block_fee(block, &*RPC_CLIENT);
        assert_eq!(block_fee, 339351625)
    }

    //#[test]
    fn test_high_fee_transaction() {
        let block = get_block(500000, &*RPC_CLIENT).unwrap();

        let txid = high_fee_transaction(block, &*RPC_CLIENT).unwrap();
        println!("{:?}", txid)
    }

    #[test]
    fn test_mempool() {
        let mempool_info = inspect_mempool(&*RPC_CLIENT).unwrap();
        //let mempool_info = 0;
        println!("{:?}", mempool_info);
    }

    #[test]
    fn test_who_mined_the_block() {
        let tests = [
            (1, Miners::Other),
            (800001, Miners::ViaBTC),
            (800007, Miners::Binance),
            (852638, Miners::Luxor),
            (852642, Miners::F2Pool),
            (852654, Miners::AntPool),
            (852656, Miners::SpiderPool),
            (852653, Miners::MaraPool),
            (852647, Miners::Foundry),
        ];

        for test in tests {
            let block_height = test.0;
            let block = get_block(block_height, &*RPC_CLIENT).expect("error fetching block");
            let miner = who_mined_the_block(block);
            assert_eq!(miner, test.1)
        }
        let block_height = 2;
        let block = get_block(block_height, &*RPC_CLIENT).expect("error fetching block");
        let miner = who_mined_the_block(block);

        assert_ne!(miner, Miners::Foundry)
    }
}
