mod request_refer_image;
mod auto_launch;
mod preview_page;
mod clipboard;
mod local_storage;

pub use request_refer_image::request_refer_image;
pub use self::auto_launch::{auto_launch_setting,set_auto_launch};
pub use preview_page::read_detail;
pub use clipboard::copy_image;
pub use local_storage::{get_item,set_item};