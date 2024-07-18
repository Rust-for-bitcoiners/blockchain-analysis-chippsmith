use bitcoin::{Block};
use bitcoincore_rpc::{Client, RpcApi};

#[derive(Debug, PartialEq)]
pub enum Miners {
    Foundry,
    MaraPool,
    AntPool,
    F2Pool,
    SpiderPool,
    ViaBTC,
    Binance,
    KuCoinPool,
    Luxor,
    Other,
}

pub fn block_fee(block: Block, client: &Client) -> u64 {
    let txs = block.txdata;
    if txs.len() == 1 {
        0; // Only coinbase transaction
    }

    let mut total_block_fees = 0;
    for tx in txs {
        if tx.is_coinbase() {
            continue;
        }

        let total_output_value = tx
            .output
            .iter()
            .fold(0, |acc, txout| acc + txout.value.to_sat());

        let mut total_input_value = 0;
        tx.input.iter().for_each(|txin| {
            let prev_outpoint = txin.previous_output;
            let previous_transaction = client
                .get_raw_transaction(&prev_outpoint.txid, None)
                .unwrap();
            let previous_output = &previous_transaction.output[prev_outpoint.vout as usize];

            println!("{:?}", previous_output.value.to_sat());
            total_input_value += previous_output.value.to_sat();
        });

        let transaction_fee = total_input_value - total_output_value;
        total_block_fees += transaction_fee
    }
    total_block_fees
}

//takes byte slice and returns ascii characters
fn ascii(b: &[u8]) -> String {
    let mut s = String::new();

    for c in b {
        if *c > 32 && *c < 122 {
            s.push(*c as char)
        }
    }
    s
}

fn parse_string_for_miners(s: &String) -> Miners {
    if s.contains("MARAPool") {
        Miners::MaraPool
    } else if s.contains("Foundry") {
        Miners::Foundry
    } else if s.contains("AntPool") {
        Miners::AntPool
    } else if s.contains("F2Pool") {
        Miners::F2Pool
    } else if s.contains("SpiderPool") {
        Miners::SpiderPool
    } else if s.contains("ViaBTC") {
        Miners::ViaBTC
    } else if s.contains("binance") || s.contains("Binance") {
        Miners::Binance
    } else if s.contains("KuCoinPool") {
        Miners::KuCoinPool
    } else if s.contains("Luxor") {
        Miners::Luxor
    } else {
        Miners::Other
    }
}

pub fn who_mined_the_block(block: Block) -> Miners {
    let txs = block.txdata;
    let coinbase_tx = &txs[0];
    let s = ascii(coinbase_tx.input[0].script_sig.as_bytes());
    let miner = parse_string_for_miners(&s);
    miner
}
