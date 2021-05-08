// currdb_manual_rates_mod
#![allow(dead_code)]

// the store manual_rates in indexeddb database currdb

use crate::currdb_mod::{Databases, ObjectStores};
use crate::idbr_mod as idbr;
//use crate::web_sys_mod as w;
use serde::{Deserialize, Serialize};
//use serde_json::{Map, Value};
use strum::AsStaticRef;
//use unwrap::unwrap;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value manual_rates
pub struct ValueStruct {
    pub input_currency: String,
    pub output_currency: String,
    pub date: String,
    pub rate: f64,
    pub description: String,
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
pub async fn put_inside_object_store(
    object_store: &idbr::ObjectStoreInsideTransaction,
    object_store_key: String,
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
    let db = idbr::Database::use_db(&Databases::Currdb.as_static()).await;
    let jsvalue = db.get_jsvalue(ObjectStores::ManualRates.as_static(), object_store_key).await;
    let (_input_currency, _output_currency, date, rate, description) = from_jsvalue(jsvalue);
    // return
    (date, rate, description)
}
