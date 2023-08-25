use serde_json::Value;
use tauri::{command, State};
use crate::storage::LocalStorage;

#[command]
pub fn get_item(key:String, state:State<LocalStorage>)->Value{
    state.get(&key)
}

#[command]
pub fn set_item(key:String,value:Value,state:State<LocalStorage>)->Result<(),String>{
    state.set(&key,value)
}