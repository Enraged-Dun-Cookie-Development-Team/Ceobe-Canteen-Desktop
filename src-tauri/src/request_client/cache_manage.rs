use futures::{FutureExt, TryFutureExt};
use http_cache::Result;
use http_cache_reqwest::{CACacheManager, CacheManager, HttpResponse};
use http_cache_semantics::CachePolicy;
use std::path::PathBuf;

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
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Option<(HttpResponse, CachePolicy)>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.inner
            .get(cache_key)
            .map_ok(move |ret| {
                println!("{}: `{cache_key}`", if ret.is_some(){"HIT"} else{"MISS"});
                ret
            })
            .boxed()
    }

    fn put<'life0, 'async_trait>(
        &'life0 self,
        cache_key: String,
        res: HttpResponse,
        policy: CachePolicy,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<HttpResponse>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        println!(
            "caching: {}: payload vaildation: {}, code:{}",
            cache_key,
            res.must_revalidate(),
            res.status
        );
        self.inner.put(cache_key, res, policy)
    }

    fn delete<'life0, 'life1, 'async_trait>(
        &'life0 self,
        cache_key: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        println!("Delete Cache Key `{}`", cache_key);
        self.inner.delete(cache_key)
    }
}
