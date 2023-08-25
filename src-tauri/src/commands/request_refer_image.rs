use std::sync::OnceLock;
use base64::Engine;
use tauri::api::http::{Client, ClientBuilder, HttpRequestBuilder, ResponseType};
use tauri::command;

#[command]
pub async fn request_refer_image(url: &str, refer: &str) -> tauri::Result<String> {
    static CLIENT: std::sync::OnceLock<Client> = OnceLock::new();
    let client =
        CLIENT.get_or_init(|| ClientBuilder::new().build().expect("create Client Failure"));

    let builder = HttpRequestBuilder::new("GET", url)?
        .header("Referer", refer)?
        .response_type(ResponseType::Binary);
    let resp = client.send(builder).await?;
    let payload = resp.bytes().await?;
    let payload = base64::engine::general_purpose::STANDARD.encode(payload.data);
    Ok(payload)
}
