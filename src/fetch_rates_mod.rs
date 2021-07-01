// fetch_rates_mod

// fetch from floatrates.com

use serde_json::Value;
use unwrap::unwrap;

use crate::web_sys_mod as w;

pub async fn fetch_and_save() {
    let input_currency = crate::currdb_config_mod::get_input_currency().await;
    let v = fetch_and_serde_json(&input_currency).await;
    let json_map_string_value = unwrap!(v.as_object());
    crate::currdb_currency_mod::fill_currency_store(&input_currency, json_map_string_value).await;
    crate::currdb_config_mod::set_date_fetch(&w::get_now_date()).await;
}

pub async fn fetch_and_serde_json(input_currency: &str) -> Value {
    let url = format!("https://www.floatrates.com/daily/{}.json", input_currency.to_lowercase());
    let resp_body_text = w::fetch_response(&url).await;
    if resp_body_text.is_empty() {
        w::debug_write("error: resp_body_text is empty. Probably stupid CORS or CORB. ");
    }
    // there is 8 special characters I want to avoid
    let resp_body_text = resp_body_text
        .replace(r"\t", "")
        .replace(r"\u02bb", "ʻ")
        .replace(r"\u00e3", "ã")
        .replace(r"\u00f3", "ó")
        .replace(r"\u00e9", "é")
        .replace(r"\u00ed", "í");
    if resp_body_text.contains(r"\") {
        w::debug_write("error: resp_body_text contains backslash");
        //w::debug_write(&resp_body_text);
    }
    // Parse the string of data into serde_json::Value.
    let json_value: serde_json::Value = unwrap!(serde_json::from_str(&resp_body_text));
    // return
    json_value
}

pub async fn modify_rate() {
    let output_currency = crate::currdb_config_mod::get_output_currency().await;
    let rate = crate::currdb_currency_mod::get_rate(&output_currency).await;
    crate::currdb_config_mod::set_rate(&rate.to_string()).await;
}
