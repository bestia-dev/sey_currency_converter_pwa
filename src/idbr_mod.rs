// idbr_mod.rs
#![allow(dead_code)]

// Objects and method to work with indexeddb.
// Fully rust code and types. All the JsValue are wrapped.
// It uses the idb javascript library, idb_exports.ts and idbr_imports_mod.rs
// There are 4 objects: Database, Transaction, ObjectStoreInsideTransaction and Cursor
// The long way to do anything is: open database, open transaction, open object_store and then do something.
// It is really long when you want just to change one single value. To help with this there are a few 'shortcut functions'.

use std::usize;

use crate::idbr_imports_mod as idbjs;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub fn check_browser_capability() {
    idbjs::check_browser_capability();
}

// region: shortcut functions
// returns JsValue
pub async fn get_key_value(db_name: &str, store_name: &str, key: &str) -> JsValue {
    // return
    idbjs::get_key_value(db_name, store_name, key).await
}

// put JsValue
pub async fn put_key_value(db_name: &str, store_name: &str, key: &str, value: &JsValue) -> Result<(), JsValue> {
    // return
    idbjs::put_key_value(db_name, store_name, key, value).await
}

// delete key
pub async fn delete_key(db_name: &str, store_name: &str, key: &str) {
    // return
    idbjs::delete_key(db_name, store_name, key).await
}

pub async fn db_store_count_item(db_name: &str, store_name: &str) -> usize {
    // return
    let js_count = idbjs::db_store_count_item(db_name, store_name).await;
    let f64_count = unwrap!(js_count.as_f64());
    let usize_count = f64_count as usize;
    // return
    usize_count
}
// endregion: shortcut functions

// region: Database
pub struct Database {
    db: JsValue,
}
impl Database {
    /// init and upgrade
    pub async fn init_upgrade_db(db_name: &str, version: u32, rust_closure_for_upgrade: &Closure<dyn Fn(JsValue, JsValue, JsValue, JsValue)>) {
        idbjs::init_upgrade_db(db_name, version, rust_closure_for_upgrade).await.unwrap();
    }
    pub async fn use_db(db_name: &str) -> Self {
        let db = idbjs::use_db(&db_name).await;
        // return
        Database { db }
    }
    pub fn create_object_store(&self, store_name: &str) {
        idbjs::create_object_store(&self.db, store_name);
    }
    pub fn transaction_open(&self) -> Transaction {
        let tx = idbjs::transaction_open(&self.db);
        Transaction::from(tx)
    }
    pub async fn get_cursor(&self, store_name: &str) -> Cursor {
        let cursor = idbjs::cursor(&self.db, store_name).await;
        //return
        Cursor { cursor }
    }
    pub async fn get_jsvalue(&self, store_name: &str, key: &str) -> JsValue {
        // return
        idbjs::db_get_jsvalue(&self.db, store_name, key).await
    }
    pub async fn put_key_value(&self, store_name: &str, key: &str, value: &str) -> Result<(), JsValue> {
        idbjs::db_put_key_value(&self.db, store_name, key, value).await
    }
}
/// Database from JsValue
impl From<JsValue> for Database {
    /// Database from JsValue
    fn from(db: JsValue) -> Self {
        // return
        Database { db }
    }
}
// endregion: Database

// region: Transaction
pub struct Transaction {
    tx: JsValue,
}
impl Transaction {
    pub fn get_object_store_versionchange(&self, store_name: &str) -> ObjectStoreInsideTransaction {
        let tx_ob_st = idbjs::get_object_store_from_transaction_versionchange(&self.tx, store_name);
        let tx_ob_st = ObjectStoreInsideTransaction::from(tx_ob_st);
        // return
        tx_ob_st
    }
    pub fn get_object_store_readwrite(&self, store_name: &str) -> ObjectStoreInsideTransaction {
        let tx_ob_st = idbjs::get_object_store_from_transaction_readwrite(&self.tx, store_name);
        let tx_ob_st = ObjectStoreInsideTransaction::from(tx_ob_st);
        // return
        tx_ob_st
    }
    pub fn close(&self) {
        idbjs::transaction_close(&self.tx);
    }
}
/// Transaction from JsValue
impl From<JsValue> for Transaction {
    /// Transaction from JsValue
    fn from(tx: JsValue) -> Self {
        // return
        Transaction { tx }
    }
}
/// Transaction into JsValue
impl From<Transaction> for JsValue {
    /// Transaction into JsValue
    fn from(tx: Transaction) -> JsValue {
        tx.tx
    }
}
// endregion: Transaction

// region: ObjectStoreInsideTransaction
pub struct ObjectStoreInsideTransaction {
    tx_ob_st: JsValue,
}
impl ObjectStoreInsideTransaction {
    pub async fn get_jsvalue(&self, key: &str) -> JsValue {
        idbjs::transaction_object_store_get_jsvalue(&self.tx_ob_st, key).await
    }
    pub fn put_jsvalue(&self, key: &str, value: &JsValue) {
        idbjs::transaction_object_store_put_jsvalue(&self.tx_ob_st, key, value);
    }
    pub fn delete_key(&self, key: &str) {
        idbjs::transaction_object_store_delete_key(&self.tx_ob_st, key);
    }
}
/// ObjectStoreInsideTransaction from JsValue
impl From<JsValue> for ObjectStoreInsideTransaction {
    /// ObjectStoreInsideTransaction from JsValue
    fn from(tx_ob_st: JsValue) -> Self {
        // return
        ObjectStoreInsideTransaction { tx_ob_st }
    }
}
/// ObjectStoreInsideTransaction into JsValue
impl From<ObjectStoreInsideTransaction> for JsValue {
    /// ObjectStoreInsideTransaction into JsValue
    fn from(tx: ObjectStoreInsideTransaction) -> JsValue {
        tx.tx_ob_st.clone()
    }
}
// endregion: ObjectStoreInsideTransaction

// region: Cursor
pub struct Cursor {
    cursor: JsValue,
}
impl Cursor {
    // return None when is end of cursor
    pub async fn next(&self) -> Option<()> {
        // a new cursor ?
        let cursor = idbjs::cursor_continue(&self.cursor).await;
        // return
        if cursor.is_null() {
            return None;
        } else {
            return Some(());
        }
    }
    pub fn get_key(&self) -> JsValue {
        idbjs::cursor_key(&self.cursor)
    }
    pub fn get_value(&self) -> JsValue {
        idbjs::cursor_value(&self.cursor)
    }
}
/// Cursor from JsValue
impl From<JsValue> for Cursor {
    /// Cursor from JsValue
    fn from(cursor: JsValue) -> Self {
        // return
        Cursor { cursor }
    }
}
/// Cursor into JsValue
impl From<Cursor> for JsValue {
    /// Cursor into JsValue
    fn from(cr: Cursor) -> JsValue {
        cr.cursor.clone()
    }
}

// endregion: Cursor
