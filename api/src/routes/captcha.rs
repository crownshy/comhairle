use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use aide::axum::{routing::get_with, ApiRouter};
use altcha::{
    create_challenge, verify_solution, Challenge, CreateChallengeOptions, Payload,
    VerifySolutionOptions, VerifySolutionResult,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::Rng;
use serde::Serialize;
use tracing::instrument;

use crate::{config::CaptchaConfig, error::ComhairleError, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn get_challenge(
    State(state): State<Arc<ComhairleState>>,
) -> Result<(StatusCode, Json<Challenge>), ComhairleError> {
    let captch_config = state
        .config
        .captcha
        .as_ref()
        .ok_or(ComhairleError::NoCaptchaConfigured)?;

    let expiry = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 300; // 5 minutes
    let options = CreateChallengeOptions {
        algorithm: "PBKDF2/SHA-256".to_string(),
        cost: 5_000,
        counter: Some(rand::thread_rng().gen_range(5_000..=10_000)),
        expires_at: Some(expiry),
        hmac_signature_secret: Some(captch_config.signature_secret.clone()),
        hmac_key_signature_secret: Some(captch_config.key_secret.clone()),
        ..Default::default()
    };

    let challenge = create_challenge(options)?;

    Ok((StatusCode::OK, Json(challenge)))
}

#[derive(Serialize)]
pub struct CaptchaVerificationResult {
    verified: bool,
    expired: bool,
    invalid_signature: Option<bool>,
    invalid_solution: Option<bool>,
    time: f64,
}

impl From<VerifySolutionResult> for CaptchaVerificationResult {
    fn from(r: VerifySolutionResult) -> Self {
        Self {
            verified: r.verified,
            expired: r.expired,
            invalid_solution: r.invalid_solution,
            invalid_signature: r.invalid_signature,
            time: r.time,
        }
    }
}

pub fn verify(
    solution: String,
    captcha_config: &CaptchaConfig,
) -> Result<CaptchaVerificationResult, ComhairleError> {
    let bytes = BASE64.decode(solution)?;

    let payload: Payload = serde_json::from_slice(&bytes)?;

    let result = verify_solution(VerifySolutionOptions {
        hmac_key_signature_secret: Some(captcha_config.key_secret.clone()),
        ..VerifySolutionOptions::new(
            &payload.challenge,
            &payload.solution,
            captcha_config.signature_secret.clone(),
        )
    })?;

    Ok(result.into())
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/challenge",
            get_with(get_challenge, |op| {
                op.id("GetChallenge")
                    .summary("Get captcha challenge")
                    .description("Returns a captcha challenge for frontend widget to solve")
                    .tag("Captcha")
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::error::Error;

    use crate::models::model_test_helpers::setup_default_app_and_session;

    #[sqlx::test]
    fn should_return_captcha_challenge(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, value, _) = session.get(&app, "/captcha/challenge").await?;

        let challenge: Challenge = serde_json::from_value(value)?;

        assert!(challenge.signature.is_some(), "missing challenge signature");

        Ok(())
    }
}
