use crate::storage::LocalStorage;
use serde_json::Value;
use tauri::{command, AppHandle};

#[command]
pub fn get_item(key: String, app: AppHandle) -> Value {
    LocalStorage::get_this(app).get(&key)
}

#[command]
pub fn set_item(key: String, value: Value, app: AppHandle) -> Result<(), String> {
    LocalStorage::get_this(app).set(&key, value)
}
