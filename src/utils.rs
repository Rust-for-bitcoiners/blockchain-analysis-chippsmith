use bitcoin::Block;
use bitcoincore_rpc::{Client, Error, RpcApi};

pub fn get_block(height: u64, client: &Client) -> Result<Block, Error> {
    let block_hash = (client).get_block_hash(height)?;
    (client).get_block(&block_hash)
}
