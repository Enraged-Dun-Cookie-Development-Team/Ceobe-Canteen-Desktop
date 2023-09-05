use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;
use tracing::{debug, error, info, instrument, trace, warn};

#[derive(Serialize, Deserialize)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[command]
#[instrument(skip_all, name = "FontEndLog")]
pub fn front_logger(level: Level, local: String, payload: Value) {
    match level {
        Level::Trace => {
            trace!(logSource = "FontEnd", frontFile = local, payload = %payload);
        }
        Level::Debug => {
            debug!(logSource = "FontEnd", frontFile = local, payload = %payload);
        }
        Level::Info => {
            info!(logSource = "FontEnd", frontFile = local, payload = %payload);
        }
        Level::Warn => {
            warn!(logSource = "FontEnd", frontFile = local, payload = %payload);
        }
        Level::Error => {
            error!(logSource = "FontEnd", frontFile = local, payload = %payload);
        }
    }
}
