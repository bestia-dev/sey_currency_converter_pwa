// currdb_currency_mod

// the store Currency in indexeddb database currdb

use crate::currdb_mod as db;
use crate::web_sys_mod as w;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use strum::AsStaticRef;
use unwrap::unwrap;
use wasm_bindgen::JsValue;

static THIS_OBJECT_STORE: &crate::currdb_mod::ObjectStores = &crate::currdb_mod::ObjectStores::Currency;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value currency
pub struct ValueStruct {
    pub name: String,
    pub rate: f64,
}

pub fn create_store_and_init(db: &db::Database, tx: &db::Transaction) {
    db.create_object_store(&THIS_OBJECT_STORE.as_static());
    let currency_object_store = tx.get_object_store_versionchange(&THIS_OBJECT_STORE.as_static());
    let jsvalue = crate::currdb_currency_mod::to_jsvalue("Euro".to_owned(), 1.0);
    currency_object_store.put_jsvalue("EUR", &jsvalue);
}

pub fn to_jsvalue(name: String, rate: f64) -> JsValue {
    let value = ValueStruct { name, rate };
    let jsvalue = serde_wasm_bindgen::to_value(&value).unwrap();
    // return
    jsvalue
}

pub fn from_jsvalue(jsvalue: JsValue) -> (String, f64) {
    let value_struct: ValueStruct = serde_wasm_bindgen::from_value(jsvalue).unwrap();
    // return
    (value_struct.name, value_struct.rate)
}

/// put in ObjectStore
pub async fn put_object_store_inside_transaction(object_store: &db::ObjectStoreInsideTransaction, iso_code: &str, name: String, rate: f64) {
    let jsvalue = to_jsvalue(name, rate);
    object_store.put_jsvalue(iso_code, &jsvalue);
}

pub async fn fill_currency_store(input_currency: &str, json_map_string_value: &Map<String, Value>) {
    let tx = crate::currdb_mod::transaction_open().await;
    let store_inside_transaction = tx.get_object_store_readwrite(&THIS_OBJECT_STORE.as_static());
    for string_value in json_map_string_value {
        let iso_code = string_value.0.to_uppercase();
        let name = string_value.1["name"].to_string();
        // remove quotes in currency name
        let name = name.replace(r#"""#, "");
        let rate = unwrap!(string_value.1["rate"].as_f64());
        put_object_store_inside_transaction(&store_inside_transaction, &iso_code, name, rate).await;
    }
    // put also base currency
    let start = w::now_performance_millisecond();
    // this get and from_js is strangely very slow
    update_rate(&store_inside_transaction, input_currency, 1.0).await;
    w::debug_duration("update_rate", start);
    tx.close();
}

pub async fn get_rate(iso_code: &str) -> f64 {
    // let start = w::now_performance_millisecond();
    let jsvalue = crate::currdb_mod::get_key_value(&THIS_OBJECT_STORE, iso_code).await;
    let (_name, rate) = from_jsvalue(jsvalue);
    // w::debug_duration("get_rate", start);
    // return
    rate
}

pub async fn get_name(iso_code: &str) -> String {
    // let start = w::now_performance_millisecond();
    let jsvalue = crate::currdb_mod::get_key_value(&THIS_OBJECT_STORE, iso_code).await;
    let (name, _rate) = from_jsvalue(jsvalue);
    // w::debug_duration("get_name", start);
    // return
    name
}

// the part of get_jsvalue and from_jsvalue is strangely slow
pub async fn update_rate(store_inside_transaction: &db::ObjectStoreInsideTransaction, iso_code: &str, rate: f64) {
    // let start = w::now_performance_millisecond();
    let jsvalue = store_inside_transaction.get_jsvalue(iso_code).await;
    // w::debug_duration("get_jsvalue", start);
    // let start = w::now_performance_millisecond();
    let (name, _old_rate) = from_jsvalue(jsvalue);
    // w::debug_duration("from_jsvalue", start);
    put_object_store_inside_transaction(store_inside_transaction, iso_code, name, rate).await;
}

// count items in store
pub async fn count_item() -> usize {
    crate::currdb_mod::db_store_count_item(&THIS_OBJECT_STORE).await
}
