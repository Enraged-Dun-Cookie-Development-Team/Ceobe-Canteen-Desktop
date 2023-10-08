use crate::request_client::{RequestClient, response_type::ResponseType};
use http::header;
use http::method::InvalidMethod;
use reqwest::{Method, Response, Url};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::FromUtf8Error;
use std::time::Duration;
use tauri::{command, AppHandle};
use thiserror::Error;
use tracing::{debug, instrument};

#[derive(Clone, Debug, Deserialize)]
pub struct RequestOptions {
    method: String,
    url: String,
    headers: Option<HeaderMap>,
    query: Option<HashMap<String, String>>,
    body: Option<Value>,
    timeout: Option<u64>,
    response_type: Option<ResponseType>,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Request Client Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Request Error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
    #[error("Unknown Request Method: {0}")]
    Method(#[from] InvalidMethod),
    #[error("Invalid Url : {0}")]
    Url(#[from] url::ParseError),
    #[error("NotUtf8Encode: {0}")]
    StringNotUtf8(#[from] FromUtf8Error),
    #[error("SerdeJsonError: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl Serialize for RequestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[command(async)]
#[instrument(err,skip(app,options),name="SendRequest",fields(url=options.url,method=options.method))]
pub async fn send_request(
    options: RequestOptions,
    app: AppHandle,
) -> Result<ResponseData, RequestError> {
    let client = RequestClient::get_this(app);
    let method = Method::from_str(&options.method)?;
    let url = Url::from_str(&options.url)?;
    let mut request = client.request(method, url.clone());

    if let Some(map) = options.headers {
        request = request.headers(map.0)
    }
    if let Some(query) = options.query {
        request = request.query(&query);
    }
    if let Some(body) = options.body {
        request = request.json(&body);
    }
    if let Some(timeout) = options.timeout {
        request = request.timeout(Duration::from_millis(timeout))
    }
    let request = request.build()?;

    debug!(state = "PrepareDone", url = options.url);
    let resp = client.send(request).await?;

    let response = response_to_data(
        url,
        resp,
        options.response_type.unwrap_or(ResponseType::Json),
    )
    .await?;

    Ok(response)
}

/// A set of HTTP headers.
#[derive(Debug, Default, Clone)]
pub struct HeaderMap(header::HeaderMap);

impl<'de> Deserialize<'de> for HeaderMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = HashMap::<String, String>::deserialize(deserializer)?;
        let mut headers = header::HeaderMap::default();
        for (key, value) in map {
            if let (Ok(key), Ok(value)) = (
                header::HeaderName::from_bytes(key.as_bytes()),
                header::HeaderValue::from_str(&value),
            ) {
                headers.insert(key, value);
            } else {
                return Err(serde::de::Error::custom(format!(
                    "invalid header `{key}` `{value}`"
                )));
            }
        }
        Ok(Self(headers))
    }
}

async fn response_to_data(
    url: Url,
    resp: Response,
    ty: ResponseType,
) -> Result<ResponseData, RequestError> {
    let mut headers = HashMap::new();
    let mut raw_headers = HashMap::new();
    for (name, value) in resp.headers() {
        headers.insert(
            name.as_str().to_string(),
            String::from_utf8(value.as_bytes().to_vec())?,
        );
        raw_headers.insert(
            name.as_str().to_string(),
            resp.headers()
                .get_all(name)
                .into_iter()
                .map(|v| String::from_utf8(v.as_bytes().to_vec()).map_err(Into::into))
                .collect::<Result<Vec<String>, RequestError>>()?,
        );
    }
    let status = resp.status().as_u16();
    let data = match ty {
        ResponseType::Json => resp.json().await?,
        ResponseType::Text => Value::String(resp.text().await?),
        ResponseType::Binary => serde_json::from_slice(&resp.bytes().await?)?,
    };

    Ok(ResponseData {
        url,
        status,
        headers,
        raw_headers,
        data,
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseData {
    /// Response URL. Useful if it followed redirects.
    pub url: Url,
    /// Response status code.
    pub status: u16,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Response raw headers.
    pub raw_headers: HashMap<String, Vec<String>>,
    /// Response data.
    pub data: Value,
}
