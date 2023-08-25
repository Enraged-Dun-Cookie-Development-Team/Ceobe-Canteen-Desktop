
use base64::Engine;

use tauri::{AppHandle, command};
use thiserror::Error;
use crate::request_client::RequestClient;

#[derive(Debug,Error)]
pub enum RefImgError{
    #[error("Request Error: {0}")]
    Reqwest(#[from]reqwest::Error),
    #[error("Sending Request Error: {0}")]
    Middleware(#[from]reqwest_middleware::Error)
}

#[command]
pub async fn request_refer_image(url: &str, refer: &str,app:AppHandle) -> Result<String,RefImgError> {
    let client =RequestClient::get_this(app);

    let builder = client.inner.get(url)
        .header("Referer",refer)
        .build()?;

    let resp = client.inner.execute(builder).await?;
    let payload = resp.bytes().await?;

    let payload = base64::engine::general_purpose::STANDARD.encode(payload);
    Ok(payload)
}
