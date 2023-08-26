use reqwest::{Request, Response};
use reqwest_middleware::Middleware;
use reqwest_middleware::Next;
use reqwest_middleware::Result;
use task_local_extensions::Extensions;
use tracing::debug;
use tracing::Instrument;
use tracing::Level;
pub struct CeobeLoggingRequest;

impl Middleware for CeobeLoggingRequest {
    fn handle<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        req: Request,
        extensions: &'life1 mut Extensions,
        next: Next<'life2>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Response>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        let span = tracing::span!(Level::DEBUG, "PerformRequest");
        Box::pin(
            async move {
                debug!(action = "RealSendingRequest",url = %req.url(),headers= ?req.headers());
                let resp = next.run(req, extensions).await?;
                debug!(action = "RealSendResponse", status = resp.status().as_u16());
                Ok(resp)
            }
            .instrument(span),
        )
    }
}
