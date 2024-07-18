use bitcoincore_rpc::{json::GetMempoolInfoResult, Client, Error, RpcApi};

pub fn inspect_mempool(client: &Client) -> Result<GetMempoolInfoResult, Error> {
    //let mempool_txs = client.get_raw_mempool()?;

    let details = client.get_mempool_info()?;
    Ok(details)
}
