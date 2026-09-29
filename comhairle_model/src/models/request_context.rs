//! Per-request client details that get persisted.
//!
//! These are stamped onto the request extensions by
//! [`crate::middleware::request_logging::log_requests`] and then written to the
//! database (refresh token audit rows, signup metadata). They are plain data with
//! no web-layer dependencies, so they belong with the models that store them
//! rather than with the middleware that populates them.

/// The resolved client IP for the current request. Handlers extract it with
/// `Extension<ClientIp>`.
#[derive(Debug, Clone)]
pub struct ClientIp(pub String);

/// The client browser signature (`User-Agent`) for the current request. `None`
/// when the header is absent or not valid UTF-8.
#[derive(Debug, Clone)]
pub struct ClientUserAgent(pub Option<String>);
