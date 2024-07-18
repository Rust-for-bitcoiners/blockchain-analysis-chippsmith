use bitcoin::{Block, OutPoint, Transaction, TxOut};
use bitcoincore_rpc::{
    json::{GetRawTransactionResult, GetRawTransactionResultVout},
    Client, RpcApi,
};


//delcins code for reference 
#[derive(Debug)]
pub struct TxId(String);
pub fn high_fee_transaction(block: Block, client: &Client) -> Option<TxId> {
    let txs = block.txdata;

    if txs.len() == 1 {
        return None; // Only coinbase transaction
    }

    let mut max_fee = 0;
    let mut max_fee_txid = "".to_owned();

    for tx in txs {
        if tx.is_coinbase() {
            continue;
        }

        // Find the total value of outputs
        // let total = tx.output.iter().map(|txout| txout.value.to_sat()).sum();
        let total_output_value: u64 = tx
            .output
            .iter()
            .fold(0, |acc, txout| acc + txout.value.to_sat());
        // Find the total value of inputs
        let mut total_input_value: u64 = 0;
        tx.input.iter().for_each(|txin| {
            let previous_outpoint: OutPoint = txin.previous_output;
            let previous_transaction: Transaction = client
                .get_raw_transaction(&previous_outpoint.txid, None)
                .expect("The bitcoin node does not support transaction indexing or the given block has invalid data");
            let previous_output: &TxOut = &previous_transaction.output[previous_outpoint.vout as usize];
            total_input_value += previous_output.value.to_sat();
        });

        // let total_input_value: u64 = tx.input.iter().map(|txin| {
        //     let previous_outpoint = txin.previous_output;
        //     let previous_transaction: GetRawTransactionResult = client
        //         .get_raw_transaction_info(&previous_outpoint.txid, None)
        //         .expect("The bitcoin node does not support transaction indexing or the given block has invalid data");
        //     let previous_output: &GetRawTransactionResultVout = &previous_transaction.vout[previous_outpoint.vout as usize];
        //     previous_output.value.to_sat() // input value of this txin
        // }).sum();

        let fee = total_input_value - total_output_value;

        if fee > max_fee {
            max_fee = fee;
            max_fee_txid = tx.compute_txid().to_string();
        }
    }
    println!("fee {:?}", max_fee);
    Some(TxId(max_fee_txid))
}
