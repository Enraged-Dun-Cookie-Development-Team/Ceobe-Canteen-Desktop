use std::sync::OnceLock;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{Client};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use tauri::AppHandle;
use crate::state::{get_cache_dir};

pub struct RequestClient {
    pub inner: ClientWithMiddleware,
}

const HTTP_CACHE: &str = "http_cache";
static HTTP_CLIENT: OnceLock<RequestClient> = OnceLock::new();

impl RequestClient {
    pub fn get_this(app: AppHandle) -> &'static Self {
        HTTP_CLIENT.get_or_init(|| {
            let cache_location = get_cache_dir(app).join(HTTP_CACHE);

            let client = ClientBuilder::new(Client::new()).with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager { path: cache_location },
                options: HttpCacheOptions::default(),
            }))

                .build();
            Self { inner: client }
        })
    }
}