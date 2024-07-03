use module::macros::get_coin_price;
pub mod module;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            get_coin_price!(bitcoin);
            match bitcoin()
            {
                Ok(btc_price)=>
                {
                    println!("Hi!");
                    let btc_price:String = format!("{:.2}",btc_price);
                    ui.set_coin_price(slint::SharedString::from(btc_price.clone()));
                },
                Err(_)=>{
                    println!("Failed to grab price!");
                }
            };
        }
    });

    ui.run()
}
