//! Tests for [`crate::models::invites`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::{
        bot_service::{ComhairleBotService, MockComhairleBotService},
        models::{
            conversation::{self, CreateConversation, PartialConversation},
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
            users,
            workflow::{self, CreateWorkflow},
        },
        routes::events::dto::EventDto,
        test_helpers::test_config,
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    use crate::models::invites::*;
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress};
    #[allow(unused_imports)]
    use ::std::collections::{HashMap, HashSet};
    #[allow(unused_imports)]
    use chrono::{DateTime, Utc};
    use fake::{Fake, Faker};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    use std::{error::Error, sync::Arc};
    #[allow(unused_imports)]
    use uuid::Uuid;

    #[test]
    fn invite_check_for_user_should_be_case_insensitive() -> Result<(), Box<dyn Error>> {
        let user = User {
            id: Uuid::new_v4(),
            username: Some("Name".into()),
            password: Some("some password".into()),
            avatar_url: Some("".into()),
            auth_type: users::UserAuthType::EmailPassword,
            email: Some("TestEmail@gmail.com".into()),
            guest_code: None,
            email_verified: false,
            organization_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            signup_ip: None,
            signup_user_agent: None,
        };

        let invite = Invite {
            id: Uuid::new_v4(),
            invite_type: InviteType::Email("testemail@gmail.com".into()),
            created_by: Some(Uuid::new_v4()),
            status: InviteStatus::Pending,
            expires_at: None,
            conversation_id: Uuid::new_v4(),
            event_id: None,
            workflow_id: None,
            workflow_step_id: None,
            login_behaviour: LoginBehaviour::Manual,
            tags: vec![],
            label: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            accept_count: 0,
        };

        assert!(
            invite.is_for_user(&user).is_ok(),
            "User should be identified even if their emails dont match"
        );
        Ok(())
    }
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_correct_stats_for_invite(db: PgPool) -> Result<(), Box<dyn Error>> {
        let user1 = users::create_user(&Faker.fake(), &db).await?;
        let user2 = users::create_user(&Faker.fake(), &db).await?;
        let user3 = users::create_user(&Faker.fake(), &db).await?;
        let user4 = users::create_user(&Faker.fake(), &db).await?;

        let bot_service = MockComhairleBotService::base();
        let bot_service: Arc<dyn ComhairleBotService> = Arc::new(bot_service);
        let config = test_config().unwrap();

        let conversation = crate::services::conversation::create(
            &db,
            &Some(bot_service),
            &config,
            &CreateConversation {
                is_public: true,
                is_invite_only: true,
                ..Faker.fake()
            },
            user1.id,
            None,
        )
        .await?;

        let workflow = workflow::create(
            &db,
            &CreateWorkflow {
                region_id: None,
                ..Faker.fake()
            },
            Some(conversation.id),
            None,
            user1.id,
        )
        .await?;

        let conversation = conversation::update(
            &db,
            &conversation.id,
            &PartialConversation {
                default_workflow_id: Some(workflow.id),
                ..Default::default()
            },
        )
        .await?;

        let invite = create(
            &db,
            CreateInviteDTO {
                invite_type: InviteType::Open,
                login_behaviour: LoginBehaviour::Manual,
                expires_at: None,
                label: None,
                event_id: None,
            },
            &conversation.id,
            Some(user1.id),
        )
        .await?;

        invite.accept(&db, &user2).await?;
        invite.reject(&db, &user3).await?;
        invite.accept(&db, &user4).await?;

        let stats = get_stats_for_invite(&db, &invite.id).await?;

        assert_eq!(
            stats,
            vec![DailyResponseStats {
                day: Utc::now()
                    .date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc(),
                accept: 2,
                reject: 1
            }],
            "should get the correct daily stats"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn open_invite_should_remain_open_when_rejected(
        db: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user1 = users::create_user(&Faker.fake(), &db).await?;
        let user2 = users::create_user(&Faker.fake(), &db).await?;

        let bot_service = MockComhairleBotService::base();
        let bot_service: Arc<dyn ComhairleBotService> = Arc::new(bot_service);
        let config = test_config().unwrap();

        let conversation = crate::services::conversation::create(
            &db,
            &Some(bot_service),
            &config,
            &CreateConversation {
                is_public: true,
                is_invite_only: true,
                ..Faker.fake()
            },
            user1.id,
            None,
        )
        .await?;

        let workflow = workflow::create(
            &db,
            &CreateWorkflow {
                region_id: None,
                ..Faker.fake()
            },
            Some(conversation.id),
            None,
            user1.id,
        )
        .await?;

        let conversation = conversation::update(
            &db,
            &conversation.id,
            &PartialConversation {
                default_workflow_id: Some(workflow.id),
                ..Default::default()
            },
        )
        .await?;

        let invite = create(
            &db,
            CreateInviteDTO {
                invite_type: InviteType::Open,
                login_behaviour: LoginBehaviour::Manual,
                expires_at: None,
                label: None,
                event_id: None,
            },
            &conversation.id,
            Some(user1.id),
        )
        .await?;

        assert_eq!(
            invite.status,
            InviteStatus::Open,
            "Invite should start as Open"
        );

        let rejected_invite = invite.reject(&db, &user2).await?;

        assert_eq!(
            rejected_invite.status,
            InviteStatus::Open,
            "Open invite should remain Open after rejection"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn pending_invite_should_be_marked_rejected_when_rejected(
        db: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user1 = users::create_user(&Faker.fake(), &db).await?;
        let user2 = users::create_user(&Faker.fake(), &db).await?;

        let bot_service = MockComhairleBotService::base();
        let bot_service: Arc<dyn ComhairleBotService> = Arc::new(bot_service);
        let config = test_config().unwrap();

        let conversation = crate::services::conversation::create(
            &db,
            &Some(bot_service),
            &config,
            &CreateConversation {
                is_public: true,
                is_invite_only: true,
                ..Faker.fake()
            },
            user1.id,
            None,
        )
        .await?;

        let workflow = workflow::create(
            &db,
            &CreateWorkflow {
                region_id: None,
                ..Faker.fake()
            },
            Some(conversation.id),
            None,
            user1.id,
        )
        .await?;

        let conversation = conversation::update(
            &db,
            &conversation.id,
            &PartialConversation {
                default_workflow_id: Some(workflow.id),
                ..Default::default()
            },
        )
        .await?;

        let invite = create(
            &db,
            CreateInviteDTO {
                invite_type: InviteType::Email(user2.email.clone().unwrap()),
                login_behaviour: LoginBehaviour::Manual,
                expires_at: None,
                label: None,
                event_id: None,
            },
            &conversation.id,
            Some(user1.id),
        )
        .await?;

        assert_eq!(
            invite.status,
            InviteStatus::Pending,
            "Email invite should start as Pending"
        );

        let rejected_invite = invite.reject(&db, &user2).await?;

        assert_eq!(
            rejected_invite.status,
            InviteStatus::Rejected,
            "Pending invite should be marked as Rejected after rejection"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_invites_for_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user1 = users::create_user(&Faker.fake(), &pool).await?;
        let user2 = users::create_user(&Faker.fake(), &pool).await?;
        let user3 = users::create_user(&Faker.fake(), &pool).await?;
        let user4 = users::create_user(&Faker.fake(), &pool).await?;

        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(value)?;

        create(
            &pool,
            CreateInviteDTO {
                invite_type: InviteType::Email(user2.email.clone().unwrap()),
                login_behaviour: LoginBehaviour::Manual,
                expires_at: None,
                label: None,
                event_id: Some(event.id),
            },
            &conversation_id,
            Some(user1.id),
        )
        .await?;
        create(
            &pool,
            CreateInviteDTO {
                invite_type: InviteType::Email(user3.email.clone().unwrap()),
                login_behaviour: LoginBehaviour::Manual,
                expires_at: None,
                label: None,
                event_id: Some(event.id),
            },
            &conversation_id,
            Some(user1.id),
        )
        .await?;
        create(
            &pool,
            CreateInviteDTO {
                invite_type: InviteType::Email(user4.email.clone().unwrap()),
                login_behaviour: LoginBehaviour::Manual,
                expires_at: None,
                label: None,
                event_id: Some(event.id),
            },
            &conversation_id,
            Some(user1.id),
        )
        .await?;

        let invites = list_for_event(&pool, &event.id).await?;

        assert!(
            invites.iter().any(|invite| invite.invite_type
                == InviteType::Email(user2.email.as_ref().unwrap().clone())),
            "missing user2"
        );
        assert!(
            invites.iter().any(|invite| invite.invite_type
                == InviteType::Email(user3.email.as_ref().unwrap().clone())),
            "missing user3"
        );
        assert!(
            invites.iter().any(|invite| invite.invite_type
                == InviteType::Email(user4.email.as_ref().unwrap().clone())),
            "missing user4"
        );

        Ok(())
    }
}
