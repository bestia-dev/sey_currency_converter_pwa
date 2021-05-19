// currdb_currency_used_mod
#![allow(dead_code)]

// the store CurrencyUsed in indexeddb database currdb

use crate::currdb_mod as db;
//use crate::web_sys_mod as w;
use serde::{Deserialize, Serialize};
//use serde_json::{Map, Value};
use strum::AsStaticRef;
//use unwrap::unwrap;
use wasm_bindgen::JsValue;

static THIS_OBJECT_STORE: &crate::currdb_mod::ObjectStores = &crate::currdb_mod::ObjectStores::CurrencyUsed;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value currency
pub struct ValueStruct {
    pub name: String,
}

pub struct StoreCurrdbCurrencyUsed {
    object_store_inside_transaction: db::ObjectStoreInsideTransaction,
}

impl StoreCurrdbCurrencyUsed {
    pub fn new_for_versionchange(tx: &db::Transaction) -> Self {
        let object_store_inside_transaction = tx.get_object_store_versionchange(&THIS_OBJECT_STORE.as_static());
        // return
        StoreCurrdbCurrencyUsed { object_store_inside_transaction }
    }
    pub fn new_for_readwrite(tx: &db::Transaction) -> Self {
        let object_store_inside_transaction = tx.get_object_store_readwrite(&THIS_OBJECT_STORE.as_static());
        // return
        StoreCurrdbCurrencyUsed { object_store_inside_transaction }
    }
    pub fn put(&self, iso_code: &str, name: &str) {
        let jsvalue = to_jsvalue(name);
        self.object_store_inside_transaction.put_jsvalue(iso_code, &jsvalue);
    }
}

pub fn create_store_and_init(db: &db::Database, tx: &db::Transaction) {
    db.create_object_store(&THIS_OBJECT_STORE.as_static());
    let store_currency_used = crate::currdb_currency_used_mod::StoreCurrdbCurrencyUsed::new_for_versionchange(tx);
    store_currency_used.put("EUR", "Euro");
}

fn to_jsvalue(name: &str) -> JsValue {
    let name = name.to_owned();
    let value = ValueStruct { name };
    let jsvalue = serde_wasm_bindgen::to_value(&value).unwrap();
    // return
    jsvalue
}

fn from_jsvalue(jsvalue: JsValue) -> String {
    let value_struct: ValueStruct = serde_wasm_bindgen::from_value(jsvalue).unwrap();
    // return
    value_struct.name
}

// shortcut function to put only one item
pub async fn put_single_item(iso_code: &str, name: &str) {
    let tx = crate::currdb_mod::transaction_open().await;
    let store_currency_used = StoreCurrdbCurrencyUsed::new_for_readwrite(&tx);
    store_currency_used.put(iso_code, name);
    tx.close()
}

// count items in store
pub async fn count_item() -> usize {
    crate::currdb_mod::db_store_count_item(THIS_OBJECT_STORE).await
}
