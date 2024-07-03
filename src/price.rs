use serde::Deserialize;
use rust_decimal::prelude::*;


#[derive(Deserialize,Debug)]
struct PriceResponse
{
    data: CryptoRate,
}

#[allow(non_snake_case)]
#[derive(Deserialize,Debug)]
struct CryptoRate
{
    rateUsd: String,
}

pub fn get_btc_price() -> Result<Decimal, Box<dyn std::error::Error>> 
{
    let resp = reqwest::blocking::get("https://api.coincap.io/v2/rates/bitcoin")?;
    let body: PriceResponse = resp.json::<PriceResponse>()?;

    let price: Decimal = match Decimal::from_str(&body.data.rateUsd)
    {
        Ok(num)=> num,
        Err(_) =>
            {
                println!("Error on converting!");
                let a = Decimal::new(0,1);
                a
            },
    };
    Ok(price)
}

pub fn get_eth_price() -> Result<Decimal, Box<dyn std::error::Error>> 
{
    let resp = reqwest::blocking::get("https://api.coincap.io/v2/rates/ethereum")?;
    let body: PriceResponse = resp.json::<PriceResponse>()?;

    let price: Decimal = match Decimal::from_str(&body.data.rateUsd)
    {
        Ok(num)=> num,
        Err(_) =>
            {
                println!("Error on converting!");
                let a = Decimal::new(0,1);
                a
            },
    };
    Ok(price)
}

pub fn get_bnb_price() -> Result<Decimal, Box<dyn std::error::Error>> 
{
    let resp = reqwest::blocking::get("https://api.coincap.io/v2/rates/binance-coin")?;
    let body: PriceResponse = resp.json::<PriceResponse>()?;

    let price: Decimal = match Decimal::from_str(&body.data.rateUsd)
    {
        Ok(num)=> num,
        Err(_) =>
            {
                println!("Error on converting!");
                let a = Decimal::new(0,1);
                a
            },
    };
    Ok(price)
}
