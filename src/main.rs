use match_module::macros::create_match;
use price_module::macros::get_coin_price;
pub mod match_module;
pub mod price_module;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            create_match!(ui);
        }
    });

    ui.run()
}
