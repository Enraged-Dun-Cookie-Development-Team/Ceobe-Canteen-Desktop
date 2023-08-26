use std::io::stdout;
use tracing::Level;
use tracing_subscriber::fmt;

pub fn init_logger() {
    fmt()
        .with_max_level(Level::DEBUG)
        .pretty()
        .with_thread_ids(true)
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_line_number(true)
        .with_writer(stdout)
        .init()
}
