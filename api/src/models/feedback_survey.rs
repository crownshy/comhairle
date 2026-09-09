//! The feedback survey attached to a conversation's thank-you page.
//!
//! One HeyForm form per conversation, asked once the participant has finished the flow. It is
//! deliberately not a workflow step: it does not count towards progress, is never required,
//! and is about the experience rather than the topic. The tool config is the same
//! [`ToolConfig`] a HeyForm step stores, so the builder, embed and insights code all reuse.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::SqlxResultExt,
    tools::{ToolConfig, ToolConfigSanitize},
};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "conversation_feedback_survey")]
pub struct FeedbackSurvey {
    pub conversation_id: Uuid,
    pub tool_config: ToolConfig,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FeedbackSurvey {
    /// Strips the tool's admin credentials. Participants only need the survey id and server.
    pub fn sanitize(mut self) -> Self {
        self.tool_config = self.tool_config.sanitize();
        self
    }
}

const DEFAULT_COLUMNS: [FeedbackSurveyIden; 4] = [
    FeedbackSurveyIden::ConversationId,
    FeedbackSurveyIden::ToolConfig,
    FeedbackSurveyIden::CreatedAt,
    FeedbackSurveyIden::UpdatedAt,
];

#[instrument(err(Debug), skip(db))]
pub async fn get_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Option<FeedbackSurvey>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(FeedbackSurveyIden::Table)
        .and_where(Expr::col(FeedbackSurveyIden::ConversationId).eq(*conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, FeedbackSurvey, _>(&sql, values)
        .fetch_optional(db)
        .await
        .resolve_db_err("Feedback survey")
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    conversation_id: &Uuid,
    tool_config: ToolConfig,
) -> Result<FeedbackSurvey, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(FeedbackSurveyIden::Table)
        .columns([
            FeedbackSurveyIden::ConversationId,
            FeedbackSurveyIden::ToolConfig,
        ])
        .values([(*conversation_id).into(), tool_config.into()])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, FeedbackSurvey, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Feedback survey")
}

/// One row per participant who finished the survey. Keyed by both ids, so a repeat is a no-op.
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "conversation_feedback_survey_completion")]
pub struct FeedbackSurveyCompletion {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub completed_at: DateTime<Utc>,
}

#[instrument(err(Debug), skip(db))]
pub async fn mark_completed(
    db: &PgPool,
    conversation_id: &Uuid,
    user_id: &Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(FeedbackSurveyCompletionIden::Table)
        .columns([
            FeedbackSurveyCompletionIden::ConversationId,
            FeedbackSurveyCompletionIden::UserId,
        ])
        .values([(*conversation_id).into(), (*user_id).into()])
        .unwrap()
        .on_conflict(
            OnConflict::columns([
                FeedbackSurveyCompletionIden::ConversationId,
                FeedbackSurveyCompletionIden::UserId,
            ])
            .do_nothing()
            .to_owned(),
        )
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values)
        .execute(db)
        .await
        .resolve_db_err("Feedback survey completion")?;

    Ok(())
}

#[instrument(err(Debug), skip(db))]
pub async fn is_completed(
    db: &PgPool,
    conversation_id: &Uuid,
    user_id: &Uuid,
) -> Result<bool, ComhairleError> {
    let (sql, values) = Query::select()
        .column(FeedbackSurveyCompletionIden::UserId)
        .from(FeedbackSurveyCompletionIden::Table)
        .and_where(Expr::col(FeedbackSurveyCompletionIden::ConversationId).eq(*conversation_id))
        .and_where(Expr::col(FeedbackSurveyCompletionIden::UserId).eq(*user_id))
        .build_sqlx(PostgresQueryBuilder);

    let row: Option<(Uuid,)> = sqlx::query_as_with(&sql, values)
        .fetch_optional(db)
        .await
        .resolve_db_err("Feedback survey completion")?;

    Ok(row.is_some())
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, conversation_id: &Uuid) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(FeedbackSurveyIden::Table)
        .and_where(Expr::col(FeedbackSurveyIden::ConversationId).eq(*conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values)
        .execute(db)
        .await
        .resolve_db_err("Feedback survey")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::{
        models::model_test_helpers::{
            get_random_conversation_id, get_random_user_id, setup_default_app_and_session,
        },
        tools::heyform::HeyFormToolConfig,
    };

    fn survey_config() -> ToolConfig {
        ToolConfig::HeyForm(HeyFormToolConfig {
            survey_id: "form-1".into(),
            survey_url: "forms.example.com/form/form-1".into(),
            admin_user: "forms-admin@example.com".into(),
            admin_password: "secret".into(),
            workspace_id: "ws-1".into(),
            project_id: "proj-1".into(),
            server_url: "forms.example.com".into(),
        })
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn round_trips_the_tool_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        assert!(
            get_for_conversation(&pool, &conversation_id)
                .await?
                .is_none()
        );

        let created = create(&pool, &conversation_id, survey_config()).await?;
        assert_eq!(created.conversation_id, conversation_id);
        assert_eq!(created.tool_config, survey_config());

        let fetched = get_for_conversation(&pool, &conversation_id)
            .await?
            .expect("survey was created");
        assert_eq!(fetched.tool_config, survey_config());

        delete(&pool, &conversation_id).await?;
        assert!(
            get_for_conversation(&pool, &conversation_id)
                .await?
                .is_none()
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn a_conversation_has_at_most_one_survey(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        create(&pool, &conversation_id, survey_config()).await?;
        let second = create(&pool, &conversation_id, survey_config()).await;
        assert!(second.is_err());
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn completion_is_per_user_and_idempotent(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;
        create(&pool, &conversation_id, survey_config()).await?;

        assert!(!is_completed(&pool, &conversation_id, &user_id).await?);

        mark_completed(&pool, &conversation_id, &user_id).await?;
        mark_completed(&pool, &conversation_id, &user_id).await?;
        assert!(is_completed(&pool, &conversation_id, &user_id).await?);
        assert!(!is_completed(&pool, &conversation_id, &Uuid::new_v4()).await?);

        // Removing the survey takes its completions with it.
        delete(&pool, &conversation_id).await?;
        assert!(!is_completed(&pool, &conversation_id, &user_id).await?);
        Ok(())
    }

    #[test]
    fn sanitize_strips_the_credentials() {
        let survey = FeedbackSurvey {
            conversation_id: Uuid::nil(),
            tool_config: survey_config(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let ToolConfig::HeyForm(config) = survey.sanitize().tool_config else {
            panic!("config type changed");
        };
        assert_eq!(config.survey_id, "form-1");
        assert_eq!(config.admin_user, "");
        assert_eq!(config.admin_password, "");
    }
}
