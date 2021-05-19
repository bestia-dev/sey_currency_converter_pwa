// idb_exports.ts
// Typescript module that exports functions from 
// `idb` <https://github.com/jakearchibald/idb> (small wrapper for indexeddb)
// and prepares functions to be imported into rust.
// This import paths must be defined in tsconfig.json including the .d.ts types
// because the difference of folder structure and url paths.
import * as idb from '/pwa_currency_converter/idb/index.js';
/// does the browser have indexedDB
export function check_browser_capability() {
    if (!window.indexedDB) {
        console.log("NO, IndexedDB is not available.");
    }
    else {
        console.log("Yes, Indexeddb is available");
    }
}
//#region shortcut functions
/// get key-value in a store 
export async function get_key_value(db_name, store, key) {
    let db = await use_db(db_name);
    const value = await db.get(store, key);
    return value;
}
/// put key-value in a store (upsert)
export async function put_key_value(db_name, store, key, value) {
    let db = await use_db(db_name);
    db.put(store, value, key);
}
export async function db_store_count_item(db_name, store_name) {
    let db = await use_db(db_name);
    const my_count = await db.count(store_name);
    return my_count;
}
//#endregion shortcut functions
//#region db
/// Init db with upgrade (passed as function name), returns a promise
/// It must be the first command for indexeddb and it must have enough time to upgrade before later commands.
export async function init_upgrade_db(db_name, version, rust_closure_for_upgrade) {
    console.log("init_upgrade_db");
    let db = await idb.openDB(db_name, version, {
        upgrade(db, oldVersion, newVersion, transaction) {
            rust_closure_for_upgrade(db, oldVersion, newVersion, transaction);
        },
    });
    return db;
}
/// use_db returns a promise. 
/// I hope this is fast, because I call it often.
/// I hope it is reused and don't makes millions of unclosed db objects in memory.
export async function use_db(db_name) {
    let db = await idb.openDB(db_name);
    return db;
}
/// create object store
export async function create_object_store(db, store_name) {
    db.createObjectStore(store_name);
}
/// open transaction
export function transaction_open(db) {
    // this transaction will block all stores in the database
    const tx = db.transaction(db.objectStoreNames, 'readwrite');
    return tx;
}
export async function cursor(db, store_name) {
    let cursor = await db.transaction(store_name).store.openCursor();
    return cursor;
}
/// get key-value in a store 
export async function db_get_jsvalue(db, store, key) {
    const value = await db.get(store, key);
    return value;
}
/// put key-value in a store (upsert)
export async function db_put_key_value(db, store, key, value) {
    db.put(store, value, key);
}
//#endregion db
//#region transaction
/// get object store from transaction versionchange
export function get_object_store_from_transaction_versionchange(tx, store_name) {
    let object_store = tx.objectStore(store_name);
    return object_store;
}
/// get object store from transaction readwrite
export function get_object_store_from_transaction_readwrite(tx, store_name) {
    let object_store = tx.objectStore(store_name);
    return object_store;
}
/// close transaction
export async function transaction_close(tx) {
    await tx.done;
}
//#endregion transaction
//#region object store
// get inside a transaction_object_store
export async function transaction_object_store_get_jsvalue(tx_ob_st, key) {
    const value = await tx_ob_st.get(key);
    return value;
}
// put inside a transaction_object_store
export function transaction_object_store_put_jsvalue(tx_ob_st, key, value) {
    tx_ob_st.put(value, key);
}
//#endregion object store
//#region cursor
export async function cursor_continue(cursor) {
    let new_cursor_or_null = await cursor.continue();
    return new_cursor_or_null;
}
export function cursor_key(cursor) {
    return cursor.key;
}
export function cursor_value(cursor) {
    return cursor.value;
}
//#endregion
//# sourceMappingURL=idb_exports.js.map