use crate::{request_client::cache_manage::CeobeCacheManager, state::get_cache_dir};
use http::Method;
use http_cache_reqwest::{Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{Client, IntoUrl, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use std::sync::OnceLock;
use tauri::AppHandle;
use tracing::{debug, info, instrument};

pub struct RequestClient {
    pub inner: ClientWithMiddleware,
}
mod cache_manage;

const HTTP_CACHE: &str = "http_cache";
static HTTP_CLIENT: OnceLock<RequestClient> = OnceLock::new();

impl RequestClient {
    #[instrument(name="RequestClient",skip_all)]
    pub fn get_this(app: AppHandle) -> &'static Self {
        HTTP_CLIENT.get_or_init(|| {
            let cache_location = get_cache_dir(app).join(HTTP_CACHE);
            info!(firstInit="RequestClient",cacheLocal = ?cache_location);
            let client = ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: CeobeCacheManager::new(cache_location),
                    options: HttpCacheOptions::default(),
                }))
                .build();
            Self { inner: client }
        })
    }

    #[instrument(name="RequestClient",skip_all)]
    pub fn request(&self, method: Method, url: impl IntoUrl) -> RequestBuilder {
        self.inner.request(method, url)
    }
    #[instrument(name="RequestClient",skip_all,err,ret(level = Level::TRACE))]
    pub async fn send(&self, request: Request) -> reqwest_middleware::Result<Response> {
        debug!(action="SendRequest",URL= %request.url(), method = %request.method());
        let resp = self.inner.execute(request).await?;
        Ok(resp)
    }
}
