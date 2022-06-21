# EthInvestingCalculator
ETH income calculator in Rust is based on the assumption that an income transaction is a buy transaction at that time on the market and an outcome transaction is a sell.

To run it we can simply change the wallet address and add api_key:
```
let api_key = "TNYD******************************";
let address = "0xc55dbe3cd4afa41e8c24283c5be8d2481e2b79c1";
```

and type:
```
cd .\eth_investing_calculator\
cargo run
```

Result will be:
```
[1641801643] sell value in usd: 50.19578
[1640947695] sell value in usd: 122.71446
sum_buy_value: 1.3889345
sum_buy_usd: 3068.7747

sum_sell_value: 1.6399325
sum_sell_usd: 3467.0566

Income in USD: 398.28198
```