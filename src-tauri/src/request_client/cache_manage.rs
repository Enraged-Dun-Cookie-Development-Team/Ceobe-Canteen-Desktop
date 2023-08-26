use core::future::Future;
use core::pin::Pin;
use futures::{FutureExt, TryFutureExt};
use http_cache::Result;
use http_cache_reqwest::{CACacheManager, CacheManager, HttpResponse};
use http_cache_semantics::CachePolicy;
use std::path::PathBuf;
use tracing::Level;
use tracing::{debug, span};
use tracing::Instrument;
pub(super) struct CeobeCacheManager {
    inner: CACacheManager,
}

impl CeobeCacheManager {
    pub(super) fn new(path: PathBuf) -> Self {
        Self {
            inner: CACacheManager { path },
        }
    }
}

impl CacheManager for CeobeCacheManager {
    fn get<'life0, 'life1, 'async_trait>(
        &'life0 self,
        cache_key: &'life1 str,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Option<(HttpResponse, CachePolicy)>>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        let span = span!(Level::DEBUG,"CacheManger");
        self.inner
            .get(cache_key)
            .map_ok(move |ret| {
                debug!(action="tryHitCache",isHit = ret.is_some(), cacheKey = cache_key);
                ret
            })
            .instrument(span)
            .boxed()
    }

    fn put<'life0, 'async_trait>(
        &'life0 self,
        cache_key: String,
        res: HttpResponse,
        policy: CachePolicy,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<HttpResponse>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let span = span!(Level::DEBUG,"CacheManger");
        let _entry = span.enter();
        debug!(
            action="Caching",
            cacheKey = cache_key,
            respond.status=res.status,
        );
        drop(_entry);
        self.inner.put(cache_key, res, policy)
    }

    fn delete<'life0, 'life1, 'async_trait>(
        &'life0 self,
        cache_key: &'life1 str,
    ) -> Pin<
        Box<dyn Future<Output = Result<()>> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        let span = span!(Level::DEBUG,"CacheManger");
        let _entry = span.enter();
        debug!(action="DeleteCache",cacheKey=cache_key);
        drop(_entry);
        self.inner.delete(cache_key)
    }
}
