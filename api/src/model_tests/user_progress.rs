//! Tests for [`crate::models::user_progress`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::{
        models::{
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
            users,
        },
        routes::{workflow_steps::dto::WorkflowStepDto, workflows::dto::WorkflowDto},
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    use crate::models::user_progress::*;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress};
    #[allow(unused_imports)]
    use ::std::collections::{HashMap, HashSet};
    #[allow(unused_imports)]
    use chrono::{DateTime, Utc};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    #[allow(unused_imports)]
    use uuid::Uuid;

    use crate::models::conversation::{self, PartialConversation};
    use axum::Router;
    use std::error::Error;

    /// A conversation with one workflow of `step_count` steps and one participant. The app and
    /// session are kept so a test can add more steps afterwards.
    struct Fixture {
        app: Router,
        session: crate::test_helpers::UserSession,
        conversation_id: Uuid,
        workflow: WorkflowDto,
        steps: Vec<WorkflowStepDto>,
        user: User,
    }

    impl Fixture {
        async fn new(pool: &PgPool, step_count: i32) -> Result<Self, Box<dyn Error>> {
            let (app, mut session) = setup_default_app_and_session(pool).await?;
            let conversation_id = get_random_conversation_id(&app, &mut session).await?;
            let (_, value, _) = session
                .create_random_workflow(&app, &conversation_id.to_string())
                .await?;
            let workflow: WorkflowDto = serde_json::from_value(value)?;

            let steps =
                add_steps(&app, &mut session, &conversation_id, &workflow, step_count).await?;
            let user = users::create_guest_user(pool).await?;

            Ok(Self {
                app,
                session,
                conversation_id,
                workflow,
                steps,
                user,
            })
        }

        /// Turn off `allow_revisit_after_finishing`, the only configuration under which
        /// finishing seals anyone.
        async fn disallow_revisits(&self, pool: &PgPool) -> Result<(), Box<dyn Error>> {
            conversation::update(
                pool,
                &self.conversation_id,
                &PartialConversation {
                    allow_revisit_after_finishing: Some(false),
                    ..Default::default()
                },
            )
            .await?;

            Ok(())
        }

        async fn add_steps(&mut self, count: i32) -> Result<Vec<WorkflowStepDto>, Box<dyn Error>> {
            add_steps(
                &self.app,
                &mut self.session,
                &self.conversation_id,
                &self.workflow,
                count,
            )
            .await
        }

        async fn is_sealed(&self, pool: &PgPool) -> Result<bool, Box<dyn Error>> {
            Ok(is_sealed(pool, &self.user.id, &self.workflow.id).await?)
        }
    }

    async fn add_steps(
        app: &Router,
        session: &mut crate::test_helpers::UserSession,
        conversation_id: &Uuid,
        workflow: &WorkflowDto,
        count: i32,
    ) -> Result<Vec<WorkflowStepDto>, Box<dyn Error>> {
        let values = session
            .create_random_workflow_steps(
                app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                count,
            )
            .await?;

        Ok(values
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<_, _>>()?)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_user_progress(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let f = Fixture::new(&pool, 2).await?;

        let user_progress = create(
            &pool,
            &f.user.id,
            &f.steps[0].id,
            ProgressStatus::InProgress,
        )
        .await?;

        assert_eq!(user_progress.user_id, f.user.id, "user_ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_user_progress(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let f = Fixture::new(&pool, 2).await?;

        let user_progress = create(
            &pool,
            &f.user.id,
            &f.steps[0].id,
            ProgressStatus::NotStarted,
        )
        .await?;

        assert_eq!(
            user_progress.status,
            ProgressStatus::NotStarted,
            "incorrect status before update"
        );
        assert!(
            user_progress.permission_to_share_with_organizers,
            "incorrect permission before update"
        );

        let update_params = UpdateUserProgress {
            status: Some(ProgressStatus::Done),
            permission_to_share_with_organizers: Some(false),
            ..Default::default()
        };

        let user_progress = update(&pool, &f.user.id, &f.steps[0].id, &update_params).await?;

        assert_eq!(
            user_progress.status,
            ProgressStatus::Done,
            "incorrect status after update"
        );
        assert!(
            !user_progress.permission_to_share_with_organizers,
            "incorrect permission after update"
        );

        Ok(())
    }

    /// Walks a participant through a two-step workflow and checks the seal at each stage,
    /// with `allow_revisit_after_finishing` off. The interesting assertion is the middle one:
    /// finishing *some* steps must not seal, or a participant gets locked out mid-flow.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_seal_a_participant_only_once_every_step_is_done(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let f = Fixture::new(&pool, 2).await?;
        f.disallow_revisits(&pool).await?;

        for step in &f.steps {
            create(&pool, &f.user.id, &step.id, ProgressStatus::NotStarted).await?;
        }

        assert!(
            !f.is_sealed(&pool).await?,
            "a participant who has done nothing must not be sealed"
        );

        let done = UpdateUserProgress {
            status: Some(ProgressStatus::Done),
            ..Default::default()
        };
        update(&pool, &f.user.id, &f.steps[0].id, &done).await?;

        assert!(
            !f.is_sealed(&pool).await?,
            "a participant part-way through must not be sealed"
        );

        update(&pool, &f.user.id, &f.steps[1].id, &done).await?;

        assert!(
            f.is_sealed(&pool).await?,
            "a participant who has finished every step must be sealed"
        );

        Ok(())
    }

    /// The default. Existing conversations must behave exactly as they did before the seal
    /// existed, which means finishing everything still seals nobody.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_never_seal_when_revisits_are_allowed(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let f = Fixture::new(&pool, 1).await?;

        create(&pool, &f.user.id, &f.steps[0].id, ProgressStatus::Done).await?;

        assert!(
            has_finished(&pool, &f.user.id, &f.workflow.id).await?,
            "every step is done, so they have finished"
        );
        assert!(
            !f.is_sealed(&pool).await?,
            "finishing must not seal while the conversation allows revisits"
        );

        Ok(())
    }

    /// A workflow with no steps must not read as finished. Otherwise "you completed all zero
    /// of them" would seal a participant out of a conversation before it had any content.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_treat_an_empty_workflow_as_finished(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let f = Fixture::new(&pool, 0).await?;

        assert!(
            !has_finished(&pool, &f.user.id, &f.workflow.id).await?,
            "an empty workflow must never count as finished"
        );

        Ok(())
    }

    /// A step added after a participant finished leaves them with no progress row for it. The
    /// left join has to read that as unfinished, which is what un-seals them (ADR-0016).
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_unseal_when_a_step_is_added_after_finishing(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut f = Fixture::new(&pool, 1).await?;
        f.disallow_revisits(&pool).await?;

        create(&pool, &f.user.id, &f.steps[0].id, ProgressStatus::Done).await?;
        assert!(
            f.is_sealed(&pool).await?,
            "sealed after finishing the only step"
        );

        f.add_steps(1).await?;

        assert!(
            !f.is_sealed(&pool).await?,
            "adding a step un-seals a participant who had already finished"
        );

        Ok(())
    }
}
