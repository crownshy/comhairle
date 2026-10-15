# Rate Limiting Middleware

This module provides IP-based rate limiting for the Comhairle API using tower_governor.

## Features

- **IP-based rate limiting**: Automatically extracts client IP from X-Forwarded-For, X-Real-IP, or socket address
- **Rate limit headers**: Responses include headers indicating remaining requests and reset time
- **Multiple presets**: Different rate limits for different endpoint sensitivity levels
- **Reusable**: Easy to apply to any route or router

## Usage

Limiters are created once per app in `middleware::rate_limit::RateLimitLayers`
and applied to individual routes or routers.

### Why one instance of each limiter

Each limiter keeps its own per-IP bucket. A new set of buckets is created every
time a `tower_governor` config is built, so **two limiters for the same purpose
count separately**. If `/auth/login` and `/other_router/something_auth` each
had their own strict limiter, a client blocked on one could carry on at the other
with a fresh allowance.

To prevent this, every limit it built exactly once, in `RateLimitLayers::new()`.
The config builder is private to `rate_limit.rs`, so routers can't create limiters
of their own; they can only get layers from `RateLimitLayers`.

### Apply to Router

```rust
use tower::util::option_layer;

use crate::middleware::rate_limit::RateLimitLayers;

let rate_limiters = state.config.enable_rate_limiting
    .then(|| RateLimitLayers::new());

...

// Wrap in `option_layer` to the route works with rate limiting on or off.
let auth_router = routes::auth::router()
    .await
    .layer(option_layer(rate_limiters.auth_rate_limiter()));
```

### Apply to routes

Take `Option<&RateLimitLayers>` in your router function, call the accessor for the
limit you want, and wrap it in `option_layer` so the route works with rate
limiting on or off:

```rust
use tower::util::option_layer;

use crate::middleware::rate_limit::RateLimitLayers;

pub fn router(rate_limiters: Option<&RateLimitLayers>) -> ApiRouter<Arc<ComhairleState>> {
    let standard_limit =
        option_layer(rate_limiters.map(|limiters| limiters.standard_rate_limiter()));

    ApiRouter::new().api_route(
        "/",
        post_with(create, |op| op.id("CreateResource")).layer(standard_limit),
    )
}
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

let custom_limiter = create_custom_rate_limiter(RateLimitConfig::custom(30, 10));

let router = ApiRouter::new()
    .route("/custom", get(handler))
    .layer(custom_limiter);
```

## Cleaning up idle entries

Each limiter stores an entry for every IP it has seen. `RateLimitLayers::retain_recent`
drops entries whose buckets have fully replenished. Those entries behave exactly
like new ones, so dropping them never changes a limit. It can be run periodically
to clean up stale entries.

## Response Headers

Only the auth rate limiter adds `x-ratelimit-*` headers to successful responses.
Every limiter adds `retry-after` when it rejects a request.

### Auth rate limiter (`auth_rate_limiter()`)

Built with `.use_headers()` (`StateInformationMiddleware`).

On every response from a route behind it:

- `x-ratelimit-limit`: the burst size, the most requests allowed back to back.
- `x-ratelimit-remaining`: how many of those requests the client has left.

When it rejects a request (`429`), the response also includes:

- `retry-after` and `x-ratelimit-after`: whole seconds until the next request is
  allowed.
- `x-ratelimit-remaining`: `0`.

### Standard and lenient rate limiters

Built without `.use_headers()` (`NoOpMiddleware`).

- Allowed requests pass through with **no** rate limit headers.
- Rejected requests (`429`) include only `retry-after` and `x-ratelimit-after`.

### Why only the auth rate limiter adds headers

Routes can sit behind more than one limiter. The lenient limiter wraps the whole
API in `build_router_and_spec`, so every auth route is behind both the global
limiter and the auth limiter.

When two limiters both add headers, the outer one runs last on the way out and
**overwrites** the inner one's `x-ratelimit-limit` and `x-ratelimit-remaining`.
If the global limiter added headers, auth responses would report the global
allowance instead of the auth one. Even a `429` caused by the auth limiter would
say something like `x-ratelimit-limit: 50, x-ratelimit-remaining: 47`.

Keeping headers on the auth limiter only avoids that:

- The numbers on auth routes always describe the auth limiter, the one that
  matters there. It allows only 3 attempts in a burst, so a client can use
  `x-ratelimit-remaining` to tell the user how many attempts they have left.
- The standard and lenient allowances (20 and 50 back to back) are large enough
  that clients have no use for a running count. On those routes, `retry-after`
  on a `429` is all a client needs.

`retry-after` is never overwritten. A limiter that allows a request adds nothing
to a rejection made by another one, so `retry-after` always comes from the
limiter that rejected the request. Clients should rely on `retry-after`, not on
`x-ratelimit-remaining`, to know when to retry.

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
