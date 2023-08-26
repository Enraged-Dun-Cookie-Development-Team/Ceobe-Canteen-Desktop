use crate::storage::LocalStorage;
use serde_json::Value;
use tauri::{command, AppHandle};
use tracing::instrument;

#[command]
#[instrument(name = "GetStorage", skip(app))]
pub fn get_item(key: String, app: AppHandle) -> Value {
    LocalStorage::get_this(app).get(&key)
}

#[command]
#[instrument(name = "SetStorage", skip(app), err)]
pub fn set_item(key: String, value: Value, app: AppHandle) -> Result<(), String> {
    LocalStorage::get_this(app).set(&key, value)
}
