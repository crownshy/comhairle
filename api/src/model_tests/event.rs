//! Tests for [`crate::models::event`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use chrono::Duration;

    use crate::models::{
        event_attendance::{self, CreateEventAttendance},
        model_test_helpers::{
            get_random_conversation_id, get_random_user_id, setup_default_app_and_session,
        },
    };

    #[allow(unused_imports)]
    use crate::models::error::{DataError, EventError, ModelError, ValidationError};
    use crate::models::event::*;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    #[allow(unused_imports)]
    use ::std::collections::{HashMap, HashSet};
    #[allow(unused_imports)]
    use chrono::{DateTime, Utc};
    #[allow(unused_imports)]
    use sqlx::PgPool;
    use std::error::Error;
    #[allow(unused_imports)]
    use uuid::Uuid;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_and_return_new_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            ..Default::default()
        };

        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(event.capacity, Some(10), "incorrect capacity");
        assert_eq!(event.conversation_id, conversation_id, "incorrect capacity");
        assert!(event.start_time < Utc::now(), "start time not past");
        assert_eq!(
            event.video_meeting_id.unwrap().get_version().unwrap(),
            uuid::Version::Random,
            "invalid video_meeting_id on creation"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_event_with_location(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            location: Some(EventLocation {
                venue_name: "Test venue".to_string(),
                address_line_1: "123 Main Street".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(
            event.location.as_ref().unwrap().venue_name,
            "Test venue".to_string(),
            "incorrect venue"
        );
        assert_eq!(
            event.location.as_ref().unwrap().address_line_1,
            "123 Main Street".to_string(),
            "incorrect address"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_event_data(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            ..Default::default()
        };
        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(
            event.capacity,
            Some(10),
            "incorrect capacity after creation"
        );
        assert_eq!(
            event.signup_mode,
            SignupMode::Invite,
            "incorrect signup_mode after creation"
        );

        let update_event = PartialEvent {
            capacity: Some(20),
            signup_mode: Some(SignupMode::Open),
            ..Default::default()
        };
        let event = update(&pool, &event.id, &update_event).await?;

        assert_eq!(event.capacity, Some(20), "incorrect capacity after update");
        assert_eq!(
            event.signup_mode,
            SignupMode::Open,
            "incorrect signup_mode after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_event_location_data(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            location: Some(EventLocation {
                venue_name: "Test venue".to_string(),
                address_line_1: "123 Main Street".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let event = create(&pool, &conversation_id, &new_event).await?;

        assert_eq!(
            event.location.unwrap().venue_name,
            "Test venue".to_string(),
            "incorrect venue before update"
        );

        let params = PartialEvent {
            location: Some(EventLocation {
                venue_name: "A change of venue".to_string(),
                address_line_1: "123 Main Street".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let event = update(&pool, &event.id, &params).await?;

        assert_eq!(
            event.location.unwrap().venue_name,
            "A change of venue".to_string(),
            "incorrect venue after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_event_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id_1 = get_random_conversation_id(&app, &mut session).await?;
        let conversation_id_2 = get_random_conversation_id(&app, &mut session).await?;

        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            ..Default::default()
        };
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            ..Default::default()
        };
        let event_1 = create(&pool, &conversation_id_1, &new_event_1).await?;
        let event_2 = create(&pool, &conversation_id_2, &new_event_2).await?;

        let get_event_1 = get_localized_by_id(&pool, &event_1.id, "en").await?;
        let get_event_2 = get_localized_by_id(&pool, &event_2.id, "en").await?;

        assert_eq!(get_event_1.id, event_1.id, "incorrect id for event 1");
        assert_eq!(get_event_2.id, event_2.id, "incorrect id for event 2");
        assert_eq!(
            get_event_1.name,
            "test_event_1".to_string(),
            "incorrect name for event 1"
        );
        assert_eq!(
            get_event_2.name,
            "test_event_2".to_string(),
            "incorrect name for event 2"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_event_with_current_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            ..Default::default()
        };
        let event = create(&pool, &conversation_id, &new_event).await?;

        let create_attendance_1 = CreateEventAttendance {
            event_id: event.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let create_attendance_2 = CreateEventAttendance {
            event_id: event.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let create_attendance_3 = CreateEventAttendance {
            event_id: event.id,
            user_id: user_id_3,
            role: "participant".to_string(),
        };
        let _ = event_attendance::create(&pool, &create_attendance_1).await?;
        let _ = event_attendance::create(&pool, &create_attendance_2).await?;
        let _ = event_attendance::create(&pool, &create_attendance_3).await?;

        let get_event = get_localized_by_id(&pool, &event.id, "en").await?;

        assert_eq!(
            get_event.name,
            "test_event".to_string(),
            "incorrect name for event"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_events(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event_3 = CreateEvent {
            name: "test_event_3".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let new_event_4 = CreateEvent {
            name: "test_event_4".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let _ = create(&pool, &conversation_id, &new_event_1).await?;
        let _ = create(&pool, &conversation_id, &new_event_2).await?;
        let _ = create(&pool, &conversation_id, &new_event_3).await?;
        let _ = create(&pool, &conversation_id, &new_event_4).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = EventFilterOptions {
            ..Default::default()
        };
        let order_options = EventOrderOptions {
            ..Default::default()
        };
        let results = list(
            &pool,
            &conversation_id,
            page_options,
            filter_options,
            order_options,
            None,
        )
        .await?;

        assert_eq!(results.total, 4, "incorrect number of events");
        assert_eq!(
            results.records[2].event.name,
            "test_event_3".to_string(),
            "incorrect event name"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_events_by_time_status(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            signup_mode: SignupMode::Invite,
            start_time: Utc::now() + Duration::days(1),
            ..Default::default()
        };
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            signup_mode: SignupMode::Invite,
            start_time: Utc::now() + Duration::days(2),
            ..Default::default()
        };
        let new_event_3 = CreateEvent {
            name: "test_event_3".to_string(),
            signup_mode: SignupMode::Invite,
            start_time: Utc::now() + Duration::days(3),
            ..Default::default()
        };
        let new_event_4 = CreateEvent {
            name: "test_event_4".to_string(),
            signup_mode: SignupMode::Invite,
            start_time: Utc::now() - Duration::days(3),
            ..Default::default()
        };
        let _ = create(&pool, &conversation_id, &new_event_1).await?;
        let _ = create(&pool, &conversation_id, &new_event_2).await?;
        let _ = create(&pool, &conversation_id, &new_event_3).await?;
        let _ = create(&pool, &conversation_id, &new_event_4).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let future_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                time_status: Some(TimeStatus::Future),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;
        let past_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                time_status: Some(TimeStatus::Past),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;

        assert_eq!(future_results.total, 3, "incorrect number of past events");
        assert_eq!(
            future_results.records[1].event.name,
            "test_event_2".to_string(),
            "incorrect future event name"
        );
        assert_eq!(past_results.total, 1, "incorrect number of past events");
        assert_eq!(
            past_results.records[0].event.name,
            "test_event_4".to_string(),
            "incorrect past event name"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_events_by_capacity(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let user_id_1 = get_random_user_id(&app, &mut session).await?;
        let user_id_2 = get_random_user_id(&app, &mut session).await?;
        let user_id_3 = get_random_user_id(&app, &mut session).await?;

        // Full: will add one attendee
        let new_event_1 = CreateEvent {
            name: "test_event_1".to_string(),
            capacity: Some(1),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        // Full: will add three attendees
        let new_event_2 = CreateEvent {
            name: "test_event_2".to_string(),
            capacity: Some(3),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        // Available: has capacity but will add no attendees
        let new_event_3 = CreateEvent {
            name: "test_event_3".to_string(),
            capacity: Some(1),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        // Available: capacity null so always has availability
        let new_event_4 = CreateEvent {
            name: "test_event_4".to_string(),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        // Full: will add 2 attendees
        let new_event_5 = CreateEvent {
            name: "test_event_5".to_string(),
            capacity: Some(2),
            signup_mode: SignupMode::Invite,
            ..Default::default()
        };
        let event_1 = create(&pool, &conversation_id, &new_event_1).await?;
        let event_2 = create(&pool, &conversation_id, &new_event_2).await?;
        let _ = create(&pool, &conversation_id, &new_event_3).await?;
        let _ = create(&pool, &conversation_id, &new_event_4).await?;
        let event_5 = create(&pool, &conversation_id, &new_event_5).await?;

        let attendance_1_a = CreateEventAttendance {
            event_id: event_1.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let attendance_2_a = CreateEventAttendance {
            event_id: event_2.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let attendance_2_b = CreateEventAttendance {
            event_id: event_2.id,
            user_id: user_id_3,
            role: "participant".to_string(),
        };
        let attendance_2_c = CreateEventAttendance {
            event_id: event_2.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let attendance_5_a = CreateEventAttendance {
            event_id: event_5.id,
            user_id: user_id_1,
            role: "participant".to_string(),
        };
        let attendance_5_b = CreateEventAttendance {
            event_id: event_5.id,
            user_id: user_id_2,
            role: "participant".to_string(),
        };
        let _ = event_attendance::create(&pool, &attendance_1_a).await?;
        let _ = event_attendance::create(&pool, &attendance_2_a).await?;
        let _ = event_attendance::create(&pool, &attendance_2_b).await?;
        let _ = event_attendance::create(&pool, &attendance_2_c).await?;
        let _ = event_attendance::create(&pool, &attendance_5_a).await?;
        let _ = event_attendance::create(&pool, &attendance_5_b).await?;

        // Event 1 at capacity
        // Event 2 at capacity
        // Event 3 has capacity but no attendees (available)
        // Event 4 has no capacity (available)
        // Event 5 at capacity

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let full_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                capacity_status: Some(CapacityStatus::Full),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;
        let available_results = list(
            &pool,
            &conversation_id,
            page_options.clone(),
            EventFilterOptions {
                capacity_status: Some(CapacityStatus::Available),
                ..Default::default()
            },
            EventOrderOptions {
                ..Default::default()
            },
            None,
        )
        .await?;

        assert_eq!(full_results.total, 3, "incorrect number of past events");
        assert_eq!(
            full_results.records[0].event.name,
            "test_event_1".to_string(),
            "incorrect full event name [0]"
        );
        assert_eq!(
            full_results.records[0].current_attendance, 1,
            "incorrect full attendance [0]"
        );
        assert_eq!(
            full_results.records[1].event.name,
            "test_event_2".to_string(),
            "incorrect full event name [1]"
        );
        assert_eq!(
            full_results.records[1].current_attendance, 3,
            "incorrect full attendance [1]"
        );
        assert_eq!(
            full_results.records[2].event.name,
            "test_event_5".to_string(),
            "incorrect full event name [2]"
        );
        assert_eq!(
            full_results.records[2].current_attendance, 2,
            "incorrect full attendance [2]"
        );
        assert_eq!(
            available_results.total, 2,
            "incorrect number of past events"
        );
        assert_eq!(
            available_results.records[0].event.name,
            "test_event_3".to_string(),
            "incorrect available event name [0]"
        );
        assert_eq!(
            available_results.records[0].current_attendance, 0,
            "incorrect available attendance [0]"
        );
        assert_eq!(
            available_results.records[1].event.name,
            "test_event_4".to_string(),
            "incorrect available event name [1]"
        );
        assert_eq!(
            available_results.records[1].current_attendance, 0,
            "incorrect available attendance [1]"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            ..Default::default()
        };

        let event = create(&pool, &conversation_id, &new_event).await?;

        let _ = delete(&pool, &event.id).await?;

        let err = get_localized_by_id(&pool, &event.id, "en")
            .await
            .unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(e, "Event".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
