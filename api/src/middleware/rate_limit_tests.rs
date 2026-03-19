#[cfg(test)]
mod tests {
    use crate::test_helpers::{test_state, TEST_PASSWORD};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use sqlx::PgPool;
    use std::{error::Error, sync::Arc};
    use tower::ServiceExt;

    /// Helper to create test state with rate limiting enabled
    fn test_state_with_rate_limiting(pool: sqlx::PgPool) -> Result<Arc<crate::ComhairleState>, Box<dyn Error>> {
        let mut state = test_state().db(pool).call()?;
        state.config.enable_rate_limiting = true; // Enable rate limiting for these tests
        Ok(Arc::new(state))
    }

    /// Helper to create an app with rate limiting enabled for testing
    async fn setup_rate_limited_server(
        state: Arc<crate::ComhairleState>,
    ) -> Result<axum::Router, Box<dyn Error>> {
        // With config.enable_rate_limiting = true, setup_server will apply rate limiting
        Ok(crate::setup_server(state).await?)
    }

    /// Helper to make a signup request with a custom IP address
    async fn signup_request_with_ip(
        app: &axum::Router,
        ip: &str,
        username: &str,
        email: &str,
    ) -> Result<axum::response::Response, Box<dyn Error>> {
        let body = json!({
            "username": username,
            "password": TEST_PASSWORD,
            "email": email
        })
        .to_string();

        let request = Request::builder()
            .uri("/auth/signup")
            .method("POST")
            .header("content-type", "application/json")
            .header("X-Forwarded-For", ip) // Set the IP for rate limiting
            .body(Body::from(body))
            .unwrap();

        Ok(app.clone().oneshot(request).await?)
    }

    /// Helper to make a login request with a custom IP address
    async fn login_request_with_ip(
        app: &axum::Router,
        ip: &str,
        email: &str,
        password: &str,
    ) -> Result<axum::response::Response, Box<dyn Error>> {
        let body = json!({
            "email": email,
            "password": password
        })
        .to_string();

        let request = Request::builder()
            .uri("/auth/login")
            .method("POST")
            .header("content-type", "application/json")
            .header("X-Forwarded-For", ip)
            .body(Body::from(body))
            .unwrap();

        Ok(app.clone().oneshot(request).await?)
    }

    #[sqlx::test]
    async fn test_signup_within_burst_limit_succeeds(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state_with_rate_limiting(pool)?;
        let app = setup_rate_limited_server(state).await?;

        // Auth rate limit: 5 requests per minute with burst of 3
        // First 3 requests should succeed immediately (burst)
        for i in 0..3 {
            let response = signup_request_with_ip(
                &app,
                "192.168.1.100",
                &format!("user{}", i),
                &format!("user{}@test.com", i),
            )
            .await?;

            assert_eq!(
                response.status(),
                StatusCode::CREATED,
                "Request {} should succeed within burst limit",
                i + 1
            );

            // Check for rate limit headers
            let headers = response.headers();
            assert!(
                headers.contains_key("x-ratelimit-limit"),
                "Response should include x-ratelimit-limit header"
            );
            assert!(
                headers.contains_key("x-ratelimit-remaining"),
                "Response should include x-ratelimit-remaining header"
            );
        }

        Ok(())
    }

    #[sqlx::test]
    async fn test_signup_exceeding_rate_limit_returns_429(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state_with_rate_limiting(pool)?;
        let app = setup_rate_limited_server(state).await?;

        // Auth rate limit: 5 requests per minute with burst of 3
        // Make 6 requests - the 6th should be rate limited
        let mut last_response = None;
        for i in 0..6 {
            let response = signup_request_with_ip(
                &app,
                "192.168.1.101",
                &format!("user{}", i),
                &format!("user{}@test.com", i),
            )
            .await?;

            last_response = Some(response);
        }

        let response = last_response.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::TOO_MANY_REQUESTS,
            "Request exceeding rate limit should return 429"
        );

        // Check for rate limit headers
        let headers = response.headers();
        assert!(
            headers.contains_key("x-ratelimit-after"),
            "Rate limited response should include x-ratelimit-after header"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_login_exceeding_rate_limit_returns_429(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state_with_rate_limiting(pool)?;
        let app = setup_rate_limited_server(state).await?;

        // First, create a user
        let email = "testuser@test.com";
        let _ = signup_request_with_ip(&app, "192.168.1.200", "testuser", email).await?;

        // Now make multiple login attempts from a different IP to hit rate limit
        let mut last_response = None;
        for _ in 0..6 {
            let response = login_request_with_ip(&app, "192.168.1.102", email, TEST_PASSWORD).await?;
            last_response = Some(response);
        }

        let response = last_response.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::TOO_MANY_REQUESTS,
            "Login request exceeding rate limit should return 429"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_different_ips_have_separate_rate_limits(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state_with_rate_limiting(pool)?;
        let app = setup_rate_limited_server(state).await?;

        // Make 3 requests from IP 1 (within burst limit)
        for i in 0..3 {
            let response = signup_request_with_ip(
                &app,
                "192.168.1.103",
                &format!("user_ip1_{}", i),
                &format!("user_ip1_{}@test.com", i),
            )
            .await?;

            assert_eq!(
                response.status(),
                StatusCode::CREATED,
                "IP 1 request {} should succeed",
                i + 1
            );
        }

        // Make 3 requests from IP 2 (should also succeed - separate limit)
        for i in 0..3 {
            let response = signup_request_with_ip(
                &app,
                "192.168.1.104",
                &format!("user_ip2_{}", i),
                &format!("user_ip2_{}@test.com", i),
            )
            .await?;

            assert_eq!(
                response.status(),
                StatusCode::CREATED,
                "IP 2 request {} should succeed (separate rate limit)",
                i + 1
            );
        }

        Ok(())
    }

    #[sqlx::test]
    async fn test_rate_limit_headers_are_present(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state_with_rate_limiting(pool)?;
        let app = setup_rate_limited_server(state).await?;

        let response =
            signup_request_with_ip(&app, "192.168.1.105", "headertest", "headertest@test.com")
                .await?;

        let headers = response.headers();

        // Check that all expected rate limit headers are present
        assert!(
            headers.contains_key("x-ratelimit-limit"),
            "Response should include x-ratelimit-limit header"
        );

        assert!(
            headers.contains_key("x-ratelimit-remaining"),
            "Response should include x-ratelimit-remaining header"
        );

        // Parse and validate header values
        let limit = headers
            .get("x-ratelimit-limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u32>().ok());

        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u32>().ok());

        assert!(limit.is_some(), "x-ratelimit-limit should be parseable");
        assert!(
            remaining.is_some(),
            "x-ratelimit-remaining should be parseable"
        );

        let limit = limit.unwrap();
        let remaining = remaining.unwrap();

        assert!(
            limit > 0,
            "Rate limit should be greater than 0, got {}",
            limit
        );
        assert!(
            remaining < limit,
            "Remaining should be less than limit after one request. limit={}, remaining={}",
            limit,
            remaining
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_rate_limit_decreases_with_each_request(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state_with_rate_limiting(pool)?;
        let app = setup_rate_limited_server(state).await?;

        let mut previous_remaining: Option<u32> = None;

        // Make 3 requests and verify remaining count decreases
        for i in 0..3 {
            let response = signup_request_with_ip(
                &app,
                "192.168.1.106",
                &format!("decreasetest{}", i),
                &format!("decreasetest{}@test.com", i),
            )
            .await?;

            assert_eq!(response.status(), StatusCode::CREATED);

            let remaining = response
                .headers()
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u32>().ok())
                .expect("Should have x-ratelimit-remaining header");

            if let Some(prev) = previous_remaining {
                assert!(
                    remaining < prev,
                    "Remaining count should decrease with each request. Previous: {}, Current: {}",
                    prev,
                    remaining
                );
            }

            previous_remaining = Some(remaining);
        }

        Ok(())
    }
}
