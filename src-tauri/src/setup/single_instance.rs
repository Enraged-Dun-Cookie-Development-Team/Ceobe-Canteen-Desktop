use crate::state::get_config_dir;
use parking_lot::RwLock;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use tauri::{App, AppHandle, Config, Manager};
use tracing::{error, info, instrument};
use tracing_subscriber::reload::Handle;

const LOCK_FILE_NAME: &'static str = "ceobe-desktop.lck";
pub struct FileHandle(RwLock<Option<File>>);

impl From<File> for FileHandle {
    fn from(value: File) -> Self {
        Self(RwLock::new(Some(value)))
    }
}

pub struct SingleInstanceManager;

impl SingleInstanceManager {
    #[instrument(name = "CheckSingleInstance", skip_all)]
    pub fn check_instance(&self, handle: &Config) -> io::Result<FileHandle> {
        let dir = get_config_dir(handle);
        let file_path = dir.join(LOCK_FILE_NAME);
        match File::create_new(file_path) {
            // file no exist, go
            Ok(file) => Ok(file.into()),
            Err(err) => {
                // file not exist, can be startup
                if err.kind() == ErrorKind::AlreadyExists {
                    error!(IsSingleInstance = false);
                }
                Err(err)
            }
        }
    }
    #[instrument(name = "LeaveSingleInstance", skip_all)]
    pub fn quit_instance(&self, handle: &Config, fd: &FileHandle) -> io::Result<()> {
        let dir = get_config_dir(handle);
        let file_path = dir.join(LOCK_FILE_NAME);
        info!(closeFd = true);
        if let Some(fd) = fd.0.write().take() {
            fd.sync_all()?;
            drop(fd);
        }

        info!(removeFile = true);
        match std::fs::remove_file(file_path) {
            Ok(()) => Ok(()),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(err)
                }
            }
        }
    }
}

pub trait BeforeExit {
    fn graceful_exit(&self, code: i32);
}

impl BeforeExit for AppHandle {
    fn graceful_exit(&self, code: i32) {
        let state = self.state::<FileHandle>().inner();
        SingleInstanceManager
            .quit_instance(&self.config(), state)
            .expect("Failure remove Lock File");
        self.exit(code)
    }
}
