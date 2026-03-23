use crate::config::ComhairleConfig;
use axum::body::Body;
use governor::middleware::StateInformationMiddleware;
use std::sync::Arc;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};

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
    pub fn custom(per_minute: u32, burst_size: u32) -> Self {
        Self {
            per_minute,
            burst_size,
        }
    }
}

/// Creates a rate limiter layer based on the provided configuration
/// This layer extracts the client IP from X-Forwarded-For, X-Real-IP, or socket address
pub fn create_rate_limiter(
    config: RateLimitConfig,
) -> GovernorLayer<SmartIpKeyExtractor, StateInformationMiddleware, Body> {
    // Convert per_minute to per_second for governor
    let per_second = config.per_minute as f64 / 60.0;
    let seconds = (1.0 / per_second).ceil() as u64;

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(seconds)
        .burst_size(config.burst_size)
        .use_headers() // Add rate limit info to response headers
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .expect("Failed to create rate limiter configuration");

    GovernorLayer::new(Arc::new(governor_conf))
}

/// Pre-configured rate limiter for authentication endpoints (strict)
/// - 5 requests per minute per IP
/// - Burst capacity of 3 requests
/// - Suitable for login, signup, password reset, etc.
pub fn auth_rate_limiter() -> GovernorLayer<SmartIpKeyExtractor, StateInformationMiddleware, Body>
{
    create_rate_limiter(RateLimitConfig::auth_strict())
}

/// Config-aware rate limiter that only applies if enabled in config
/// Returns Some(layer) if rate limiting is enabled, None otherwise
pub fn auth_rate_limiter_if_enabled(
    config: &ComhairleConfig,
) -> Option<GovernorLayer<SmartIpKeyExtractor, StateInformationMiddleware, Body>> {
    if config.enable_rate_limiting {
        Some(auth_rate_limiter())
    } else {
        None
    }
}

/// Pre-configured rate limiter for standard API endpoints
/// - 100 requests per minute per IP
/// - Burst capacity of 20 requests
/// - Suitable for most CRUD operations
pub fn standard_rate_limiter(
) -> GovernorLayer<SmartIpKeyExtractor, StateInformationMiddleware, Body> {
    create_rate_limiter(RateLimitConfig::standard())
}

/// Pre-configured rate limiter for lenient endpoints
/// - 300 requests per minute per IP
/// - Burst capacity of 50 requests
/// - Suitable for read-heavy operations
pub fn lenient_rate_limiter(
) -> GovernorLayer<SmartIpKeyExtractor, StateInformationMiddleware, Body> {
    create_rate_limiter(RateLimitConfig::lenient())
}

#[cfg(test)]
mod tests {
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
}

#[cfg(test)]
#[path = "rate_limit_tests.rs"]
mod rate_limit_tests;
