use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_calculate_roi(move  | cost_price: SharedString, sell_price: SharedString, invest_price: SharedString | {
        let ui = ui_handle.unwrap();
        let cost_price_number = cost_price.parse::<f32>().unwrap();
        let sell_price_number = sell_price.parse::<f32>().unwrap();
        let invest_price_number = invest_price.parse::<f32>().unwrap();
        let profits = sell_price_number - cost_price_number;
        let mut rentability = invest_price_number / profits;

        if  rentability % 1.0 != 0.0 {
            rentability = rentability + 1.0;
        }

        ui.set_result(SharedString::from(format!("{:.0} ventes à réaliser", rentability)));
    });

    ui.run()
}
