use std::sync::Arc;

use axum::body::Body;
use governor::middleware::{NoOpMiddleware, StateInformationMiddleware};
use tower_governor::GovernorLayer;
use tower_governor::governor::{GovernorConfig, GovernorConfigBuilder};
use tower_governor::key_extractor::SmartIpKeyExtractor;

/// Configuration for rate limiting
#[derive(Clone)]
pub struct RateLimitConfig {
    /// Requests allowed per minute
    pub per_minute: u32,
    /// Burst capacity (max requests allowed at once)
    pub burst_size: u32,
}

impl RateLimitConfig {
    /// Creates a strict rate limit config suitable for authentication endpoints
    /// 5 requests per minute with a burst of 3
    pub fn auth_strict() -> Self {
        Self {
            per_minute: 5,
            burst_size: 3,
        }
    }

    /// Creates a rate limit config for OTP endpoints.
    ///
    /// A single OTP sign-in already costs two credential-shaped requests
    /// (`create_otp` then `login_otp`), plus a possible resend, so it needs more
    /// headroom than `auth_strict` to avoid locking out a genuine sign-in.
    /// 10 requests per minute with a burst of 6
    pub fn otp_strict() -> Self {
        Self {
            per_minute: 10,
            burst_size: 6,
        }
    }

    /// Creates a standard rate limit config for general API endpoints
    /// 100 requests per minute with a burst of 20
    pub fn standard() -> Self {
        Self {
            per_minute: 100,
            burst_size: 20,
        }
    }

    /// Creates a lenient rate limit config for less sensitive endpoints
    /// 300 requests per minute with a burst of 50
    pub fn lenient() -> Self {
        Self {
            per_minute: 300,
            burst_size: 50,
        }
    }

    /// Creates a custom rate limit config
    #[cfg_attr(not(test), expect(dead_code, reason = "For future use"))]
    pub fn custom(per_minute: u32, burst_size: u32) -> Self {
        Self {
            per_minute,
            burst_size,
        }
    }
}

pub type LimitLayer<M> = GovernorLayer<SmartIpKeyExtractor, M, Body>;

/// The single set of rate limiters shared by the whole API.
///
/// Each limiter keeps its own per-IP buckets, and a new set is created every time a
/// `tower_governor` config is built. Two limiters for the same purpose would give a
/// client a separate allowance on each: a client blocked on `/auth/login` could
/// carry on at `/other_router/auth_endpoint`. So each limit is built once, in
/// [`RateLimitLayers::new`], and routers get their layers from the accessor methods
/// rather than building their own.
///
/// `build_app` creates one instance per app when
/// `ComhairleConfig::enable_rate_limiting` is set, and passes it to the routers as
/// `Option<&RateLimitLayers>` (`None` when rate limiting is off).
#[derive(Clone)]
pub struct RateLimitLayers {
    standard: Arc<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>>,
    auth_strict: Arc<GovernorConfig<SmartIpKeyExtractor, StateInformationMiddleware>>,
    otp_strict: Arc<GovernorConfig<SmartIpKeyExtractor, StateInformationMiddleware>>,
    lenient: Arc<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>>,
}

impl RateLimitLayers {
    /// Builds each limiter, and its per-IP buckets, once.
    ///
    /// Call this once per app. Every call creates new, independent buckets.
    pub fn new() -> Self {
        Self {
            standard: Arc::new(build_governor_config(RateLimitConfig::standard())),
            auth_strict: Arc::new(build_governor_config_with_headers(
                RateLimitConfig::auth_strict(),
            )),
            otp_strict: Arc::new(build_governor_config_with_headers(
                RateLimitConfig::otp_strict(),
            )),
            lenient: Arc::new(build_governor_config(RateLimitConfig::lenient())),
        }
    }

    /// Pre-configured rate limiter for standard API endpoints
    /// - 100 requests per minute per IP
    /// - Burst capacity of 20 requests
    /// - Suitable for most CRUD operations
    pub fn standard_rate_limiter(&self) -> LimitLayer<NoOpMiddleware> {
        GovernorLayer::new(Arc::clone(&self.standard))
    }

    /// Pre-configured rate limiter for authentication endpoints (strict)
    /// - 5 requests per minute per IP
    /// - Burst capacity of 3 requests
    /// - Suitable for login, signup, password reset, etc.
    ///
    /// This is the only rate limiter that adds `x-ratelimit-*` headers to responses.
    pub fn auth_rate_limiter(&self) -> LimitLayer<StateInformationMiddleware> {
        GovernorLayer::new(Arc::clone(&self.auth_strict))
    }

    /// Pre-configured rate limiter for OTP endpoints (create/login/resend by OTP)
    /// - 10 requests per minute per IP
    /// - Burst capacity of 6 requests
    /// - Kept separate from `auth_rate_limiter` so a normal OTP sign-in
    ///   (create + login, plus an occasional resend) isn't starved by the
    ///   stricter password login/signup bucket on the same IP.
    ///
    /// This is also wired to add `x-ratelimit-*` headers to responses.
    pub fn otp_rate_limiter(&self) -> LimitLayer<StateInformationMiddleware> {
        GovernorLayer::new(Arc::clone(&self.otp_strict))
    }

    /// Pre-configured rate limiter for lenient endpoints
    /// - 300 requests per minute per IP
    /// - Burst capacity of 50 requests
    /// - Suitable for read-heavy operations
    pub fn lenient_rate_limiter(&self) -> LimitLayer<NoOpMiddleware> {
        GovernorLayer::new(Arc::clone(&self.lenient))
    }

    /// Drops per-IP entries whose buckets have fully replenished
    ///
    /// A dropped entry behaves exactly like a new one, so this never changes a
    /// limit. Without it, each limiter keeps an entry for every IP it has seen, for
    /// the life of the process. `build_app` runs it on an interval.
    pub fn retain_recent(&self) {
        self.standard.limiter().retain_recent();
        self.auth_strict.limiter().retain_recent();
        self.otp_strict.limiter().retain_recent();
        self.lenient.limiter().retain_recent();
    }

    /// Spawns a separate background task to clean up stale entries.
    /// See [tower-governor docs](https://deepwiki.com/benwis/tower-governor/4.1-axum-integration)
    pub fn spawn_cleanup(&self, duration: tokio::time::Duration) {
        let limiters = self.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(duration).await;
                limiters.retain_recent();
            }
        });
    }
}

fn governor_builder(
    config: &RateLimitConfig,
) -> GovernorConfigBuilder<SmartIpKeyExtractor, NoOpMiddleware> {
    // Convert per_minute to per_millisecond for governor
    // Use ms over seconds to account for more lenient rate limits (above 60
    // minute) which result in a refill period of less than one second
    let per_ms = 60_000 / config.per_minute as u64;

    GovernorConfigBuilder::default()
        .per_millisecond(per_ms)
        .burst_size(config.burst_size)
        .key_extractor(SmartIpKeyExtractor)
}

#[allow(
    clippy::expect_used,
    reason = "only run when starting api so acceptable to panic"
)]
fn build_governor_config(
    config: RateLimitConfig,
) -> GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware> {
    governor_builder(&config)
        .finish()
        .expect("Failed to create rate limiter configuration")
}

#[allow(
    clippy::expect_used,
    reason = "only run when starting api so acceptable to panic"
)]
fn build_governor_config_with_headers(
    config: RateLimitConfig,
) -> GovernorConfig<SmartIpKeyExtractor, StateInformationMiddleware> {
    governor_builder(&config)
        .use_headers()
        .finish()
        .expect("Failed to create rate limiter configuration")
}

/// Creates a custom rate limiter layer based on the provided configuration.
///
/// Warning! This should only be used to create custom rate limiters with
/// [`RateLimitConfig::custom()`]. Other rate limit types should use the globally
/// available instances from [`RateLimitLayers`]. See middleware::README.md
#[allow(dead_code, reason = "For possible future use")]
pub fn create_custom_rate_limiter(
    config: RateLimitConfig,
) -> GovernorLayer<SmartIpKeyExtractor, NoOpMiddleware, Body> {
    let governor_conf = build_governor_config(config);

    GovernorLayer::new(Arc::new(governor_conf))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::Router;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use tower::ServiceExt;

    use super::*;

    #[test]
    fn test_auth_strict_config() {
        let config = RateLimitConfig::auth_strict();
        assert_eq!(config.per_minute, 5);
        assert_eq!(config.burst_size, 3);
    }

    #[test]
    fn test_standard_config() {
        let config = RateLimitConfig::standard();
        assert_eq!(config.per_minute, 100);
        assert_eq!(config.burst_size, 20);
    }

    #[test]
    fn test_custom_config() {
        let config = RateLimitConfig::custom(50, 10);
        assert_eq!(config.per_minute, 50);
        assert_eq!(config.burst_size, 10);
    }

    async fn status_for_hit(app: &Router) -> StatusCode {
        let request = Request::builder()
            .header("X-Forwarded-For", "10.0.0.1")
            .body(Body::empty())
            .unwrap();

        app.clone().oneshot(request).await.unwrap().status()
    }

    /// Sends requests until one is rejected and returns how many got through
    async fn successes_until_429(app: &Router, max: u32) -> u32 {
        for allowed in 0..=max {
            if status_for_hit(app).await == StatusCode::TOO_MANY_REQUESTS {
                return allowed;
            }
        }
        panic!("limiter let more than {max} request through");
    }

    /// Empties the bucket, waits then returns how many requests were replenished
    /// in that time
    async fn replenished_after(app: Router, burst: u32, wait: Duration) -> u32 {
        assert_eq!(
            successes_until_429(&app, burst).await,
            burst,
            "the full burst should be allowed up front"
        );

        tokio::time::sleep(wait).await;
        successes_until_429(&app, burst).await
    }

    /// 300/min (lenient) is one request every 200ms (refill period). The wait sits halfway between
    /// refills so timer jitter cannot affect the count.
    #[tokio::test]
    async fn test_lenient_replenished_at_300_per_minute() {
        let limiters = RateLimitLayers::new();
        let router = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(limiters.lenient_rate_limiter());

        let replenished = replenished_after(
            router,
            RateLimitConfig::lenient().burst_size,
            Duration::from_millis(1100),
        )
        .await;

        // 1100ms / 200ms -> 5 successful refills
        assert_eq!(
            replenished, 5,
            "300/min should replenish 5 requests in 1.1s, got {replenished}"
        );
    }

    /// 100/min (standard) is one request every 600ms (refill period)
    #[tokio::test]
    async fn test_standard_replenishes_at_100_per_minute() {
        let limiters = RateLimitLayers::new();
        let router = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(limiters.standard_rate_limiter());

        let replenished = replenished_after(
            router,
            RateLimitConfig::standard().burst_size,
            Duration::from_millis(1500),
        )
        .await;

        // 1500ms / 600ms -> 2 successful refills
        assert_eq!(
            replenished, 2,
            "100/min should replenish 2 requests in 1.5s, got {replenished}"
        );
    }

    /// 5/min (auth) is one request every 12s (refill period), so nothing comes
    /// back within a second.
    #[tokio::test]
    async fn test_auth_strict_does_not_replish_within_second() {
        let limiters = RateLimitLayers::new();
        let router = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(limiters.auth_rate_limiter());

        let replenished = replenished_after(
            router,
            RateLimitConfig::auth_strict().burst_size,
            Duration::from_millis(1100),
        )
        .await;

        assert_eq!(
            replenished, 0,
            "5/min should replenish nothing in 1.1s, got {replenished}"
        );
    }
}

#[cfg(test)]
#[path = "rate_limit_tests.rs"]
mod rate_limit_tests;
