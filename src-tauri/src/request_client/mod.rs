use crate::{
    request_client::{
        cache_manage::CeobeCacheManager, logging_request_middleware::CeobeLoggingRequest,
    },
    state::get_cache_dir,
};
use http::Method;
use http_cache_reqwest::{Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{Client, IntoUrl, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::sync::OnceLock;
use std::time::Duration;
use tauri::AppHandle;
use tracing::{info, instrument, Level};
pub mod response_type;
pub struct RequestClient {
    pub inner: ClientWithMiddleware,
}
mod cache_manage;
mod logging_request_middleware;
const HTTP_CACHE: &str = "http_cache";
static HTTP_CLIENT: OnceLock<RequestClient> = OnceLock::new();

impl RequestClient {
    #[instrument(name = "RequestClient", skip_all)]
    pub fn get_this(app: AppHandle) -> &'static Self {
        HTTP_CLIENT.get_or_init(|| {
            let cache_location = get_cache_dir(app).join(HTTP_CACHE);
            let retry_policy = ExponentialBackoff::builder()
                .backoff_exponent(2)
                .retry_bounds(Duration::from_millis(500), Duration::from_secs(16))
                .build_with_total_retry_duration(Duration::from_secs(120));
            info!(firstInit="RequestClient",cacheLocal = ?cache_location);
            let client = ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: CeobeCacheManager::new(cache_location),
                    options: HttpCacheOptions::default(),
                }))
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .with(CeobeLoggingRequest)
                .build();
            Self { inner: client }
        })
    }

    #[instrument(name = "RequestClient", skip_all)]
    pub fn request(&self, method: Method, url: impl IntoUrl) -> RequestBuilder {
        self.inner.request(method, url)
    }
    #[instrument(name="RequestClient",skip_all,err,ret(level = Level::TRACE))]
    pub async fn send(&self, request: Request) -> reqwest_middleware::Result<Response> {
        info!(
            action = "SendRequest",
            Url = %request.url(),
            method = %request.method()
        );
        let resp = self.inner.execute(request).await?;
        info!(
            action = "RequestRespond",
            Url = %resp.url(),
            resp.status = %resp.status(),
            resp.version = ?resp.version()
        );
        Ok(resp)
    }
}
