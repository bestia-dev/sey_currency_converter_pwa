// pwa_currency_converter lib.rs

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod currdb_config_mod;
mod currdb_currency_mod;
mod currdb_currency_used_mod;
mod currdb_manual_rates_mod;
mod currdb_mod;
mod fetch_rates_mod;
mod idbr_imports_mod;
mod idbr_mod;
mod page_input_currency_mod;
mod page_input_currency_used_mod;
mod page_main_mod;
mod page_manual_rates_mod;
mod page_modal_about_mod;
mod page_output_currency_mod;
mod utils_mod;
mod web_sys_mod;

use wasm_bindgen_futures::spawn_local;

use crate::web_sys_mod as w;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    w::debug_write(&format!("pwa_currency_converter v{}", env!("CARGO_PKG_VERSION")));
    // set the window initial size (only on desktop)
    unwrap!(w::window().resize_to(360, 640));

    crate::idbr_mod::check_browser_capability();

    //async block
    spawn_local(async {
        crate::currdb_mod::init_upgrade_currdb().await;
        crate::page_main_mod::page_main().await;
        let count_of_currency = crate::currdb_currency_mod::count_item().await;
        // if the database is empty (only one item), fetch exchange rates
        if count_of_currency <= 1 {
            crate::fetch_rates_mod::fetch_and_save().await;
        }
    });

    // return
    Ok(())
}
