pub mod macros {
    macro_rules! create_match {
        ($window:expr) => {
            get_coin_price!(bitcoin);
            match bitcoin() {
                Ok(price) => {
                    let _price: String = format!("{:.2}", price);
                    $window.set_coin_price(slint::SharedString::from(_price.clone()));
                }
                Err(_) => {
                    println!("Failed to grab price!");
                }
            };
        };
    }
    pub(crate) use create_match;
}
