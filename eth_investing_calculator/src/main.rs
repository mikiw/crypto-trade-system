use std::io;
use reqwest::Client;
use serde::Deserialize;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Debug)]
struct Transactions {
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

async fn get_data(address: String, api_key: String) -> Result<(Transactions)> {
    let request_url = format!("http://api.etherscan.io/api?module=account&action=txlist&sort=desc&address={address}&apikey={api_key}",
                            address = address,
                            api_key = api_key);

    let http_response = reqwest::get(request_url).await?;
    let response = http_response.json::<Transactions>().await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Convert eth value as decimal value.
    // TODO: Calculate inputs and outputs.
    // TODO: Get eth value as dollar in from timeStamp like 1640947396 UnixToDate.

    let mut api_key = String::from("");
    let mut address = String::from("0xc55dbe3cd4afa41e8c24283c5be8d2481e2b79c1");

    println!("Input your ETH address or press enter to get transactions from {} address.", address);
    io::stdin()
        .read_line(&mut address)
        .expect("Failed to read line");

    println!("Your address: {}", address);

    let data = get_data(address, api_key).await?;

    for i in data.result {
        println!("Transaction: ");
        println!("{:#?}", i.timeStamp);
        println!("{:#?}", i.value);
        println!("{:#?}", i.from);
        println!("{:#?}", i.to);
        println!("");
    }

    Ok(())
}