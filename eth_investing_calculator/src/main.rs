use std::io;
use serde::Deserialize;
use chrono::prelude::*;

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

#[derive(Deserialize, Debug)]
struct EthPrice {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData,
}

#[derive(Deserialize, Debug)]
struct MarketData {
    current_price: Price,
}

#[derive(Deserialize, Debug)]
struct Price {
    usd: f32,
}

fn timestamp_to_date(unix_timestamp: String) -> DateTime<Utc> {
    let timestamp = unix_timestamp.parse::<i64>().unwrap();
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    datetime
}

async fn get_transactions(address: String, api_key: String) -> Result<Transactions> {
    let request_url = format!("http://api.etherscan.io/api?module=account&action=txlist&sort=desc&address={address}&apikey={api_key}",
                            address = address,
                            api_key = api_key);

    let http_response = reqwest::get(request_url).await?;
    let response = http_response.json::<Transactions>().await?;

    Ok(response)
}

async fn get_price(unix_timestamp: String) -> Result<EthPrice> {
    let request_url = format!("https://api.coingecko.com/api/v3/coins/ethereum/history?date={date}",
                            date = timestamp_to_date(unix_timestamp).format("%d-%m-%Y"));

    let http_response = reqwest::get(request_url).await?;
    let response = http_response.json::<EthPrice>().await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Convert eth value as decimal value.
    // TODO: Calculate inputs (buy) and outputs (sell).

    let mut api_key = String::from("");
    let mut address = String::from("0xc55dbe3cd4afa41e8c24283c5be8d2481e2b79c1");

    println!("Input your ETH address or press enter to get transactions from {} address.", address);
    io::stdin()
        .read_line(&mut address)
        .expect("Failed to read line");

    println!("Your address: {}", address);

    let transactions = get_transactions(address, api_key).await?;

    for i in transactions.result {
        println!("Transaction: ");
        println!("{:#?}", i.value);
        println!("{:#?}", i.from);
        println!("{:#?}", i.to);

        let price = get_price(i.timeStamp).await?;
        println!("{:#?}", price);

    }

    Ok(())
}