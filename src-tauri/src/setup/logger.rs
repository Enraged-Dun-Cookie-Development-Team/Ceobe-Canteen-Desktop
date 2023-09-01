use chrono::Local;
use parking_lot::RwLock;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{stdout, IoSlice, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::level_filters::LevelFilter;
use tracing::{info, Subscriber};
use tracing_subscriber::fmt::{format, MakeWriter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};
pub fn init_logger(log_dir: PathBuf) -> io::Result<()> {
    tracing_subscriber::registry()
        .with(LevelLayer)
        .with(
            fmt::layer()
                .event_format(
                    format()
                        .pretty()
                        .with_thread_ids(true)
                        .with_ansi(true)
                        .with_level(true)
                        .with_target(true)
                        .with_line_number(true)
                        .with_thread_names(true),
                )
                .with_writer(stdout),
        )
        .with(
            fmt::layer()
                .event_format(
                    format()
                        .pretty()
                        .with_ansi(false)
                        .with_level(true)
                        .with_target(true)
                        .with_thread_names(true),
                )
                .with_writer(LogWriter::new(log_dir.clone())?),
        )
        .init();

    info!(step="StartUpLogger", path = ?log_dir);
    Ok(())
}

struct LevelLayer;

impl<S: Subscriber> Layer<S> for LevelLayer {
    fn max_level_hint(&self) -> Option<LevelFilter> {
        if cfg!(debug_assertions) {
            Some(LevelFilter::DEBUG)
        } else {
            Some(LevelFilter::INFO)
        }
    }
}

const LOG_FILE_SIZE: usize = 1024 * 256;

pub struct LogWriter {
    root: PathBuf,
    file: RwLock<File>,
    counter: AtomicUsize,
}

impl LogWriter {
    fn new(log_path: PathBuf) -> io::Result<Self> {
        if !log_path.exists() {
            std::fs::create_dir_all(&log_path)?;
        }
        let file = Self::open_log_file(&log_path)?;
        Ok(Self {
            root: log_path,
            file: RwLock::new(file),
            counter: AtomicUsize::new(0),
        })
    }
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = &'a Self;

    fn make_writer(&'a self) -> Self::Writer {
        self
    }
}

impl LogWriter {
    fn open_log_file(path: &PathBuf) -> io::Result<File> {
        let now = Local::now();
        let format = now.format("%Y-%m-%dT%H-%M-%S");
        let filename = format!("log-{format}.log");
        let filepath = path.join(filename);
        let fs = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filepath)?;
        Ok(fs)
    }
    fn check_file_fill(&self) -> io::Result<()> {
        let mut lock_file = self.file.write();
        let current_count = self.counter.load(Ordering::Acquire);
        if current_count >= LOG_FILE_SIZE {
            let fs = Self::open_log_file(&self.root)?;
            lock_file.flush()?;
            *lock_file = fs;

            self.counter.store(0, Ordering::Release);
        }
        drop(lock_file);
        Ok(())
    }
}

impl Write for &LogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let count = (&*self.file.read()).write(buf)?;
        self.counter.fetch_add(count, Ordering::AcqRel);
        self.check_file_fill()?;
        Ok(count)
    }
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let c = (self.file.read().deref()).write_vectored(bufs)?;
        self.counter.fetch_add(c, Ordering::AcqRel);
        self.check_file_fill()?;
        Ok(c)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&*(self.file.read())).flush()?;
        self.counter.store(0, Ordering::Release);
        Ok(())
    }
}
