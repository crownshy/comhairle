use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use tracing::Instrument;

use crate::ComhairleState;
use crate::routes::auth::{AUTH_KEY, user_id_from_session_token};

/// The resolved client IP for the current request, stamped into the request
/// extensions by [`log_requests`] so downstream handlers can read it without
/// re-deriving it. Handlers extract it with `Extension<ClientIp>`.
#[derive(Debug, Clone)]
pub struct ClientIp(pub String);

/// Middleware that opens an `api_request` span carrying the client IP, method,
/// path, and the logged-in user's id (when a valid session cookie is present).
/// Running the handler inside the span stamps those fields onto every
/// request-scoped log event, giving per-IP / per-user request tracing.
///
/// It also inserts a [`ClientIp`] into the request extensions so handlers (e.g.
/// signup) can persist the client IP without re-parsing proxy headers.
pub async fn log_requests(
    State(state): State<Arc<ComhairleState>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    // `ConnectInfo` is placed in the request extensions by
    // `into_make_service_with_connect_info`; it is absent in tests that drive
    // the router directly (e.g. via `oneshot`), so treat it as optional.
    let addr = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0);
    let ip = client_ip(req.headers(), addr);
    req.extensions_mut().insert(ClientIp(ip.clone()));

    let user_id = CookieJar::from_headers(req.headers())
        .get(AUTH_KEY)
        .and_then(|cookie| user_id_from_session_token(cookie.value(), &state.config.jwt_secret));

    let method = req.method().clone();
    let path = req.uri().path().to_owned();

    let span = tracing::info_span!(
        "api_request",
        %ip,
        user_id = user_id.as_deref().unwrap_or("-"),
        %method,
        %path,
    );

    next.run(req).instrument(span).await
}

/// Determine the client IP, preferring proxy headers (`X-Forwarded-For`,
/// `X-Real-IP`) and falling back to the socket address. Mirrors the behaviour
/// of the rate limiter's `SmartIpKeyExtractor`. `addr` is `None` only when the
/// connection info extension is absent (e.g. in tests), in which case we fall
/// back to `"unknown"`.
pub fn client_ip(headers: &HeaderMap, addr: Option<SocketAddr>) -> String {
    if let Some(forwarded) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = forwarded.split(',').next() {
            let trimmed = first.trim();
            if !trimmed.is_empty() {
                return trimmed.to_owned();
            }
        }
    }

    if let Some(real_ip) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        if !real_ip.is_empty() {
            return real_ip.to_owned();
        }
    }

    addr.map(|a| a.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
