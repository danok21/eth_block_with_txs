
use web3::types::{BlockId, U64, BlockNumber};
use std::io::Write;
use std::fs::File;
use std::path::Path;
use tokio::io::AsyncReadExt;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;


#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    let block_num = web3.eth().block_number().await?;
    println!("The number of the most recent block is : {:?}", block_num);

    let block_data = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(block_num)))).await?;
    println!("The {:?} block data is： {:?}", block_num, block_data);
    // let mut  text = File::create("./test.txt").expect("create failed");
    // write!(text,"{:?}",block_data).expect("write failed");//Write into file.

    if let Some(data) = block_data {
        let mut eth_block = serde_json::to_value(&data).unwrap();
        let mut json = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", json);

        //block_head
        let hash = eth_block["hash"].as_str().unwrap();
        let parent_hash = eth_block["parentHash"].as_str().unwrap();
        let sha3_uncles = eth_block["sha3Uncles"].as_str().unwrap();
        let miner = eth_block["miner"].as_str().unwrap();
        let state_root = eth_block["stateRoot"].as_str().unwrap();
        let transactions_root = eth_block["transactionsRoot"].as_str().unwrap();
        let receipts_root = eth_block["receiptsRoot"].as_str().unwrap();
        let number = eth_block["number"].as_str().unwrap();
        let gas_used = eth_block["gasUsed"].as_str().unwrap();
        let gas_limit = eth_block["gasLimit"].as_str().unwrap();
        let base_fee_per_gas = "null";
        let extra_data = eth_block["extraData"].as_str().unwrap();
        let logs_bloom = eth_block["logsBloom"].as_str().unwrap();
        let timestamp = eth_block["timestamp"].as_str().unwrap();
        let difficulty = eth_block["difficulty"].as_str().unwrap();
        let total_difficulty = eth_block["totalDifficulty"].as_str().unwrap();
        let seal_fields = "[]";
        let uncles = "[]";
        let size = eth_block["size"].as_str().unwrap();
        let mix_hash = "null";
        let nonce = "null";
        println!("{:?}",seal_fields);
        let tx = eth_block["transactions"].as_array().unwrap();//

        //block_tx
        for i in tx {
           // println!("{}", i);//single tx.

                let hash = i["hash"].as_str().unwrap();
                let nonce = i["nonce"].as_str().unwrap();
                let block_hash = i["blockHash"].as_str().unwrap();
                let block_number = i["blockNumber"].as_str().unwrap();
                let transaction_index = i["transactionIndex"].as_str().unwrap();
                let from_addr = i["from"].as_str().unwrap();
                let to_addr = i["to"].as_str().unwrap();
                let value = i["value"].as_str().unwrap();
                let gas_price = i["gasPrice"].as_str().unwrap();
                let gas = i["gas"].as_str().unwrap();
                let input = i["input"].as_str().unwrap();
                let v = i["v"].as_str().unwrap();
                let r = i["r"].as_str().unwrap();
                let s = i["s"].as_str().unwrap();
                let raw = i["raw"].as_str().unwrap();
            println!("{}",input);
        }

    }
        Ok(())
}