use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use super::responses::upsert_response;
use crate::error::ComhairleError;

/// Fixed ids seeded by the migration.
pub const AGE_VERSION_ID: Uuid = Uuid::from_u128(0xde000002_0000_4000_8000_000000000001);
/// Signup writes this answer without a consent row, so reuse is refused and it is asked again.
pub const ETHNICITY_VERSION_ID: Uuid = Uuid::from_u128(0xde000002_0000_4000_8000_000000000002);
pub const GENDER_VERSION_ID: Uuid = Uuid::from_u128(0xde000002_0000_4000_8000_000000000003);
pub const POSTCODE_VERSION_ID: Uuid = Uuid::from_u128(0xde000002_0000_4000_8000_000000000004);
pub const POLITICAL_PARTY_VERSION_ID: Uuid =
    Uuid::from_u128(0xde000002_0000_4000_8000_000000000005);

#[derive(Debug, Default)]
pub struct SignupAnswers {
    pub age: Option<i32>,
    pub ethnicity: Option<String>,
    pub gender: Option<String>,
    pub zipcode: Option<String>,
    pub political_party: Option<String>,
}

pub async fn record_signup_answers(
    db: &PgPool,
    user_id: Uuid,
    answers: SignupAnswers,
) -> Result<(), ComhairleError> {
    let mut conn = db.acquire().await?;
    let candidates = [
        (AGE_VERSION_ID, answers.age.map(|age| json!(age))),
        (ETHNICITY_VERSION_ID, text_answer(answers.ethnicity)),
        (GENDER_VERSION_ID, text_answer(answers.gender)),
        (
            POSTCODE_VERSION_ID,
            text_answer(answers.zipcode).map(|postcode| json!({ "postcode": postcode })),
        ),
        (
            POLITICAL_PARTY_VERSION_ID,
            text_answer(answers.political_party),
        ),
    ];
    for (version_id, value) in candidates {
        if let Some(value) = value {
            upsert_response(&mut conn, user_id, version_id, None, &value).await?;
        }
    }
    Ok(())
}

fn text_answer(value: Option<String>) -> Option<Value> {
    value
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .map(Value::from)
}
