//! Tests for [`crate::models::event_attendance`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::{
        event::{self, CreateEvent, SignupMode},
        model_test_helpers::{
            get_random_conversation_id, get_random_user_id, setup_default_app_and_session,
        },
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    use crate::models::event_attendance::*;
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
    use crate::models::{proposal_section, user_progress, users};
    #[allow(unused_imports)]
    use ::std::collections::{HashMap, HashSet};
    #[allow(unused_imports)]
    use chrono::{DateTime, Utc};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    use std::error::Error;
    #[allow(unused_imports)]
    use uuid::Uuid;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_attendance_for_event_without_capacity(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        assert_eq!(attendance.event_id, new_event.id, "incorrect event_id");
        assert_eq!(attendance.user_id, user_id, "incorrect user_id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_attendance_for_event_with_capacity(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            capacity: Some(10),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        assert_eq!(attendance.event_id, new_event.id, "incorrect event_id");
        assert_eq!(attendance.user_id, user_id, "incorrect user_id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_only_check_capacity_against_participant_attendees(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;
        let user_id_4 = get_random_user_id(&app, &mut session).await?;
        let user_id_5 = get_random_user_id(&app, &mut session).await?;
        let user_id_6 = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            capacity: Some(3),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_3,
            role: "participant".to_string(),
        };
        let create_attendance_4 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_4,
            role: "facilitator".to_string(),
        };
        let create_attendance_5 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_5,
            role: "participant".to_string(),
        };
        let create_attendance_6 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_6,
            role: "something_different".to_string(),
        };
        let _ = create(&pool, &create_attendance_1).await?;
        let _ = create(&pool, &create_attendance_2).await?;
        let _ = create(&pool, &create_attendance_3).await?;
        let attendance_4 = create(&pool, &create_attendance_4).await?;
        let err = create(&pool, &create_attendance_5).await.unwrap_err();
        let attendance_6 = create(&pool, &create_attendance_6).await?;

        assert_eq!(
            attendance_4.event_id, new_event.id,
            "facilitator attendance not successful"
        );
        match err {
            ModelError::Event(EventError::EventAtCapacity) => (),
            _ => panic!("incorrect error type"),
        };
        assert_eq!(
            attendance_6.event_id, new_event.id,
            "post error attendance not successful"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn user_cannot_attend_same_event_twice(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let _ = create(&pool, &create_attendance).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let err = create(&pool, &create_attendance).await.unwrap_err();

        match err {
            ModelError::Event(EventError::UserAlreadyRegisteredForEvent(message)) => {
                assert!(
                    message.contains(&new_event.id.to_string()),
                    "missing event_id"
                );
            }
            _ => panic!("Expected UserAlreadyRegisteredForEvent"),
        };

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn transaction_will_not_lock_on_failure(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;

        let create_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            capacity: Some(1),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let create_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            capacity: Some(1),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event_1 = event::create(&pool, &conversation_id, &create_event_1).await?;
        let new_event_2 = event::create(&pool, &conversation_id, &create_event_2).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: new_event_1.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: new_event_1.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: new_event_2.id,
            user_id: user_id_1,
            role: "facilitator".to_string(),
        };

        let _ = create(&pool, &create_attendance_1).await?;
        // Fails because event is at capacity
        let _ = create(&pool, &create_attendance_2).await.unwrap_err();
        // Create attendance for different event to check lock is released
        let attendance = create(&pool, &create_attendance_3).await?;

        assert_eq!(attendance.role, "facilitator".to_string(), "incorrect role");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        let update_attendance = UpdateEventAttendance {
            role: Some("facilitator".to_string()),
        };
        let updated_attendance = update(&pool, &attendance.id, &update_attendance).await?;

        assert_eq!(updated_attendance.id, attendance.id, "ids do not match");
        assert_eq!(
            updated_attendance.role,
            "facilitator".to_string(),
            "role was not updated"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_attendance_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        let get_attendance = get_by_id(&pool, &attendance.id).await?;

        assert_eq!(get_attendance.id, attendance.id, "ids do not match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_attendance_by_user_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;

        let get_attendance = get_by_event_and_user(&pool, &new_event.id, &user_id).await?;

        assert_eq!(get_attendance.id, attendance.id, "ids do not match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_3,
            role: "facilitator".to_string(),
        };
        let _ = create(&pool, &create_attendance_1).await?;
        let _ = create(&pool, &create_attendance_2).await?;
        let _ = create(&pool, &create_attendance_3).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = EventAttendanceFilterOptions { role: None };
        let order_options = EventAttendanceOrderOptions { created_at: None };
        let results = list(
            &pool,
            new_event.id,
            page_options,
            filter_options,
            order_options,
        )
        .await?;

        assert_eq!(results.total, 3, "incorrect total");
        assert_eq!(
            results.records[2].role,
            "facilitator".to_string(),
            "incorrect role type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_attendance_by_role(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;
        let user_id_4 = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_3,
            role: "facilitator".to_string(),
        };
        let create_attendance_4 = CreateEventAttendance {
            event_id: new_event.id,
            user_id: user_id_4,
            role: "facilitator".to_string(),
        };
        let _ = create(&pool, &create_attendance_1).await?;
        let _ = create(&pool, &create_attendance_2).await?;
        let _ = create(&pool, &create_attendance_3).await?;
        let _ = create(&pool, &create_attendance_4).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = EventAttendanceFilterOptions {
            role: Some("facilitator".to_string()),
        };
        let order_options = EventAttendanceOrderOptions { created_at: None };
        let results = list(
            &pool,
            new_event.id,
            page_options,
            filter_options,
            order_options,
        )
        .await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert!(
            results.records.iter().all(|e| e.role == "facilitator"),
            "not all attendances have facilitator role"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id = get_random_user_id(&app, &mut session).await?;

        let create_event = CreateEvent {
            name: "test_event".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event = event::create(&pool, &conversation_id, &create_event).await?;

        let create_attendance = CreateEventAttendance {
            event_id: new_event.id,
            user_id,
            role: "participant".to_string(),
        };
        let attendance = create(&pool, &create_attendance).await?;
        let _ = delete(&pool, &attendance.id).await?;

        let err = get_by_id(&pool, &attendance.id).await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(e, "Event Attendance".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
