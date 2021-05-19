// page_input_currency_mod.rs

//use std::ops::Index;

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
//use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use lazy_static::lazy_static;
use std::sync::Mutex;

// region: use macros

use crate::on_click;
use crate::on_keyup;
use crate::row_on_click;
// endregion: use macros

use crate::currdb_mod as db;
use crate::utils_mod as ut;
use crate::web_sys_mod as w;

lazy_static! {
    static ref TEMPLATE: Mutex<String> = Mutex::new(String::new());
}

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_input_currency() {
    // fetch page_unit.html and inject it
    let resp_body_text = w::fetch_response("pages/page_input_currency.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = unwrap!(ut::get_delimited_text(&resp_body_text, 0, "<body>", "</body>"));
    // get template
    let (template, _new_pos_cursor) = unwrap!(ut::get_delimited_text(&html_fragment, 0, "<!--use as template-->", "<!--end use as template-->",));
    //store it once in a global variable for later use
    TEMPLATE.lock().unwrap().truncate(0);
    TEMPLATE.lock().unwrap().push_str(&template);
    // remove template from html_fragment
    let html_fragment = ut::get_text_without_delimited_fragment(&html_fragment, 0, "<!--use as template-->", "<!--end use as template-->");
    // remove ignore as template
    let html_fragment = ut::get_text_without_delimited_fragment(&html_fragment, 0, "<!--ignore as template-->", "<!--end ignore as template-->");

    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);

    // region: binding - read from config

    // endregion: binding - read from config

    // region: read from indexed db row by row

    // repeat the template with data from indexed db in template inside div_list_layout

    let cursor = crate::currdb_mod::get_cursor(&crate::currdb_mod::ObjectStores::Currency).await;
    div_list_set(cursor, &template, "").await;

    // region: event handlers
    on_click!("div_back", div_back_on_click);
    on_click!("div_search_button", div_search_button_on_click);
}

async fn div_list_set(cursor: db::Cursor, template: &str, search_str: &str) {
    let mut html_list = String::with_capacity(1000);
    // I cannot implement the iterator trait because it is sync, but I need async
    // a simple loop will be enough
    let mut row_number_counter: usize = 0;
    loop {
        let key = cursor.get_key();
        let key: String = unwrap!(serde_wasm_bindgen::from_value(key));
        let value = cursor.get_value();
        let fields: crate::currdb_currency_mod::ValueStruct = unwrap!(serde_wasm_bindgen::from_value(value));
        let search_str = &search_str.to_ascii_lowercase();
        if search_str.is_empty() || key.to_ascii_lowercase().contains(search_str) || fields.name.to_ascii_lowercase().contains(search_str) {
            let template_with_data = template.replace("row_number_counter", &row_number_counter.to_string());

            let template_with_data = ut::replace_wt_placeholder(&template_with_data, "wt_unit", &key);
            let template_with_data = ut::replace_wt_placeholder(&template_with_data, "wt_name", &fields.name);

            html_list.push_str(&template_with_data);

            row_number_counter += 1;
        }
        if cursor.next().await.is_none() {
            break;
        }
    }
    // region: read from indexed db row by row
    w::set_inner_html("div_list_layout", &html_list);
    // handler for every row
    for i in 0..row_number_counter {
        row_on_click!("div_unit_", i, row_cell_on_click);
        row_on_click!("div_name_", i, row_cell_on_click);
    }
    // endregion: event handlers
}

/// go back to page_main
pub fn div_back_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_main_mod::page_main().await;
    });
}

/// unit is a field in the row of the list
pub fn row_cell_on_click(_element_prefix: &str, row_number: usize) {
    let element_id = format!("div_unit_{}", row_number);
    let input_currency = w::get_text(&element_id);
    spawn_local(async move {
        let input_currency = input_currency.clone();
        // w::debug_write(&format!("input input_currency: {}", &input_currency));
        let currency_name = crate::currdb_currency_mod::get_name(&input_currency).await;
        crate::currdb_currency_used_mod::put_single_item(&input_currency, &currency_name).await;
        //w::debug_write("added to currency_used");
        crate::currdb_config_mod::set_input_currency(&input_currency).await;
        crate::fetch_rates_mod::fetch_and_save().await;
        crate::fetch_rates_mod::modify_rate().await;
        crate::page_main_mod::page_main().await;
    });
}

pub fn div_search_button_on_click(_element_id: &str) {
    spawn_local(async {
        // put an input box instead of div_units_title and put focus there
        let html_fragment = r#"<input type="text" id="input_search" size="5" ></input>"#;
        w::set_inner_html("div_units_title", &html_fragment);
        w::set_inner_html("div_search_button", "×");
        on_keyup!("input_search", input_search_on_keyup);
        unwrap!(w::get_html_element_by_id("input_search").focus());
    });
}

pub fn input_search_on_keyup(_element_id: &str) {
    spawn_local(async {
        // show only filtered currencies
        // the original template in global variable
        let cursor = crate::currdb_mod::get_cursor(&crate::currdb_mod::ObjectStores::Currency).await;
        let template = TEMPLATE.lock().unwrap().clone();
        let search_str = w::get_input_element_value_string_by_id("input_search");
        //w::debug_write(&search_str);
        div_list_set(cursor, &template, &search_str).await;
    });
}
