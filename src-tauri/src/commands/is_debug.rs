use tauri::command;

#[command]
pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}
