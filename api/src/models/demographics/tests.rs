//! Database-backed tests for the demographics model.

use std::error::Error;

use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use super::defaults::{ETHNICITY_VERSION_ID, GENDER_VERSION_ID};
use super::forms::{self, FormDraftInput};
use super::questions::{self, QuestionDraftInput, QuestionVersionDto};
use super::responses::{self, ConsentState, ResponseInput};
use super::validation::validate_answer;
use super::{QuestionType, VersionStatus};
use crate::error::ComhairleError;
use crate::models::organization::{self, CreateOrganization, OrganizationType};
use crate::routes::auth::SignupRequest;

const LOCALE: &str = "en";
const AGE_QUESTION_ID: Uuid = Uuid::from_u128(0xde000001_0000_4000_8000_000000000001);
const ETHNICITY_QUESTION_ID: Uuid = Uuid::from_u128(0xde000001_0000_4000_8000_000000000002);
const GENDER_QUESTION_ID: Uuid = Uuid::from_u128(0xde000001_0000_4000_8000_000000000003);

type TestResult = Result<(), Box<dyn Error>>;

fn question_input(value: Value) -> QuestionDraftInput {
    serde_json::from_value(value).expect("question input should deserialize")
}

fn form_input(value: Value) -> FormDraftInput {
    serde_json::from_value(value).expect("form input should deserialize")
}

async fn create_user(pool: &PgPool, name: &str) -> Result<Uuid, Box<dyn Error>> {
    let user = crate::models::users::create_user(
        &SignupRequest {
            username: name.to_string(),
            password: "test_pw".to_string(),
            email: format!("{name}@example.com"),
            avatar_url: None,
        },
        pool,
    )
    .await?;
    Ok(user.id)
}

async fn create_org(pool: &PgPool) -> Result<Uuid, Box<dyn Error>> {
    let admin_id = create_user(pool, "org_admin").await?;
    let org = organization::create_with_administrator(
        pool,
        &CreateOrganization {
            name: "Demographics Test Org".to_string(),
            description: "Description".to_string(),
            mission: "Mission".to_string(),
            org_type: OrganizationType::NonProfit,
            contact_email: None,
            external_url: None,
            regions: None,
        },
        LOCALE,
        admin_id,
    )
    .await?;
    Ok(org.id)
}

async fn first_tag_id(pool: &PgPool) -> Result<Uuid, Box<dyn Error>> {
    let tags = questions::list_tags(pool, None).await?;
    Ok(tags.first().ok_or("seeded tag missing")?.id)
}

/// Creates a question with a draft v1 and publishes it, returning the published version.
async fn publish_new_question(
    pool: &PgPool,
    org_id: Uuid,
    mut draft: Value,
) -> Result<QuestionVersionDto, Box<dyn Error>> {
    draft["tag_ids"] = json!([first_tag_id(pool).await?]);
    let created =
        questions::create_question(pool, Some(org_id), &question_input(draft), LOCALE).await?;
    let published = questions::publish_question(pool, Some(org_id), created.id, LOCALE).await?;
    Ok(published
        .latest_published
        .ok_or("publish left no version")?)
}

#[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
async fn should_advance_version_numbers_and_never_repeat_published_ones(
    pool: PgPool,
) -> TestResult {
    let org_id = create_org(&pool).await?;
    let tag_id = first_tag_id(&pool).await?;
    let created = questions::create_question(
        &pool,
        Some(org_id),
        &question_input(json!({
            "text": "Favourite colour",
            "question_type": "any_string",
            "tag_ids": [tag_id],
        })),
        LOCALE,
    )
    .await?;
    assert_eq!(created.draft.as_ref().map(|d| d.version_number), Some(1));

    questions::publish_question(&pool, Some(org_id), created.id, LOCALE).await?;
    questions::start_draft(&pool, Some(org_id), created.id, LOCALE).await?;
    questions::update_draft(
        &pool,
        Some(org_id),
        created.id,
        &question_input(json!({
            "text": "Favourite colour (edited)",
            "question_type": "any_string",
            "tag_ids": [tag_id],
        })),
        LOCALE,
    )
    .await?;
    let second = questions::publish_question(&pool, Some(org_id), created.id, LOCALE).await?;
    let latest = second.latest_published.ok_or("v2 should be published")?;
    assert_eq!(latest.version_number, 2);
    assert_eq!(latest.text, "Favourite colour (edited)");

    questions::start_draft(&pool, Some(org_id), created.id, LOCALE).await?;
    let third = questions::publish_question(&pool, Some(org_id), created.id, LOCALE).await?;
    let published_numbers: Vec<i32> = third
        .versions
        .iter()
        .filter(|v| v.status == VersionStatus::Published)
        .map(|v| v.version_number)
        .collect();
    assert_eq!(published_numbers, vec![1, 2, 3]);
    Ok(())
}

#[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
async fn should_keep_published_versions_when_draft_is_discarded(pool: PgPool) -> TestResult {
    let org_id = create_org(&pool).await?;
    let version = publish_new_question(
        &pool,
        org_id,
        json!({"text": "Original", "question_type": "any_string"}),
    )
    .await?;
    let question_id = version.question_id;

    questions::start_draft(&pool, Some(org_id), question_id, LOCALE).await?;
    questions::discard_draft(&pool, Some(org_id), question_id).await?;

    let question = questions::get_question(&pool, Some(org_id), question_id, LOCALE).await?;
    assert!(question.draft.is_none());
    let latest = question
        .latest_published
        .ok_or("published version should remain")?;
    assert_eq!(latest.id, version.id);
    assert_eq!(latest.text, "Original");
    assert_eq!(question.versions.len(), 1);
    Ok(())
}

#[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
async fn should_fork_default_question_once_per_organization(pool: PgPool) -> TestResult {
    let org_id = create_org(&pool).await?;

    let first = questions::start_draft(&pool, Some(org_id), AGE_QUESTION_ID, LOCALE).await?;
    assert_eq!(first.forked_from_id, Some(AGE_QUESTION_ID));
    assert_eq!(first.organization_id, Some(org_id));
    let draft = first.draft.as_ref().ok_or("fork should have a draft")?;
    assert_eq!(draft.question_type, QuestionType::Number);

    let second = questions::start_draft(&pool, Some(org_id), AGE_QUESTION_ID, LOCALE).await?;
    assert_eq!(second.id, first.id);

    let visible = questions::list_questions(&pool, Some(org_id), LOCALE).await?;
    assert!(visible.iter().any(|q| q.id == first.id));
    assert!(!visible.iter().any(|q| q.id == AGE_QUESTION_ID));
    Ok(())
}

#[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
async fn should_pin_latest_published_question_versions_in_published_form(
    pool: PgPool,
) -> TestResult {
    let org_id = create_org(&pool).await?;
    let v1 = publish_new_question(
        &pool,
        org_id,
        json!({"text": "Original", "question_type": "any_string"}),
    )
    .await?;
    let question_id = v1.question_id;

    let unpublished = questions::create_question(
        &pool,
        Some(org_id),
        &question_input(json!({"text": "Never published", "question_type": "any_string"})),
        LOCALE,
    )
    .await?;
    let rejected = forms::create_form(
        &pool,
        org_id,
        &form_input(json!({
            "name": "Bad form",
            "entries": [{"question_id": unpublished.id, "position": 1, "required": true}],
        })),
        LOCALE,
    )
    .await;
    assert!(matches!(rejected, Err(ComhairleError::BadRequest(_))));

    let form = forms::create_form(
        &pool,
        org_id,
        &form_input(json!({
            "name": "Survey",
            "entries": [{"question_id": question_id, "position": 1, "required": true}],
        })),
        LOCALE,
    )
    .await?;
    questions::start_draft(&pool, Some(org_id), question_id, LOCALE).await?;
    questions::update_draft(
        &pool,
        Some(org_id),
        question_id,
        &question_input(json!({
            "text": "Draft wording",
            "question_type": "any_string",
            "tag_ids": [first_tag_id(&pool).await?],
        })),
        LOCALE,
    )
    .await?;

    let published = forms::publish_form(&pool, org_id, form.id, LOCALE).await?;
    let pinned = published
        .latest_published
        .ok_or("form should be published")?;
    assert_eq!(pinned.entries[0].question_version_id, Some(v1.id));
    assert_eq!(pinned.entries[0].question_text, "Original");

    questions::publish_question(&pool, Some(org_id), question_id, LOCALE).await?;
    let after = forms::get_form(&pool, org_id, form.id, LOCALE).await?;
    let still_pinned = after
        .latest_published
        .ok_or("published form should stay published")?;
    assert_eq!(still_pinned.entries[0].question_version_id, Some(v1.id));
    Ok(())
}

#[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
async fn should_validate_select_date_and_location_answers(pool: PgPool) -> TestResult {
    let org_id = create_org(&pool).await?;

    let select = publish_new_question(
        &pool,
        org_id,
        json!({
            "text": "Employment",
            "question_type": "single_select",
            "options": ["Employed", "Unemployed"],
        }),
    )
    .await?;
    let option_ids: Vec<Uuid> = select.options.iter().map(|o| o.id).collect();
    let single =
        json!({"option_ids": [option_ids[0]], "other_text": null, "prefer_not_to_say": false});
    assert!(validate_answer(&select, &single).is_ok());

    let two_options =
        json!({"option_ids": option_ids, "other_text": null, "prefer_not_to_say": false});
    assert!(matches!(
        validate_answer(&select, &two_options),
        Err(ComhairleError::BadRequest(_))
    ));
    let unlisted_other =
        json!({"option_ids": [option_ids[0]], "other_text": "Retired", "prefer_not_to_say": false});
    assert!(matches!(
        validate_answer(&select, &unlisted_other),
        Err(ComhairleError::BadRequest(_))
    ));

    let date = publish_new_question(
        &pool,
        org_id,
        json!({"text": "Date of birth", "question_type": "date", "date_precision": "day"}),
    )
    .await?;
    assert!(validate_answer(&date, &json!("2024/02/29")).is_ok());
    assert!(validate_answer(&date, &json!("2023/02/29")).is_err());
    assert!(validate_answer(&date, &json!("2024/02")).is_err());

    let postcode = publish_new_question(
        &pool,
        org_id,
        json!({"text": "Postcode", "question_type": "location", "location_format": "postcode"}),
    )
    .await?;
    assert!(validate_answer(&postcode, &json!({"postcode": "SW1A 1AA"})).is_ok());
    assert!(validate_answer(&postcode, &json!({"latitude": 1.0, "longitude": 2.0})).is_err());
    Ok(())
}

#[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
async fn should_gate_special_category_answers_on_consent(pool: PgPool) -> TestResult {
    let user_id = create_user(&pool, "participant").await?;
    let version_id = ETHNICITY_VERSION_ID;
    let answer = ResponseInput {
        value: json!("Irish"),
        conversation_id: None,
    };

    let refused = responses::put_response(&pool, user_id, version_id, &answer, LOCALE).await;
    assert!(matches!(refused, Err(ComhairleError::Unprocessable(_))));

    responses::set_consent(&pool, user_id, version_id, false, LOCALE).await?;
    let refused = responses::put_response(&pool, user_id, version_id, &answer, LOCALE).await;
    assert!(matches!(refused, Err(ComhairleError::Unprocessable(_))));

    let ethnicity = questions::load_latest_published(&pool, ETHNICITY_QUESTION_ID, LOCALE)
        .await?
        .ok_or("ethnicity default should be published")?;
    let mut conn = pool.acquire().await?;
    assert_eq!(
        responses::consent_state(&mut conn, user_id, version_id).await?,
        ConsentState::Declined
    );
    assert!(
        responses::reusable_response(&mut conn, user_id, &ethnicity)
            .await?
            .is_none()
    );
    drop(conn);

    responses::set_consent(&pool, user_id, version_id, true, LOCALE).await?;
    responses::put_response(&pool, user_id, version_id, &answer, LOCALE).await?;
    let mut conn = pool.acquire().await?;
    let reused = responses::reusable_response(&mut conn, user_id, &ethnicity)
        .await?
        .ok_or("granted consent should allow reuse")?;
    assert_eq!(reused.value, json!("Irish"));
    Ok(())
}

#[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
async fn should_reuse_answer_only_for_the_exact_question_version(pool: PgPool) -> TestResult {
    let user_id = create_user(&pool, "returning_participant").await?;
    let gender_v1 = questions::load_latest_published(&pool, GENDER_QUESTION_ID, LOCALE)
        .await?
        .ok_or("gender default should be published")?;
    assert_eq!(gender_v1.id, GENDER_VERSION_ID);

    responses::put_response(
        &pool,
        user_id,
        gender_v1.id,
        &ResponseInput {
            value: json!("Prefer to describe"),
            conversation_id: None,
        },
        LOCALE,
    )
    .await?;

    let mut conn = pool.acquire().await?;
    let reused = responses::reusable_response(&mut conn, user_id, &gender_v1)
        .await?
        .ok_or("answer to the same version should be reusable")?;
    assert_eq!(reused.value, json!("Prefer to describe"));
    drop(conn);

    questions::start_draft(&pool, None, GENDER_QUESTION_ID, LOCALE).await?;
    let republished = questions::publish_question(&pool, None, GENDER_QUESTION_ID, LOCALE).await?;
    let gender_v2 = republished
        .latest_published
        .ok_or("gender v2 should be published")?;
    assert_ne!(gender_v2.id, gender_v1.id);

    let mut conn = pool.acquire().await?;
    assert!(
        responses::reusable_response(&mut conn, user_id, &gender_v2)
            .await?
            .is_none()
    );
    Ok(())
}
