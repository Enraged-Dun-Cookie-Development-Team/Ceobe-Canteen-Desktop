use crate::state::get_config_dir;
use parking_lot::RwLock;
use pickledb::PickleDb;
use pickledb::PickleDbDumpPolicy::AutoDump;
use pickledb::SerializationMethod::Bin;
use serde_json::Value;

use once_cell::sync::OnceCell;

use tauri::AppHandle;
use tracing::{debug, info, instrument};

const DB_NAME: &str = "localstorage.db";
pub struct LocalStorage {
    db: RwLock<PickleDb>,
}

static STORAGE: OnceCell<LocalStorage> = OnceCell::new();

impl LocalStorage {
    #[instrument(name = "LocalStorage", skip_all)]
    pub fn get_this(app: AppHandle) -> &'static LocalStorage {
        STORAGE.get_or_init(move || {
            let path = get_config_dir(app).join(DB_NAME);
            info!(firstInit="Storage",path = ?path);
            let db = PickleDb::load(&path, AutoDump, Bin)
                .unwrap_or_else(|_| PickleDb::new(&path, AutoDump, Bin));

            Self {
                db: RwLock::new(db),
            }
        })
    }
    #[instrument(name = "LocalStorage", skip_all)]
    pub fn set(&self, key: &str, value: Value) -> Result<(), String> {
        debug!(action="SetData",key, value = ?value);
        self.db
            .write()
            .set(key, &value)
            .map_err(|err| format!("localStorageError: {err}"))
    }
    #[instrument(name = "LocalStorage", skip_all)]
    pub fn get(&self, key: &str) -> Value {
        let value = {
            #[cfg(not(debug_assertions))]
            {
                self.db.read().get(key).unwrap_or(Value::Null)
            }
            #[cfg(debug_assertions)]
            Value::Null
        };
        debug!(action="GetData",key, value = ?value);
        value
    }
}
