// currdb_mod

// Specific code for work with the `currdb` database

//use std::fmt::format;
pub use crate::idbr_mod::{Cursor, Database, ObjectStoreInsideTransaction, Transaction};

use crate::idbr_mod as idbr;
use crate::web_sys_mod as w;
use lazy_static::lazy_static;
use strum::AsStaticRef;
use strum_macros::*;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;

// region: Enums
// for databases and object stores to avoid mistyping the names
// in Rust I can use enum, but for javascript I will convert it to &'static str with .as_static()
#[derive(AsStaticStr)]
pub enum Databases {
    Currdb,
}

#[derive(AsStaticStr)]
pub enum ObjectStores {
    Currency,
    Config,
    ManualRates,
    CurrencyUsed,
}

// endregion: Enums

lazy_static! {
    static ref THIS_DB_NAME: &'static str = crate::currdb_mod::Databases::Currdb.as_static();
}

// region: upgrade
/// init_upgrade_currdb will create the database and call upgrade_currdb()
pub async fn init_upgrade_currdb() {
    let new_version = 4;
    // pointer for rust closure on the Heap
    let rust_closure_for_upgrade = Closure::wrap(Box::new(move |db: JsValue, old_version: JsValue, new_version: JsValue, transaction: JsValue| {
        upgrade_currdb(db, old_version, new_version, transaction);
    }) as Box<dyn Fn(JsValue, JsValue, JsValue, JsValue)>);
    idbr::Database::init_upgrade_db(&THIS_DB_NAME, new_version, &rust_closure_for_upgrade).await;
}

/// pass this function to javascript as closure
fn upgrade_currdb(db: JsValue, old_version: JsValue, new_version: JsValue, transaction: JsValue) {
    let db = idbr::Database::from(db);
    let tx = idbr::Transaction::from(transaction);
    let old_version = unwrap!(old_version.as_f64()) as i32;
    let new_version = unwrap!(new_version.as_f64()) as i32;
    w::debug_write(&format!("upgrade_currdb from v{} to v{}", old_version, new_version));

    if old_version < 1 && new_version >= 1 {
        w::debug_write("upgrade_from_v00_to_v01");
        crate::currdb_currency_mod::create_store_and_init(&db, &tx);
    }
    if old_version < 2 && new_version >= 2 {
        w::debug_write("upgrade_from_v01_to_v02");
        crate::currdb_config_mod::create_store_and_init(&db, &tx);
    }
    if old_version < 3 && new_version >= 3 {
        w::debug_write("upgrade_from_v02_to_v03");
        crate::currdb_manual_rates_mod::create_store_and_init(&db, &tx);
    }
    if old_version < 4 && new_version >= 4 {
        w::debug_write("upgrade_from_v03_to_v04");
        crate::currdb_currency_used_mod::create_store_and_init(&db, &tx);
    }
}
// endregion: upgrade

// region: shortcut functions
/// returns JsValue
pub async fn get_key_value(store: &ObjectStores, key: &str) -> JsValue {
    // return
    idbr::get_key_value(&THIS_DB_NAME, store.as_static(), key).await
}

/// put JsValue
pub async fn put_key_value(store: &ObjectStores, key: &str, value: &JsValue) {
    // return
    unwrap!(idbr::put_key_value(&THIS_DB_NAME, store.as_static(), key, value).await)
}

pub async fn get_cursor(store: &ObjectStores) -> idbr::Cursor {
    let db = idbr::Database::use_db(&THIS_DB_NAME).await;
    let cursor = db.get_cursor(store.as_static()).await;
    //return
    cursor
}
pub async fn transaction_open() -> idbr::Transaction {
    let db = idbr::Database::use_db(&THIS_DB_NAME).await;
    let tx = db.transaction_open();
    //return
    tx
}
pub async fn db_store_count_item(store: &ObjectStores) -> usize {
    let count = idbr::db_store_count_item(&THIS_DB_NAME, store.as_static()).await;
    //return
    count
}

// endregion: shortcut functions
