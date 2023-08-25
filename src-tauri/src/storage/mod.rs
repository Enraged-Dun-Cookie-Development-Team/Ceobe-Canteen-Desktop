use std::path::PathBuf;
use parking_lot::RwLock;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use pickledb::PickleDbDumpPolicy::AutoDump;
use pickledb::SerializationMethod::Bin;
use serde_json::Value;
use tauri::api::path::{app_config_dir, BaseDirectory};
use tauri::AppHandle;
use crate::state::get_config_dir;

const DB_NAME:&str = "localstorage.db";
pub struct LocalStorage{
    db:RwLock<PickleDb>
}

impl LocalStorage {
    pub fn new(app:AppHandle)->Self{
        let path = get_config_dir(app).join(DB_NAME);
        let db = PickleDb::load(&path,AutoDump,Bin)
            .unwrap_or_else(|_|PickleDb::new(&path,AutoDump,Bin));

        Self{
            db:RwLock::new(db)
        }

    }

    pub fn set(&self,key:&str,value:Value)->Result<(),String>{
         self.db.write().set(key,&value).map_err(|err|format!("localStorageError: {err}"))
    }

    pub fn get(&self,key:&str)->Value{
        self.db.read().get(key).unwrap_or(Value::Null)
    }
}