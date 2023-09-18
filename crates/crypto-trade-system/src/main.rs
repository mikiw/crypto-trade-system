use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

use chrono::prelude::*;
use std::{thread, time::Duration};

extern crate ether_converter;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Debug)]
struct Transactions {
    result: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    blockNumber: String, // TODO: implement custom deserialization with appropriate types
    timeStamp: String,
    from: String,
    to: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EthPrice {
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Price,
}

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    usd: f32,
}

impl Transactions {
    fn result_iter(&self) -> impl IntoIterator<Item = &Transaction> {
        self.result.iter()
    }
}

fn timestamp_to_date(unix_timestamp: &String) -> DateTime<Utc> {
    let timestamp = unix_timestamp.parse::<i64>().unwrap();  // TODO: change to match to avoid panic
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    datetime
}

async fn buy(transactions: &Transactions, address: &str) -> Result<(f32, f32)> {

	let mut sum_buy_value = 0.0;
	let mut sum_buy_usd = 0.0;

    for t in transactions.result_iter() {
        if t.to == address {
            let price = get_price(&t.timeStamp).await?;
            let map = ether_converter::convert(&t.value, "wei");
            let value: f32 = map.get("ether").unwrap().parse().unwrap(); // TODO: change to match to avoid panic

            // println!("price");
            // println!("{}", price.market_data.current_price.usd);
            // println!("value");
            // println!(value);
            
            sum_buy_value += value;
            sum_buy_usd += price.market_data.current_price.usd * value;

            println!("[{}] buy value in usd: {}", &t.timeStamp, price.market_data.current_price.usd * value);
        }
    }

	Ok((sum_buy_value, sum_buy_usd))
}

async fn sell(transactions: &Transactions, address: &str) -> Result<(f32, f32)> {

	let mut sum_sell_value = 0.0;
	let mut sum_sell_usd = 0.0;

    for t in transactions.result_iter() {
        if t.from == address {
            let price = get_price(&t.timeStamp).await?;
            let map = ether_converter::convert(&t.value, "wei");
            let value: f32 = map.get("ether").unwrap().parse().unwrap(); // TODO: change to match to avoid panic

            sum_sell_value += value;
            sum_sell_usd += price.market_data.current_price.usd * value;

            println!("[{}] sell value in usd: {}", &t.timeStamp, price.market_data.current_price.usd * value);
        }
    }

	Ok((sum_sell_value, sum_sell_usd))
}

async fn get_transactions(address: &str, api_key: &str) -> Result<Transactions> {

    let request_url = format!("http://api.etherscan.io/api?module=account&action=txlist&sort=desc&address={address}&apikey={api_key}",
                            address = address,
                            api_key = api_key);

    let http_response = reqwest::get(request_url).await?;
    let response = http_response.json::<Transactions>().await?;

    Ok(response)
}

async fn get_price(unix_timestamp: &String) -> Result<EthPrice> {
    let date = timestamp_to_date(unix_timestamp).format("%d-%m-%Y");
    let request_url = format!("https://api.coingecko.com/api/v3/coins/ethereum/history?date={date}");
    let http_response = reqwest::get(request_url).await?;
    let response = http_response.json::<EthPrice>().await?;
    thread::sleep(Duration::from_millis(1000)); // Thread sleep to avoid HTTP 429 [Too Many Requests]

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Syncing prices from_start date to end_date in file on startup
    // TODO: Split program into modules.
    // TODO: Add MACD.
    // TODO: Add toolkits settings, formatting etc.
    // TODO: Add tests.

    let test = types::add(1,2);
    println!("{}", test);

    let api_key = "";
    let address = "0x8547db9f02643312a16643cb6733a9de19b51415";

    println!("Your address: {}", address);

    let transactions = get_transactions(address, api_key).await?;
    let buy = buy(&transactions, address).await?;
    let sell = sell(&transactions, address).await?;

    println!("sum_buy_value: {}", buy.0);
    println!("sum_buy_usd: {}", buy.1);
    println!("");
    
    println!("sum_sell_value: {}", sell.0);
    println!("sum_sell_usd: {}", sell.1);
    println!("");

    println!("Income in USD: {}", sell.1 - buy.1);

    Ok(())
}