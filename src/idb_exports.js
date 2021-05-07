"use strict";
// idb_exports.ts
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
exports.cursor_value = exports.cursor_key = exports.cursor_continue = exports.cursor = exports.db_put_key_value = exports.transaction_close = exports.transaction_open = exports.get_key_value = exports.put_key_value = exports.add_key_value = exports.use_db = exports.transaction_object_store_put_jsvalue = exports.transaction_object_store_put = exports.get_object_store_from_transaction_readwrite = exports.get_object_store_from_transaction_versionchange = exports.create_object_store = exports.db_get_jsvalue = exports.init_upgrade_db = exports.check_browser_capability = void 0;
// Typescript module that exports functions from 
// `idb` <https://github.com/jakearchibald/idb> (small wrapper for indexeddb)
// and prepares functions to be imported into rust.
// This import paths must be defined in tsconfig.json including the .d.ts types
// because the difference of folder structure and url paths.
var idb = require("/pwa_currency_converter/idb/index.js");
/// does the browser have indexedDB
function check_browser_capability() {
    if (!window.indexedDB) {
        console.log("NO, IndexedDB is not available.");
    }
    else {
        console.log("Yes, Indexeddb is available");
    }
}
exports.check_browser_capability = check_browser_capability;
/// Init db with upgrade (passed as function name), returns a promise
/// It must be the first command for indexeddb and it must have enough time to upgrade before later commands.
function init_upgrade_db(db_name, version, rust_closure_for_upgrade) {
    return __awaiter(this, void 0, void 0, function () {
        var db;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    console.log("init_upgrade_db");
                    return [4 /*yield*/, idb.openDB(db_name, version, {
                            upgrade: function (db, oldVersion, newVersion, transaction) {
                                rust_closure_for_upgrade(db, oldVersion, newVersion, transaction);
                            }
                        })];
                case 1:
                    db = _a.sent();
                    return [2 /*return*/, db];
            }
        });
    });
}
exports.init_upgrade_db = init_upgrade_db;
/// get key-value in a store 
function db_get_jsvalue(db, store, key) {
    return __awaiter(this, void 0, void 0, function () {
        var value;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, db.get(store, key)];
                case 1:
                    value = _a.sent();
                    return [2 /*return*/, value];
            }
        });
    });
}
exports.db_get_jsvalue = db_get_jsvalue;
/// create object store
function create_object_store(db, store_name) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            db.createObjectStore(store_name);
            return [2 /*return*/];
        });
    });
}
exports.create_object_store = create_object_store;
/// get object store from transaction versionchange
function get_object_store_from_transaction_versionchange(tx, store_name) {
    var object_store = tx.objectStore(store_name);
    return object_store;
}
exports.get_object_store_from_transaction_versionchange = get_object_store_from_transaction_versionchange;
/// get object store from transaction readwrite
function get_object_store_from_transaction_readwrite(tx, store_name) {
    var object_store = tx.objectStore(store_name);
    return object_store;
}
exports.get_object_store_from_transaction_readwrite = get_object_store_from_transaction_readwrite;
// put inside a transaction_object_store
function transaction_object_store_put(tx_ob_st, key, value) {
    tx_ob_st.put(value, key);
}
exports.transaction_object_store_put = transaction_object_store_put;
// put inside a transaction_object_store
function transaction_object_store_put_jsvalue(tx_ob_st, key, value) {
    tx_ob_st.put(value, key);
}
exports.transaction_object_store_put_jsvalue = transaction_object_store_put_jsvalue;
/// use_db returns a promise. 
/// I hope this is fast, because I call it often.
/// I hope it is reused and don't makes millions of unclosed db objects in memory.
function use_db(db_name) {
    return __awaiter(this, void 0, void 0, function () {
        var db;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, idb.openDB(db_name)];
                case 1:
                    db = _a.sent();
                    return [2 /*return*/, db];
            }
        });
    });
}
exports.use_db = use_db;
/// add key-value in a store
function add_key_value(db_name, store, key, value) {
    return __awaiter(this, void 0, void 0, function () {
        var db;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, use_db(db_name)];
                case 1:
                    db = _a.sent();
                    db.add(store, value, key);
                    return [2 /*return*/];
            }
        });
    });
}
exports.add_key_value = add_key_value;
/// put key-value in a store (upsert)
function put_key_value(db_name, store, key, value) {
    return __awaiter(this, void 0, void 0, function () {
        var db;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, use_db(db_name)];
                case 1:
                    db = _a.sent();
                    db.put(store, value, key);
                    return [2 /*return*/];
            }
        });
    });
}
exports.put_key_value = put_key_value;
/// get key-value in a store 
function get_key_value(db_name, store, key) {
    return __awaiter(this, void 0, void 0, function () {
        var db, value;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, use_db(db_name)];
                case 1:
                    db = _a.sent();
                    return [4 /*yield*/, db.get(store, key)];
                case 2:
                    value = _a.sent();
                    return [2 /*return*/, value];
            }
        });
    });
}
exports.get_key_value = get_key_value;
/// open transaction
function transaction_open(db) {
    // this transaction will block all stores in the database
    var tx = db.transaction(db.objectStoreNames, 'readwrite');
    return tx;
}
exports.transaction_open = transaction_open;
/// close transaction
function transaction_close(tx) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, tx.done];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
exports.transaction_close = transaction_close;
/// put key-value in a store (upsert)
function db_put_key_value(db, store, key, value) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            db.put(store, value, key);
            return [2 /*return*/];
        });
    });
}
exports.db_put_key_value = db_put_key_value;
function cursor(db, store_name) {
    return __awaiter(this, void 0, void 0, function () {
        var cursor;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, db.transaction(store_name).store.openCursor()];
                case 1:
                    cursor = _a.sent();
                    return [2 /*return*/, cursor];
            }
        });
    });
}
exports.cursor = cursor;
function cursor_continue(cursor) {
    return __awaiter(this, void 0, void 0, function () {
        var new_cursor_or_null;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, cursor["continue"]()];
                case 1:
                    new_cursor_or_null = _a.sent();
                    return [2 /*return*/, new_cursor_or_null];
            }
        });
    });
}
exports.cursor_continue = cursor_continue;
function cursor_key(cursor) {
    return cursor.key;
}
exports.cursor_key = cursor_key;
function cursor_value(cursor) {
    return cursor.value;
}
exports.cursor_value = cursor_value;
