use arboard::{Clipboard, ImageData};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::{load_from_memory, EncodableLayout};
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use tauri::command;
use tracing::{info, instrument};

#[derive(Debug, thiserror::Error)]
pub enum CopyError {
    #[error("Base64 Decode Error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("Image Decode Error: {0}")]
    Image(#[from] image::ImageError),
    #[error("CLipBoard Error: {0}")]
    ClipBoard(#[from] arboard::Error),
}

impl Serialize for CopyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[command]
#[instrument(skip_all, err, name = "CopyImage")]
pub fn copy_image(image: String) -> Result<(), CopyError> {
    // data object base64 url -> base64
    let image = image.replace("data:image/jpeg;base64,", "");
    // base64 -> byte array
    let buffer = STANDARD.decode(image)?;
    // byte array -> image
    let image = load_from_memory(&buffer)?.to_rgba8();

    info!(
        action = "DecodeImage",
        image.width = image.width(),
        image.height = image.height()
    );

    let image = ImageData {
        width: image.width() as _,
        height: image.height() as _,
        bytes: Cow::from(image.as_bytes()),
    };
    let mut clipboard = Clipboard::new()?;
    clipboard.set_image(image)?;
    drop(clipboard);
    info!(action = "WriteImageToClipboard", result = "Done");
    Ok(())
}
