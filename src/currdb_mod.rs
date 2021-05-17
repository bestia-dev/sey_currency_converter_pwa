// currdb_mod

// Specific code for work with the `currdb` database

//use std::fmt::format;

use crate::idbr_mod as idbr;
use crate::web_sys_mod as w;
use strum::AsStaticRef;
use strum_macros::*;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;

// Enums for databases and object stores to avoid mistyping the names
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

/// init_upgrade_currdb will create the database and call upgrade_currdb()
pub async fn init_upgrade_currdb() {
    // pointer for rust closure on the Heap
    let rust_closure_for_upgrade = Closure::wrap(Box::new(move |db: JsValue, old_version: JsValue, new_version: JsValue, transaction: JsValue| {
        upgrade_currdb(db, old_version, new_version, transaction);
    }) as Box<dyn Fn(JsValue, JsValue, JsValue, JsValue)>);
    idbr::Database::init_upgrade_db(Databases::Currdb.as_static(), 2, &rust_closure_for_upgrade).await;
}

/// pass this function to javascript as closure
pub fn upgrade_currdb(db: JsValue, old_version: JsValue, new_version: JsValue, transaction: JsValue) {
    let db = idbr::Database::from(db);
    let tx = idbr::Transaction::from(transaction);
    let old_version = unwrap!(old_version.as_f64()) as i32;
    let new_version = unwrap!(new_version.as_f64()) as i32;
    w::debug_write(&format!("upgrade_currdb from v{} to v{}", old_version, new_version));

    if old_version < 1 {
        upgrade_from_v00_to_v01(&db, &tx);
    }
    if old_version < 2 {
        upgrade_from_v01_to_v02(&db, &tx);
    }
    if old_version < 3 {
        upgrade_from_v02_to_v03(&db, &tx);
    }
    if old_version < 4 {
        upgrade_from_v03_to_v04(&db, &tx);
    }
}

fn upgrade_from_v00_to_v01(db: &idbr::Database, tx: &idbr::Transaction) {
    w::debug_write("upgrade_from_v00_to_v01");
    db.create_object_store(ObjectStores::Currency.as_static());
    let currency_object_store = tx.get_object_store_versionchange(ObjectStores::Currency.as_static());
    let jsvalue = crate::currdb_currency_mod::to_jsvalue("Euro".to_owned(), 1.0);
    currency_object_store.put_jsvalue("EUR", &jsvalue);
}

fn upgrade_from_v01_to_v02(db: &idbr::Database, tx: &idbr::Transaction) {
    w::debug_write("upgrade_from_v01_to_v02");
    db.create_object_store(ObjectStores::Config.as_static());
    let config_object_store = tx.get_object_store_versionchange(ObjectStores::Config.as_static());
    // this is a special put inside a transaction, that is inside version upgrade
    config_object_store.put("input_currency", "EUR");
    config_object_store.put("output_currency", "EUR");
    config_object_store.put("rate", "1");
    config_object_store.put("date_fetch", "none");
    config_object_store.put("input_number", "0");
}

fn upgrade_from_v02_to_v03(db: &idbr::Database, tx: &idbr::Transaction) {
    w::debug_write("upgrade_from_v02_to_v03");
    db.create_object_store(ObjectStores::ManualRates.as_static());
    let currency_object_store = tx.get_object_store_versionchange(ObjectStores::ManualRates.as_static());
    let jsvalue = crate::currdb_manual_rates_mod::to_jsvalue("USD".to_owned(), "USD".to_owned(), "2021-01-01".to_owned(), 1.0, "manual".to_owned());
    let key = &format!("{}-{}-{}", "USD", "USD", "manual");
    currency_object_store.put_jsvalue(key, &jsvalue);
}

fn upgrade_from_v03_to_v04(db: &idbr::Database, tx: &idbr::Transaction) {
    w::debug_write("upgrade_from_v03_to_v04");
    db.create_object_store(ObjectStores::CurrencyUsed.as_static());
    let store_currency_used = crate::currdb_currency_used_mod::StoreCurrdbCurrencyUsed::new_for_versionchange(tx);
    store_currency_used.put("EUR", "Euro");
}
