// currdb_config_mod
#![allow(dead_code)]

// Config is in store config_store_name in database currdb in indexeddb

use crate::currdb_mod as db;
//use js_sys::from_char_code1;
use serde::{Deserialize, Serialize};
use strum::AsStaticRef;
// use unwrap::unwrap;
use wasm_bindgen::JsValue;

static THIS_OBJECT_STORE: &db::ObjectStores = &db::ObjectStores::Config;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value currency
pub struct ValueStruct {
    pub value: String,
}

pub struct StoreCurrdbConfig {
    object_store_inside_transaction: db::ObjectStoreInsideTransaction,
}

impl StoreCurrdbConfig {
    pub fn new_for_versionchange(tx: &db::Transaction) -> Self {
        let object_store_inside_transaction = tx.get_object_store_versionchange(&THIS_OBJECT_STORE.as_static());
        // return
        StoreCurrdbConfig { object_store_inside_transaction }
    }
    pub fn new_for_readwrite(tx: &db::Transaction) -> Self {
        let object_store_inside_transaction = tx.get_object_store_readwrite(&THIS_OBJECT_STORE.as_static());
        // return
        StoreCurrdbConfig { object_store_inside_transaction }
    }
    pub fn put(&self, key: &str, value: &str) {
        let jsvalue = to_jsvalue(value);
        self.object_store_inside_transaction.put_jsvalue(key, &jsvalue);
    }
}

pub fn create_store_and_init(db: &db::Database, tx: &db::Transaction) {
    db.create_object_store(&THIS_OBJECT_STORE.as_static());
    let config_object_store = StoreCurrdbConfig::new_for_versionchange(tx);
    config_object_store.put("input_currency", "EUR");
    config_object_store.put("output_currency", "EUR");
    config_object_store.put("rate", "1");
    config_object_store.put("date_fetch", "none");
    config_object_store.put("input_number", "0");
}

fn to_jsvalue(value: &str) -> JsValue {
    let value = value.to_owned();
    let value = ValueStruct { value };
    let jsvalue = serde_wasm_bindgen::to_value(&value).unwrap();
    // return
    jsvalue
}

fn from_jsvalue(jsvalue: JsValue) -> String {
    let value_struct: ValueStruct = serde_wasm_bindgen::from_value(jsvalue).unwrap();
    // return
    value_struct.value
}

pub async fn set_input_currency(iso_code: &str) {
    db::put_key_value(&THIS_OBJECT_STORE, "input_currency", &to_jsvalue(iso_code)).await;
}

pub async fn get_input_currency() -> String {
    let jsvalue = db::get_key_value(&THIS_OBJECT_STORE, "input_currency").await;
    // return
    from_jsvalue(jsvalue)
}

pub async fn set_output_currency(iso_code: &str) {
    db::put_key_value(&THIS_OBJECT_STORE, "output_currency", &to_jsvalue(iso_code)).await;
}

pub async fn get_output_currency() -> String {
    let jsvalue = db::get_key_value(&THIS_OBJECT_STORE, "output_currency").await;
    // return
    from_jsvalue(jsvalue)
}

pub async fn set_rate(rate: &str) {
    db::put_key_value(&THIS_OBJECT_STORE, "rate", &to_jsvalue(rate)).await;
}

pub async fn get_rate() -> String {
    let jsvalue = db::get_key_value(&THIS_OBJECT_STORE, "rate").await;
    // return
    from_jsvalue(jsvalue)
}

pub async fn set_date_fetch(date_fetch: &str) {
    db::put_key_value(&THIS_OBJECT_STORE, "date_fetch", &to_jsvalue(date_fetch)).await;
}

pub async fn get_date_fetch() -> String {
    let jsvalue = db::get_key_value(&THIS_OBJECT_STORE, "date_fetch").await;
    // return
    from_jsvalue(jsvalue)
}

pub async fn set_input_number(input_number: &str) {
    db::put_key_value(&THIS_OBJECT_STORE, "input_number", &to_jsvalue(input_number)).await;
}

pub async fn get_input_number() -> String {
    let jsvalue = db::get_key_value(&THIS_OBJECT_STORE, "input_number").await;
    // return
    from_jsvalue(jsvalue)
}
