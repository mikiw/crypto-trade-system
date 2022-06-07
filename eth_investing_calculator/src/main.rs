use std::io;
use reqwest::Client;
use serde::Deserialize;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Debug)]
struct Response {
    status: String,
    message: String,
    result: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    blockNumber: String,
    timeStamp: String,
    hash: String,
    nonce: String,
    blockHash: String,
    transactionIndex: String,
    from: String,
    to: String,
    value: String,
    gas: String,
    gasPrice: String,
    isError: String,
    txreceipt_status: String,
    input: String,
    contractAddress: String,
    cumulativeGasUsed: String,
    gasUsed: String,
    confirmations: String,
}

async fn get_data() -> Result<()> {
    let request_url = format!("http://api.etherscan.io/api?module=account&action=txlist&sort=desc&address={address}&apikey={api_key}",
                            address = "",
                            api_key = "");

    let http_response = reqwest::get(request_url).await?;
    let response = http_response.json::<Response>().await?;
    println!("{:#?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Input your ETH address!");
    let mut address = String::new();
    io::stdin()
        .read_line(&mut address)
        .expect("Failed to read line");

    println!("Your address: {}", address);

    get_data().await?;

    Ok(())
}