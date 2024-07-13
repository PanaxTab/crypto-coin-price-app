pub mod macros {
    macro_rules! get_coin_price {
        ($coin_name:ident) => {
            use rust_decimal::prelude::*;
            use serde::Deserialize;

            #[derive(Deserialize, Debug)]
            struct PriceResponse {
                data: CryptoRate,
            }
            #[allow(non_snake_case)]
            #[derive(Deserialize, Debug)]
            struct CryptoRate {
                rateUsd: String,
            }

            fn $coin_name() -> Result<Decimal, Box<dyn std::error::Error>> {
                let mut url: String = "https://api.coincap.io/v2/rates/".to_owned();
                let borrowed_string: &str = stringify!($coin_name);
                url.push_str(borrowed_string);
                let resp = reqwest::blocking::get(format!("{url}"))?;

                let body: PriceResponse = resp.json::<PriceResponse>()?;

                let price: Decimal = match Decimal::from_str(&body.data.rateUsd) {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Error on converting!");
                        let a = Decimal::new(0, 1);
                        a
                    }
                };
                Ok(price)
            }
        };
    }
    pub(crate) use get_coin_price;
}
