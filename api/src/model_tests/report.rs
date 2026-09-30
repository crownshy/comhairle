//! Tests for [`crate::models::report`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::report::*;

    use std::error::Error;

    use sqlx::PgPool;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };
    use crate::models::translations::get_text_translation_by_content_and_locale;
    use crate::routes::workflows::dto::WorkflowDto;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_report_for_conversation_with_translation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(value)?;
        session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                2,
            )
            .await?;

        let report = create_for_conversation(&pool, conversation_id, "en").await?;

        let summary_translation =
            get_text_translation_by_content_and_locale(&pool, &report.summary, "en").await?;
        let body_translation =
            get_text_translation_by_content_and_locale(&pool, &report.body.unwrap(), "en").await?;

        assert_eq!(
            summary_translation.content, "Summary to be filled out by facilitator",
            "incorrect summary translation text"
        );
        assert_eq!(
            body_translation.content, "Body to be filled out by facilitator",
            "incorrect summary translation text"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_localized_report(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(value)?;
        session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                2,
            )
            .await?;

        create_for_conversation(&pool, conversation_id, "en").await?;

        let report = get_localized_for_conversation(&pool, conversation_id, "en").await?;

        assert_eq!(
            report.summary, "Summary to be filled out by facilitator",
            "incorrect summary translation text"
        );
        assert_eq!(
            report.body,
            Some("Body to be filled out by facilitator".to_string()),
            "incorrect body translation text"
        );

        Ok(())
    }
}
