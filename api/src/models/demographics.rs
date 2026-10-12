//! Demographic questions and forms.
//!
//! Questions and forms are versioned by UUID. Published versions are immutable and
//! responses point at a question version, so an answer always records exactly what was
//! asked. See CONTEXT.md (Demographics) and ADRs 0051-0054.

pub mod defaults;
pub mod forms;
pub mod questions;
pub mod report;
pub mod responses;
pub mod steps;
pub mod types;
pub mod validation;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The type of a demographic question, indicating the kind of response it expects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    Number,
    SingleSelect,
    MultiSelect,
    Date,
    AnyString,
    Location,
    Custom,
}

impl QuestionType {
    pub fn is_select(self) -> bool {
        matches!(self, Self::SingleSelect | Self::MultiSelect)
    }
}

/// The precision of a date question, indicating the level of detail required in the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    Day,
    Month,
    Year,
}

/// The format of a location question, indicating how the location should be specified in the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LocationFormat {
    Postcode,
    LatLong,
    Country,
}

/// The status of a demographics form version, indicating whether it is a draft or published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VersionStatus {
    Draft,
    Published,
}

macro_rules! impl_sea_query_text_value {
    ($($ty:ty { $($variant:ident => $text:literal),+ $(,)? }),+ $(,)?) => {$(
        impl From<$ty> for sea_query::Value {
            fn from(value: $ty) -> Self {
                let text = match value {
                    $(<$ty>::$variant => $text,)+
                };
                sea_query::Value::String(Some(Box::new(text.to_owned())))
            }
        }

        impl sea_query::Nullable for $ty {
            fn null() -> sea_query::Value {
                sea_query::Value::String(None)
            }
        }
    )+};
}

impl_sea_query_text_value! {
    QuestionType {
        Number => "number",
        SingleSelect => "single_select",
        MultiSelect => "multi_select",
        Date => "date",
        AnyString => "any_string",
        Location => "location",
        Custom => "custom",
    },
    DatePrecision {
        Day => "day",
        Month => "month",
        Year => "year",
    },
    LocationFormat {
        Postcode => "postcode",
        LatLong => "lat_long",
        Country => "country",
    },
    VersionStatus {
        Draft => "draft",
        Published => "published",
    },
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;

    use serde_json::{Value, json};
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::error::ComhairleError;
    use crate::models::organization::{self, CreateOrganization, OrganizationType};
    use crate::models::translations::{TextFormat, new_translation};
    use crate::routes::auth::SignupRequest;
    use crate::test_helpers::test_state;
    use crate::tools::demographics::{
        DemographicFormTool, DemographicFormToolConfig, DemographicQuestionTool,
        DemographicQuestionToolConfig, VersionPolicy,
    };
    use crate::tools::{ToolConfig, ToolImpl};

    use super::VersionStatus;
    use super::defaults::{ETHNICITY_VERSION_ID, GENDER_VERSION_ID};
    use super::forms;
    use super::questions;
    use super::report;
    use super::responses::{self, ConsentState};
    use super::steps;
    use super::types::{
        DemographicFormDraftInput, QuestionDraftInput, QuestionVersionDto, ResponseInput,
    };
    use super::validation::validate_answer;

    const LOCALE: &str = "en";
    const AGE_QUESTION_ID: Uuid = Uuid::from_u128(0xde000001_0000_4000_8000_000000000001);
    const ETHNICITY_QUESTION_ID: Uuid = Uuid::from_u128(0xde000001_0000_4000_8000_000000000002);
    const GENDER_QUESTION_ID: Uuid = Uuid::from_u128(0xde000001_0000_4000_8000_000000000003);

    type TestResult = Result<(), Box<dyn Error>>;

    fn question_input(value: Value) -> QuestionDraftInput {
        serde_json::from_value(value).expect("question input should deserialize")
    }

    fn form_input(value: Value) -> DemographicFormDraftInput {
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
        let published =
            questions::publish_question(pool, Some(org_id), created.question.id, LOCALE).await?;
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
        assert_eq!(
            created.draft.as_ref().map(|d| d.version.version_number),
            Some(1)
        );
        let question_id = created.question.id;

        questions::publish_question(&pool, Some(org_id), question_id, LOCALE).await?;
        questions::start_draft(&pool, Some(org_id), question_id, LOCALE).await?;
        questions::update_draft(
            &pool,
            Some(org_id),
            question_id,
            &question_input(json!({
                "text": "Favourite colour (edited)",
                "question_type": "any_string",
                "tag_ids": [tag_id],
            })),
            LOCALE,
        )
        .await?;
        let second = questions::publish_question(&pool, Some(org_id), question_id, LOCALE).await?;
        let latest = second.latest_published.ok_or("v2 should be published")?;
        assert_eq!(latest.version.version_number, 2);
        assert_eq!(latest.text, "Favourite colour (edited)");

        questions::start_draft(&pool, Some(org_id), question_id, LOCALE).await?;
        let third = questions::publish_question(&pool, Some(org_id), question_id, LOCALE).await?;
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
        let question_id = version.version.question_id;

        questions::start_draft(&pool, Some(org_id), question_id, LOCALE).await?;
        questions::discard_draft(&pool, Some(org_id), question_id).await?;

        let question = questions::get_question(&pool, Some(org_id), question_id, LOCALE).await?;
        assert!(question.draft.is_none());
        let latest = question
            .latest_published
            .ok_or("published version should remain")?;
        assert_eq!(latest.version.id, version.version.id);
        assert_eq!(latest.text, "Original");
        assert_eq!(question.versions.len(), 1);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_let_organizations_write_system_default_questions(
        pool: PgPool,
    ) -> TestResult {
        let org_id = create_org(&pool).await?;

        let start = questions::start_draft(&pool, Some(org_id), AGE_QUESTION_ID, LOCALE).await;
        assert!(matches!(start, Err(ComhairleError::ResourceNotFound(_))));
        let archive =
            questions::archive_question(&pool, Some(org_id), AGE_QUESTION_ID, LOCALE).await;
        assert!(matches!(archive, Err(ComhairleError::ResourceNotFound(_))));

        let visible = questions::list_questions(&pool, Some(org_id), LOCALE).await?;
        assert!(visible.iter().any(|q| q.question.id == AGE_QUESTION_ID));
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
        let question_id = v1.version.question_id;

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
                "entries": [{"question_id": unpublished.question.id, "position": 1, "required": true}],
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

        let published = forms::publish_form(&pool, org_id, form.form.id, LOCALE).await?;
        let pinned = published
            .latest_published
            .ok_or("form should be published")?;
        assert_eq!(
            pinned.entries[0].entry.question_version_id,
            Some(v1.version.id)
        );
        assert_eq!(pinned.entries[0].question_text, "Original");

        questions::publish_question(&pool, Some(org_id), question_id, LOCALE).await?;
        let after = forms::get_form(&pool, org_id, form.form.id, LOCALE).await?;
        let still_pinned = after
            .latest_published
            .ok_or("published form should stay published")?;
        assert_eq!(
            still_pinned.entries[0].entry.question_version_id,
            Some(v1.version.id)
        );
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
        let unlisted_other = json!({"option_ids": [option_ids[0]], "other_text": "Retired", "prefer_not_to_say": false});
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
        assert_eq!(gender_v1.version.id, GENDER_VERSION_ID);

        responses::put_response(
            &pool,
            user_id,
            gender_v1.version.id,
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
        let republished =
            questions::publish_question(&pool, None, GENDER_QUESTION_ID, LOCALE).await?;
        let gender_v2 = republished
            .latest_published
            .ok_or("gender v2 should be published")?;
        assert_ne!(gender_v2.version.id, gender_v1.version.id);

        let mut conn = pool.acquire().await?;
        assert!(
            responses::reusable_response(&mut conn, user_id, &gender_v2)
                .await?
                .is_none()
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_keep_submitted_answers_and_only_allow_anonymising(pool: PgPool) -> TestResult {
        let user_id = create_user(&pool, "unchanging_participant").await?;
        let answer = |value: Value| ResponseInput {
            value,
            conversation_id: None,
        };

        let saved = responses::put_response(
            &pool,
            user_id,
            GENDER_VERSION_ID,
            &answer(json!("Woman")),
            LOCALE,
        )
        .await?;
        let repeated = responses::put_response(
            &pool,
            user_id,
            GENDER_VERSION_ID,
            &answer(json!("Woman")),
            LOCALE,
        )
        .await?;
        assert_eq!(repeated.id, saved.id);

        let changed = responses::put_response(
            &pool,
            user_id,
            GENDER_VERSION_ID,
            &answer(json!("Man")),
            LOCALE,
        )
        .await;
        assert!(matches!(changed, Err(ComhairleError::Conflict(_))));

        let rewrite = sqlx::query("UPDATE demographics_response SET value = $1 WHERE id = $2")
            .bind(json!("Man"))
            .bind(saved.id)
            .execute(&pool)
            .await;
        assert!(rewrite.is_err(), "the database should refuse edits");

        let stranger = create_user(&pool, "other_participant").await?;
        let refused = responses::anonymise_response(&pool, stranger, saved.id).await;
        assert!(matches!(refused, Err(ComhairleError::ResourceNotFound(_))));

        responses::anonymise_response(&pool, user_id, saved.id).await?;
        assert!(
            responses::my_answers(&pool, user_id, LOCALE)
                .await?
                .is_empty()
        );
        let kept: Value =
            sqlx::query_scalar("SELECT value FROM demographics_response WHERE id = $1")
                .bind(saved.id)
                .fetch_one(&pool)
                .await?;
        assert_eq!(kept, json!("Woman"));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_pin_latest_versions_when_a_step_goes_live(pool: PgPool) -> TestResult {
        let org_id = create_org(&pool).await?;
        let v1 = publish_new_question(
            &pool,
            org_id,
            json!({"text": "Occupation", "question_type": "any_string"}),
        )
        .await?;
        let question_id = v1.version.question_id;
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
        let form_v1 = forms::publish_form(&pool, org_id, form.form.id, LOCALE)
            .await?
            .latest_published
            .ok_or("form should be published")?;

        let state = Arc::new(test_state().db(pool.clone()).call()?);
        let question_config = DemographicQuestionToolConfig {
            question_id,
            version: VersionPolicy::Latest,
        };
        let form_config = DemographicFormToolConfig {
            form_id: form.form.id,
            version: VersionPolicy::Latest,
            questions: vec![],
        };
        let live_question = DemographicQuestionTool::clone_tool(&question_config, &state).await?;
        let live_form = DemographicFormTool::clone_tool(&form_config, &state).await?;
        assert_eq!(
            live_question.version,
            VersionPolicy::Pinned {
                version_id: v1.version.id
            }
        );
        assert_eq!(
            live_form.version,
            VersionPolicy::Pinned {
                version_id: form_v1.version.id
            }
        );

        questions::start_draft(&pool, Some(org_id), question_id, LOCALE).await?;
        questions::publish_question(&pool, Some(org_id), question_id, LOCALE).await?;
        forms::start_form_draft(&pool, org_id, form.form.id, LOCALE).await?;
        forms::publish_form(&pool, org_id, form.form.id, LOCALE).await?;

        let preview = ToolConfig::DemographicQuestion(question_config);
        assert_ne!(
            steps::freeze_tool_config(&pool, preview).await?,
            ToolConfig::DemographicQuestion(live_question),
            "a later freeze picks up the newer version, but the live step keeps its own"
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_report_form_questions_minus_disabled_overrides(pool: PgPool) -> TestResult {
        let org_id = create_org(&pool).await?;
        let asked = publish_new_question(
            &pool,
            org_id,
            json!({"text": "Asked", "question_type": "any_string"}),
        )
        .await?;
        let disabled = publish_new_question(
            &pool,
            org_id,
            json!({"text": "Disabled", "question_type": "any_string"}),
        )
        .await?;
        let form = forms::create_form(
            &pool,
            org_id,
            &form_input(json!({
                "name": "Survey",
                "entries": [
                    {"question_id": asked.version.question_id, "position": 1, "required": true},
                    {"question_id": disabled.version.question_id, "position": 2, "required": true},
                ],
            })),
            LOCALE,
        )
        .await?;
        forms::publish_form(&pool, org_id, form.form.id, LOCALE).await?;

        let workflow_id: Uuid =
            sqlx::query_scalar("INSERT INTO workflow (name) VALUES ('Report') RETURNING id")
                .fetch_one(&pool)
                .await?;
        let name = new_translation(&pool, LOCALE, "Form", TextFormat::Plain).await?;
        let description = new_translation(&pool, LOCALE, "Form", TextFormat::Rich).await?;
        sqlx::query(
            "INSERT INTO workflow_step (workflow_id, name, description, step_order, tool_config)
            VALUES ($1, $2, $3, 1, $4)",
        )
        .bind(workflow_id)
        .bind(name.id)
        .bind(description.id)
        .bind(json!({
            "type": "demographicform",
            "form_id": form.form.id,
            "version": {"type": "latest"},
            "questions": [{
                "question_id": disabled.version.question_id,
                "enabled": false,
                "required": false,
            }],
        }))
        .execute(&pool)
        .await?;

        let user_id = create_user(&pool, "reporting_participant").await?;
        let conversation_id: Uuid = sqlx::query_scalar(
            "INSERT INTO conversation (title, short_description, description, owner_id)
             VALUES ($1, $1, $1, $2) RETURNING id",
        )
        .bind(description.id)
        .bind(user_id)
        .fetch_one(&pool)
        .await?;
        sqlx::query("UPDATE workflow SET conversation_id = $1 WHERE id = $2")
            .bind(conversation_id)
            .bind(workflow_id)
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO user_profile (user_id, consented) VALUES ($1, true)")
            .bind(user_id)
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO user_participation (user_id, workflow_id) VALUES ($1, $2)")
            .bind(user_id)
            .bind(workflow_id)
            .execute(&pool)
            .await?;
        for version in [&asked, &disabled] {
            responses::put_response(
                &pool,
                user_id,
                version.version.id,
                &ResponseInput {
                    value: json!("answer"),
                    conversation_id: Some(conversation_id),
                },
                LOCALE,
            )
            .await?;
        }

        let report = report::for_workflow(&pool, &workflow_id, LOCALE).await?;
        assert_eq!(report.total_participants, 1);
        assert_eq!(
            report.questions.keys().collect::<Vec<_>>(),
            vec![&asked.version.question_id]
        );
        Ok(())
    }
}
