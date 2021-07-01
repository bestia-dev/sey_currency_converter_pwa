// page_input_currency_used_mod.rs

//use std::ops::Index;

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
//use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::currdb_mod as db;
use crate::on_click;
use crate::utils_mod as ut;
use crate::web_sys_mod as w;

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_input_currency_used() {
    // fetch page_unit.html and inject it
    let resp_body_text = w::fetch_response("pages/page_input_currency_used.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = unwrap!(ut::get_delimited_text(&resp_body_text, 0, "<body>", "</body>"));
    // get template
    let (template, _new_pos_cursor) = unwrap!(ut::get_delimited_text(&html_fragment, 0, "<!--use as template-->", "<!--end use as template-->",));
    // remove template from html_fragment
    let html_fragment = ut::get_text_without_delimited_fragment(&html_fragment, 0, "<!--use as template-->", "<!--end use as template-->");
    // remove ignore as template
    let html_fragment = ut::get_text_without_delimited_fragment(&html_fragment, 0, "<!--ignore as template-->", "<!--end ignore as template-->");

    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);

    let input_unit = &crate::currdb_config_mod::get_input_currency().await;

    // region: binding - read from config
    // endregion: binding - read from config

    // region: read from indexed db row by row
    let mut html_list = String::with_capacity(1000);
    // repeat the template with data from indexed db in template inside div_list_layout
    let cursor = crate::currdb_mod::get_cursor(&db::ObjectStores::CurrencyUsed).await;
    // I cannot implement the iterator trait because it is sync, but I need async
    // a simple loop will be enough
    let mut js_cmd = String::from("");
    loop {
        let key = cursor.get_key();
        let key: String = unwrap!(serde_wasm_bindgen::from_value(key));
        // instead of an integer, i will use the key. Iso_code is always 3 letters.
        let row_number_counter = key.clone();
        let value = cursor.get_value();
        let fields: crate::currdb_currency_used_mod::ValueStruct = unwrap!(serde_wasm_bindgen::from_value(value));

        let mut template_with_data = template.replace("row_number_counter", &row_number_counter);

        template_with_data = ut::replace_wt_placeholder(&template_with_data, "wt_unit", &key);
        template_with_data = ut::replace_wt_placeholder(&template_with_data, "wt_name", &fields.name);

        // the active currency cannot be removed and has another color
        if key.eq(input_unit) {
            template_with_data = template_with_data.replace("class_iso_code", "class_iso_code row_active_currency");
            template_with_data = template_with_data.replace("class_currency_name", "class_currency_name row_active_currency");
            template_with_data = template_with_data.replace("remove", "can't remove active");
            // no callback
            js_cmd.push_str(&format!(
                "window.mySwipe_{} = new Swipe(
                    document.getElementById('slider_{}')
                );\n",
                row_number_counter, row_number_counter
            ));
        } else {
            // callback to delete record in idb
            js_cmd.push_str(&format!(
                "window.mySwipe_{} = new Swipe(
                    document.getElementById('slider_{}'),
                    {{
                        callback: swipe_callback
                    }}
                );\n",
                row_number_counter, row_number_counter
            ));
        }

        html_list.push_str(&template_with_data);
        if cursor.next().await.is_none() {
            break;
        }
    }
    // region: read from indexed db row by row
    w::set_inner_html("div_list_layout", &html_list);
    // run javascript to activate slide
    unwrap!(js_sys::eval(&js_cmd));

    // region: event handlers
    on_click!("div_back", div_back_on_click);
    on_click!("page_input_currency_used_more", page_input_currency_used_more_on_click);
    // handler for every row
    let cursor = crate::currdb_mod::get_cursor(&db::ObjectStores::CurrencyUsed).await;
    loop {
        // Iso_code is always 3 letters.
        let key = cursor.get_key();
        let key: String = unwrap!(serde_wasm_bindgen::from_value(key));
        let element_id = format!("div_unit_{}", &key);
        on_click!(&element_id, row_cell_on_click);
        let element_id = format!("div_name_{}", &key);
        on_click!(&element_id, row_cell_on_click);

        if cursor.next().await.is_none() {
            break;
        }
    }
    // endregion: event handlers
}

/// go back to page_main
pub fn div_back_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_main_mod::page_main().await;
    });
}

/// opens the page_input_currency
fn page_input_currency_used_more_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_input_currency_mod::page_input_currency().await;
    });
}

/// unit is a field in the row of the list
pub fn row_cell_on_click(element_id: &str) {
    // Iso_code is always 3 letters.
    let input_currency = element_id[element_id.len() - 3..].to_string();
    w::debug_write(&input_currency);
    spawn_local(async move {
        let input_currency = input_currency.clone();
        //w::debug_write(&format!("input input_currency: {}", &input_currency));
        crate::currdb_config_mod::set_input_currency(&input_currency).await;
        crate::fetch_rates_mod::fetch_and_save().await;
        crate::fetch_rates_mod::modify_rate().await;
        crate::page_main_mod::page_main().await;
    });
}

#[allow(dead_code)]
#[wasm_bindgen]
pub async fn swipe_callback(_pos: JsValue, index: JsValue, _dir: JsValue) {
    let html_element: web_sys::HtmlElement = unwrap!(index.dyn_into::<web_sys::HtmlElement>());
    let parent_element = unwrap!(html_element.parent_element());
    let element_id = parent_element.id();
    // Iso_code is always 3 letters.
    let iso_code = element_id[&element_id.len() - 3..].to_string();
    w::debug_write(&iso_code);
    crate::currdb_currency_used_mod::delete_single_item(&iso_code).await;
    // refresh this page
    crate::page_input_currency_used_mod::page_input_currency_used().await;
}
