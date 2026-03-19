# Rate Limiting Middleware

This module provides IP-based rate limiting for the Comhairle API using tower_governor.

## Features

- **IP-based rate limiting**: Automatically extracts client IP from X-Forwarded-For, X-Real-IP, or socket address
- **Rate limit headers**: Responses include headers indicating remaining requests and reset time
- **Multiple presets**: Different rate limits for different endpoint sensitivity levels
- **Reusable**: Easy to apply to any route or router

## Usage

### Apply to Routes

The rate limiter is currently applied to all auth routes in `lib.rs`:

```rust
let auth_router = routes::auth::router(state.clone())
    .await
    .layer(middleware::rate_limit::auth_rate_limiter());
```

### Available Rate Limiters

#### `auth_rate_limiter()`
**Strict rate limiting for authentication endpoints**
- 5 requests per minute per IP
- Burst capacity of 3 requests
- Use for: login, signup, password reset, email verification

#### `standard_rate_limiter()`
**Standard rate limiting for general API endpoints**
- 100 requests per minute per IP
- Burst capacity of 20 requests
- Use for: most CRUD operations

#### `lenient_rate_limiter()`
**Lenient rate limiting for read-heavy operations**
- 300 requests per minute per IP
- Burst capacity of 50 requests
- Use for: public endpoints, read operations

### Custom Rate Limits

You can also create custom rate limits:

```rust
use crate::middleware::rate_limit::{create_rate_limiter, RateLimitConfig};

let custom_limiter = create_rate_limiter(RateLimitConfig::custom(30, 10));

let router = ApiRouter::new()
    .route("/custom", get(handler))
    .layer(custom_limiter);
```

## Response Headers

When rate limiting is active, responses include the following headers:

- `x-ratelimit-limit`: Maximum requests allowed in the current window
- `x-ratelimit-remaining`: Requests remaining in the current window
- `x-ratelimit-after`: Seconds until the API becomes available (after rate limit exceeded)
- `x-ratelimit-whitelisted`: Indicates if the request method is whitelisted

## Rate Limit Exceeded

When a client exceeds their rate limit:
- HTTP Status: `429 Too Many Requests`
- Response includes `x-ratelimit-after` header with retry delay

## Implementation Details

- Uses `tower_governor` 0.8 with Axum 0.8
- Uses `governor` crate's GCRA (Generic Cell Rate Algorithm)
- Extracts IP using `SmartIpKeyExtractor` which checks:
  1. X-Forwarded-For header
  2. X-Real-IP header
  3. Socket peer address

## References

- [tower_governor Documentation](https://docs.rs/tower_governor/latest/tower_governor/)
- [governor Documentation](https://docs.rs/governor/latest/governor/)
- [GitHub Repository](https://github.com/benwis/tower-governor)
