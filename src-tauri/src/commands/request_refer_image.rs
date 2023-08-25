use base64::Engine;
use serde::{Serialize, Serializer};

use crate::request_client::RequestClient;
use tauri::{command, AppHandle};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefImgError {
    #[error("Request Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Sending Request Error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
}

impl Serialize for RefImgError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[command]
pub async fn request_refer_image(
    url: &str,
    refer: &str,
    app: AppHandle,
) -> Result<String, RefImgError> {
    let client = RequestClient::get_this(app);

    let builder = client.inner.get(url).header("Referer", refer).build()?;

    let resp = client.inner.execute(builder).await?;
    let payload = resp.bytes().await?;

    let payload = base64::engine::general_purpose::STANDARD.encode(payload);
    Ok(payload)
}
