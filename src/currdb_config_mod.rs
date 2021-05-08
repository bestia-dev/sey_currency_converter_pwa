// currdb_config_mod

// Config is in store config_store_name in database currdb in indexeddb

use crate::currdb_mod::{Databases, ObjectStores};
use crate::idbr_mod as idb;
use strum::AsStaticRef;

async fn currdb() -> idb::Database {
    idb::Database::use_db(Databases::Currdb.as_static()).await
}

pub async fn set_input_currency(iso_code: &str) {
    currdb().await.put_key_value(&ObjectStores::Config.as_static(), "input_currency", iso_code).await.unwrap();
}

pub async fn get_input_currency() -> String {
    let value = idb::get_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "input_currency").await;
    // return
    value
}

pub async fn set_output_currency(iso_code: &str) {
    idb::put_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "output_currency", iso_code)
        .await
        .unwrap();
}

pub async fn get_output_currency() -> String {
    let value = idb::get_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "output_currency").await;
    // return
    value
}

pub async fn set_rate(rate: &str) {
    idb::put_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "rate", rate).await.unwrap();
}

pub async fn get_rate() -> String {
    let value = idb::get_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "rate").await;
    // return
    value
}

pub async fn set_date_fetch(date_fetch: &str) {
    idb::put_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "date_fetch", date_fetch)
        .await
        .unwrap();
}

pub async fn get_date_fetch() -> String {
    let value = idb::get_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "date_fetch").await;
    // return
    value
}

pub async fn set_input_number(input_number: &str) {
    idb::put_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "input_number", input_number)
        .await
        .unwrap();
}

pub async fn get_input_number() -> String {
    let value = idb::get_key_value(Databases::Currdb.as_static(), &ObjectStores::Config.as_static(), "input_number").await;
    // return
    value
}
