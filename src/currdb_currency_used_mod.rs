// currdb_currency_used_mod

// the store CurrencyUsed in indexeddb database currdb

use crate::currdb_mod::{Databases, ObjectStores};
use crate::idbr_mod as idbr;
//use crate::web_sys_mod as w;
use serde::{Deserialize, Serialize};
//use serde_json::{Map, Value};
use strum::AsStaticRef;
//use unwrap::unwrap;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value currency
pub struct ValueStruct {
    pub name: String,
}

pub struct StoreCurrdbCurrencyUsed {
    object_store_inside_transaction: idbr::ObjectStoreInsideTransaction,
}

impl StoreCurrdbCurrencyUsed {
    pub fn new_for_versionchange(tx: &idbr::Transaction) -> Self {
        let object_store_inside_transaction = tx.get_object_store_versionchange(ObjectStores::CurrencyUsed.as_static());
        // return
        StoreCurrdbCurrencyUsed { object_store_inside_transaction }
    }
    pub fn new_for_readwrite(tx: &idbr::Transaction) -> Self {
        let object_store_inside_transaction = tx.get_object_store_readwrite(ObjectStores::CurrencyUsed.as_static());
        // return
        StoreCurrdbCurrencyUsed { object_store_inside_transaction }
    }
    pub fn put(&self, iso_code: &str, name: &str) {
        let jsvalue = to_jsvalue(name);
        self.object_store_inside_transaction.put_jsvalue(iso_code, &jsvalue);
    }
}

fn to_jsvalue(name: &str) -> JsValue {
    let name = name.to_owned();
    let value = ValueStruct { name };
    let jsvalue = serde_wasm_bindgen::to_value(&value).unwrap();
    // return
    jsvalue
}

pub fn from_jsvalue(jsvalue: JsValue) -> String {
    let value_struct: ValueStruct = serde_wasm_bindgen::from_value(jsvalue).unwrap();
    // return
    value_struct.name
}

// shortcut to put only one item
pub async fn put_single_item(iso_code: &str, name: &str) {
    let db = idbr::Database::use_db(Databases::Currdb.as_static()).await;
    let tx = db.transaction_open();
    let store_currency_used = StoreCurrdbCurrencyUsed::new_for_readwrite(&tx);
    store_currency_used.put(iso_code, name);
}

// count items in store
pub async fn count_item() -> usize {
    idbr::db_store_count_item(Databases::Currdb.as_static(), ObjectStores::Currency.as_static()).await
}
