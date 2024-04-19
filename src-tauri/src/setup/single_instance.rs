use crate::state::get_config_dir;
use parking_lot::RwLock;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::process::id;
use tauri::{AppHandle, Config, Manager};
use tracing::{error, info, instrument};

const LOCK_FILE_NAME: &str = "ceobe-desktop.lck";
pub struct FileHandle(RwLock<Option<File>>);

impl From<File> for FileHandle {
    fn from(value: File) -> Self {
        Self(RwLock::new(Some(value)))
    }
}

pub struct SingleInstanceManager;

impl SingleInstanceManager {
    pub fn get_lck_file(&self, config: &Config) -> PathBuf {
        let dir = get_config_dir(config);
        dir.join(LOCK_FILE_NAME)
    }

    #[instrument(name = "CheckSingleInstance", skip_all)]
    pub fn check_instance(&self, handle: &Config) -> io::Result<FileHandle> {
        let dir = get_config_dir(handle);
        let file_path = dir.join(LOCK_FILE_NAME);
        let current_id = id();
        match File::create_new(file_path) {
            // file no exist, go
            Ok(mut file) => {
                file.write_all(format!("{current_id}").as_bytes())?;
                Ok(file.into())
            }
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
