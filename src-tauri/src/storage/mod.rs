use crate::state::get_config_dir;
use parking_lot::RwLock;
use pickledb::PickleDbDumpPolicy::AutoDump;
use pickledb::SerializationMethod::Bin;
use pickledb::{PickleDb};
use serde_json::Value;

use once_cell::sync::OnceCell;

use tauri::AppHandle;

const DB_NAME: &str = "localstorage.db";
pub struct LocalStorage {
    db: RwLock<PickleDb>,
}

static STORAGE: OnceCell<LocalStorage> = OnceCell::new();

impl LocalStorage {
    pub fn get_this(app: AppHandle) -> &'static LocalStorage {
        STORAGE.get_or_init(move || {
            let path = get_config_dir(app).join(DB_NAME);
            println!("Path: {path:?}");
            let db = PickleDb::load(&path, AutoDump, Bin)
                .unwrap_or_else(|_| PickleDb::new(&path, AutoDump, Bin));

            Self {
                db: RwLock::new(db),
            }
        })
    }

    pub fn set(&self, key: &str, value: Value) -> Result<(), String> {
        self.db
            .write()
            .set(key, &value)
            .map_err(|err| format!("localStorageError: {err}"))
    }

    pub fn get(&self, key: &str) -> Value {
        self.db.read().get(key).unwrap_or(Value::Null)
    }
}
