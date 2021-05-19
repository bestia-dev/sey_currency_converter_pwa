// currdb_manual_rates_mod
#![allow(dead_code)]

// the store manual_rates in indexeddb database currdb

use crate::currdb_mod as db;
//use crate::web_sys_mod as w;
use serde::{Deserialize, Serialize};
//use serde_json::{Map, Value};
use strum::AsStaticRef;
//use unwrap::unwrap;
use wasm_bindgen::JsValue;

static THIS_OBJECT_STORE: &crate::currdb_mod::ObjectStores = &crate::currdb_mod::ObjectStores::Config;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value manual_rates
pub struct ValueStruct {
    pub input_currency: String,
    pub output_currency: String,
    pub date: String,
    pub rate: f64,
    pub description: String,
}

pub fn create_store_and_init(db: &db::Database, tx: &db::Transaction) {
    db.create_object_store(THIS_OBJECT_STORE.as_static());
    let currency_object_store = tx.get_object_store_versionchange(THIS_OBJECT_STORE.as_static());
    let jsvalue = crate::currdb_manual_rates_mod::to_jsvalue("USD".to_owned(), "USD".to_owned(), "2021-01-01".to_owned(), 1.0, "manual".to_owned());
    let key = &format!("{}-{}-{}", "USD", "USD", "manual");
    currency_object_store.put_jsvalue(key, &jsvalue);
}

pub fn to_jsvalue(input_currency: String, output_currency: String, date: String, rate: f64, description: String) -> JsValue {
    let value = ValueStruct {
        input_currency,
        output_currency,
        date,
        rate,
        description,
    };
    let jsvalue = serde_wasm_bindgen::to_value(&value).unwrap();
    // return
    jsvalue
}

pub fn from_jsvalue(jsvalue: JsValue) -> (String, String, String, f64, String) {
    let value_struct: ValueStruct = serde_wasm_bindgen::from_value(jsvalue).unwrap();
    // return
    (
        value_struct.input_currency,
        value_struct.output_currency,
        value_struct.date,
        value_struct.rate,
        value_struct.description,
    )
}

/// put in ObjectStore
pub async fn put_object_store_inside_transaction(
    object_store: &db::ObjectStoreInsideTransaction,
    object_store_key: &str,
    input_currency: String,
    output_currency: String,
    date: String,
    rate: f64,
    description: String,
) {
    let jsvalue = to_jsvalue(input_currency, output_currency, date, rate, description);
    object_store.put_jsvalue(object_store_key, &jsvalue);
}

pub async fn get_rate(object_store_key: &str) -> (String, f64, String) {
    let jsvalue = crate::currdb_mod::get_key_value(&THIS_OBJECT_STORE, object_store_key).await;
    let (_input_currency, _output_currency, date, rate, description) = from_jsvalue(jsvalue);
    // return
    (date, rate, description)
}
