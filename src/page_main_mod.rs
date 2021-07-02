// page_main_mod.rs

// use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::on_click;
use crate::utils_mod as ut;
use crate::web_sys_mod as w;

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_main() {
    // fetch page_main.html and inject it
    let resp_body_text = w::fetch_response("pages/page_main.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = ut::get_delimited_text(&resp_body_text, 0, "<body>", "</body>").unwrap();
    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);

    // region: binding - read from config
    w::set_text("div_input_number", &crate::currdb_config_mod::get_input_number().await);
    let input_unit = &crate::currdb_config_mod::get_input_currency().await;
    let output_unit = &crate::currdb_config_mod::get_output_currency().await;
    let exchange_rate = &crate::currdb_config_mod::get_rate().await;
    let exchange_rate = exchange_rate.parse::<f64>().unwrap();

    w::set_text("div_input_unit", input_unit);
    let short_rate = format!("{:.3}", exchange_rate);
    let full_exchange_rate = format!("1 {} = {} {}", input_unit, short_rate, output_unit);
    w::set_text("div_exchange_rate", &full_exchange_rate);

    let date_fetch = &crate::currdb_config_mod::get_date_fetch().await;
    w::set_text("div_rate_date", date_fetch);
    w::set_text("div_output_unit", output_unit);

    convert();
    // endregion: binding - read from config

    // region: event handlers
    on_click!("div_1", div_cell_on_click);
    on_click!("div_2", div_cell_on_click);
    on_click!("div_3", div_cell_on_click);
    on_click!("div_4", div_cell_on_click);
    on_click!("div_5", div_cell_on_click);
    on_click!("div_6", div_cell_on_click);
    on_click!("div_7", div_cell_on_click);
    on_click!("div_8", div_cell_on_click);
    on_click!("div_9", div_cell_on_click);
    on_click!("div_0", div_cell_on_click);

    on_click!("div_dot", div_cell_dot_on_click);
    on_click!("div_backspace", div_backspace_on_click);
    on_click!("div_clear", div_c_on_click);

    on_click!("div_rate_date", div_rate_date_on_click);

    on_click!("div_input_unit", div_input_unit_on_click);
    on_click!("div_output_unit", div_output_unit_on_click);

    on_click!("div_hamburger_button", div_hamburger_button_on_click);
    on_click!("div_reverse", div_reverse_on_click);
    on_click!("div_exchange_rate", div_exchange_rate_on_click);

    // endregion: event handlers
}

/// reload json from floatrates.com and save to indexeddb
pub fn div_rate_date_on_click(_element_id: &str) {
    show_snackbar();

    spawn_local(async {
        crate::fetch_rates_mod::fetch_and_save().await;
        crate::fetch_rates_mod::modify_rate().await;
    });
}

pub fn show_snackbar() {
    // Get the snackbar DIV
    let element = w::get_element_by_id("snackbar");
    // Add the "show" class to DIV
    element.set_class_name("show");
    // After 3 seconds, remove the show class from DIV
    let closure = Closure::wrap(Box::new(move || {
        w::debug_write("Timeout closure.");
        let class_name = element.class_name();
        let class_name = class_name.replace("show", "");
        element.set_class_name(&class_name);
    }) as Box<dyn Fn()>);

    w::window().set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 3000).unwrap();
    closure.forget();
}

/// event handler for 0-9 numeric cells
/// if input is only 0 then remove 0
pub fn div_cell_on_click(element_id: &str) {
    let mut input = w::get_text("div_input_number");
    let text = w::get_text(element_id);
    if input == "0" {
        input.clear();
    }
    input.push_str(&text);
    w::set_text("div_input_number", &input);
    convert();
}

/// decimal dot can be used only once
pub fn div_cell_dot_on_click(_element_id: &str) {
    let mut input = w::get_text("div_input_number");
    if !input.contains(".") {
        input.push('.');
        w::set_text("div_input_number", &input);
        convert();
    }
}

/// event handler
/// input must never be incorrect f64
/// instead of clearing to empty, it makes it 0 zero
pub fn div_backspace_on_click(_element_id: &str) {
    let mut input = w::get_text("div_input_number");
    input.pop().unwrap();
    if input.is_empty() {
        input.push('0');
    }
    w::set_text("div_input_number", &input);
    convert();
}

/// event handler
/// input must never be incorrect f64
/// clear makes it 0 zero
pub fn div_c_on_click(_element_id: &str) {
    w::set_text("div_input_number", "0");
    w::set_text("div_output_number", "0");
}

/// convert Currency with exchange rates
/// input cannot never be incorrect f64
fn convert() {
    spawn_local(async move {
        let input_number = w::get_text("div_input_number");
        crate::currdb_config_mod::set_input_number(&input_number).await;

        let exchange_rate = &crate::currdb_config_mod::get_rate().await;
        let exchange_rate = exchange_rate.parse::<f64>().unwrap();
        let input_number = input_number.parse::<f64>().unwrap();
        let output = format!("{:.3}", input_number * exchange_rate);
        w::set_text("div_output_number", &output);
    });
}

/// opens the page_input_currency_used
fn div_input_unit_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_input_currency_used_mod::page_input_currency_used().await;
    });
}

/// opens the page_output_currency
fn div_output_unit_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_output_currency_used_mod::page_output_currency_used().await;
    });
}

/// opens the page_modal_about
fn div_hamburger_button_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_modal_about_mod::page_modal_about().await;
    });
}

/// reverse the input and output currency. It fetches the new rates.
fn div_reverse_on_click(_element_id: &str) {
    spawn_local(async {
        let old_output_number = w::get_text("div_output_number");
        crate::currdb_config_mod::set_input_number(&old_output_number).await;

        let input_unit = &crate::currdb_config_mod::get_input_currency().await;
        let output_unit = &crate::currdb_config_mod::get_output_currency().await;
        crate::currdb_config_mod::set_input_currency(output_unit).await;
        crate::currdb_config_mod::set_output_currency(input_unit).await;
        crate::fetch_rates_mod::fetch_and_save().await;
        crate::fetch_rates_mod::modify_rate().await;

        crate::page_main_mod::page_main().await;
    });
}
/// opens the manual rates
fn div_exchange_rate_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_manual_rates_mod::page_manual_rates().await;
    });
}
