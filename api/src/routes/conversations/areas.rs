//! Conversation coverage links and geographic discovery.

use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::middleware::from_fn_with_state;
use axum_extra::extract::cookie::CookieJar;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use super::dto::LocalizedConversationDto;
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::permissions::{PermissionRequirement, authorize};
use crate::models::conversation::{self, ConversationFilterOptions, ConversationOrderOptions};
use crate::models::pagination::{OrderParams, PageOptions, PaginatedResults};
use crate::models::permissions::ConversationResource;
use crate::models::permissions::conversation::Action;
use crate::models::{conversation_region_area, region, region_area};
use crate::routes::auth::{OptionalUser, is_user_super_admin};
use crate::routes::translations::LocaleExtractor;

/// Existing region areas defining the conversation's optional coverage.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConversationAreaLinksDto {
    pub conversation_id: Uuid,
    pub area_ids: Vec<Uuid>,
}

/// Replace the coverage set; an empty list removes all coverage.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConversationAreaLinksRequest {
    pub area_ids: Vec<Uuid>,
}

async fn area_links(
    state: &ComhairleState,
    conversation_id: Uuid,
) -> Result<Json<ConversationAreaLinksDto>, ComhairleError> {
    Ok(Json(ConversationAreaLinksDto {
        conversation_id,
        area_ids: conversation_region_area::list_area_ids(&state.db, &conversation_id).await?,
    }))
}

/// List coverage using the same read and draft-preview rules as the conversation.
#[instrument(err(Debug), skip(state, user, jar))]
async fn get_area_links(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    OptionalUser(user): OptionalUser,
    jar: CookieJar,
) -> Result<Json<ConversationAreaLinksDto>, ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    super::ensure_can_read(&state, &conversation, user.as_ref(), &jar).await?;
    area_links(&state, conversation_id).await
}

/// Replace links to existing areas without modifying their shared boundaries.
#[instrument(err(Debug), skip(state))]
async fn set_area_links(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Json(request): Json<ConversationAreaLinksRequest>,
) -> Result<Json<ConversationAreaLinksDto>, ComhairleError> {
    for area_id in &request.area_ids {
        region_area::get_by_id(&state.db, area_id).await?;
    }
    conversation_region_area::set_area_links(&state.db, &conversation_id, &request.area_ids)
        .await?;
    area_links(&state, conversation_id).await
}

/// Idempotently attach an existing area to a conversation.
#[instrument(err(Debug), skip(state))]
async fn add_area_link(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, area_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ConversationAreaLinksDto>, ComhairleError> {
    region_area::get_by_id(&state.db, &area_id).await?;
    conversation_region_area::add_area_link(&state.db, &conversation_id, &area_id).await?;
    area_links(&state, conversation_id).await
}

/// Idempotently detach an area, leaving the shared region area intact.
#[instrument(err(Debug), skip(state))]
async fn remove_area_link(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, area_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ConversationAreaLinksDto>, ComhairleError> {
    region_area::get_by_id(&state.db, &area_id).await?;
    conversation_region_area::remove_area_link(&state.db, &conversation_id, &area_id).await?;
    area_links(&state, conversation_id).await
}

/// Check geographic overlap and lifecycle status, subject to conversation read access.
#[instrument(err(Debug), skip(state, user, jar))]
async fn is_active_in_area(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, area_id)): Path<(Uuid, Uuid)>,
    OptionalUser(user): OptionalUser,
    jar: CookieJar,
) -> Result<Json<bool>, ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    super::ensure_can_read(&state, &conversation, user.as_ref(), &jar).await?;
    region_area::get_by_id(&state.db, &area_id).await?;
    Ok(Json(
        conversation_region_area::is_active_in_area(&state.db, &conversation_id, &area_id).await?,
    ))
}

/// Discover overlapping conversations the caller may see: public live ones, plus drafts and
/// private ones the signed-in user can read.
#[instrument(err(Debug), skip(state, order_options))]
async fn intersecting_region(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    OptionalUser(user): OptionalUser,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(mut filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<PaginatedResults<LocalizedConversationDto>>, ComhairleError> {
    region::get_by_id(&state.db, &region_id).await?;
    filter_options.region_id = Some(region_id);
    let is_super_admin = match &user {
        Some(user) => is_user_super_admin(&state, user).await,
        None => false,
    };
    let results = conversation::list_visible(
        &state.db,
        user.map(|user| user.id),
        is_super_admin,
        page_options,
        order_options,
        filter_options,
        Some(locale),
    )
    .await?;
    Ok(Json(super::with_media(&state, results).await?))
}

/// Routes merged into the conversation router, with updates protected by conversation permissions.
pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    let update = || {
        from_fn_with_state(
            PermissionRequirement::<ConversationResource>::new(Action::Update),
            authorize::<ConversationResource>,
        )
    };
    ApiRouter::new()
        .api_route(
            "/intersecting/{region_id}",
            get_with(intersecting_region, |op| {
                op.id("ListConversationsIntersectingRegion")
                    .tag("Conversation")
                    .summary("List visible conversations intersecting a region")
                    .description("Any intersection with any of the region's areas counts, including shared boundaries, regardless of live or complete status. Anonymous callers see public live conversations; signed-in users also see drafts and private conversations they can read. Results are localized, paginated, and unique per conversation. Conversations with no coverage and areas without geometry do not match.")
                    .response::<200, Json<PaginatedResults<LocalizedConversationDto>>>()
            }),
        )
        .api_route(
            "/{conversation_id}/areas",
            get_with(get_area_links, |op| {
                op.id("GetConversationAreaLinks")
                    .tag("Conversation")
                    .summary("List the region areas attached to a conversation")
                    .response::<200, Json<ConversationAreaLinksDto>>()
            }),
        )
        .api_route(
            "/{conversation_id}/areas",
            put_with(set_area_links, |op| {
                op.id("SetConversationAreaLinks")
                    .tag("Conversation")
                    .security_requirement("JWT")
                    .summary("Replace conversation coverage with existing region area IDs")
                    .description("Requires conversation update permission. Duplicate IDs are ignored. An empty area_ids list clears all coverage.")
                    .response::<200, Json<ConversationAreaLinksDto>>()
            })
            .route_layer(update()),
        )
        .api_route(
            "/{conversation_id}/areas/{area_id}",
            post_with(add_area_link, |op| {
                op.id("AddConversationAreaLink")
                    .tag("Conversation")
                    .security_requirement("JWT")
                    .summary("Attach an existing region area to a conversation")
                    .response::<200, Json<ConversationAreaLinksDto>>()
            })
            .route_layer(update()),
        )
        .api_route(
            "/{conversation_id}/areas/{area_id}",
            delete_with(remove_area_link, |op| {
                op.id("RemoveConversationAreaLink")
                    .tag("Conversation")
                    .security_requirement("JWT")
                    .summary("Detach a region area without deleting it")
                    .response::<200, Json<ConversationAreaLinksDto>>()
            })
            .route_layer(update()),
        )
        .api_route(
            "/{conversation_id}/areas/{area_id}/active",
            get_with(is_active_in_area, |op| {
                op.id("IsConversationActiveInArea")
                    .tag("Conversation")
                    .summary("Check whether a conversation is active in a region area")
                    .description("Returns a boolean: true only when the conversation is live, not complete, and at least one attached boundary intersects the supplied area's boundary. Shared edges count. No coverage or missing geometry returns false. Missing resources return 404. Draft conversations require team or preview access.")
                    .response::<200, Json<bool>>()
            }),
        )
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use serde_json::{Value, json};
    use sqlx::PgPool;

    use super::*;
    use crate::models::conversation::PartialConversation;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_app_and_session_with_state, setup_default_app_and_session,
    };
    use crate::models::permissions::conversation::Role;
    use crate::models::permissions::{ActorId, GrantRoleRequest, PermissionRole, grant_role};
    use crate::models::region::CreateRegion;
    use crate::models::region_area::CreateRegionArea;
    use crate::test_helpers::{UserSession, test_state};

    async fn create_area(pool: &PgPool, geometry: Option<Value>) -> Result<Uuid, ComhairleError> {
        Ok(region_area::create(
            pool,
            CreateRegionArea {
                area_geometry: geometry,
                ..Default::default()
            },
        )
        .await?
        .id)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn conversation_area_links_crud_is_idempotent_and_atomic(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let url = format!("/conversation/{conversation_id}/areas");
        let first = create_area(&pool, None).await?;
        let second = create_area(&pool, None).await?;

        let (status, response, _) = session.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK, "{response}");
        assert_eq!(response["area_ids"], json!([]));

        let (status, response, _) = session
            .put(
                &app,
                &url,
                json!({"area_ids": [first, first]}).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "{response}");
        assert_eq!(response["conversation_id"], json!(conversation_id));
        assert_eq!(response["area_ids"], json!([first]));

        for _ in 0..2 {
            let (status, response, _) = session
                .post(&app, &format!("{url}/{second}"), "{}".into())
                .await?;
            assert_eq!(status, StatusCode::OK, "{response}");
            let links: ConversationAreaLinksDto = serde_json::from_value(response)?;
            assert_eq!(links.area_ids.len(), 2);
        }
        for _ in 0..2 {
            let (status, response, _) = session.delete(&app, &format!("{url}/{first}")).await?;
            assert_eq!(status, StatusCode::OK, "{response}");
            assert_eq!(response["area_ids"], json!([second]));
        }
        region_area::get_by_id(&pool, &first).await?;

        let missing = Uuid::new_v4();
        let (status, _, _) = session
            .put(
                &app,
                &url,
                json!({"area_ids": [first, missing]}).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(
            conversation_region_area::set_area_links(&pool, &conversation_id, &[first, missing])
                .await
                .is_err()
        );
        assert_eq!(
            conversation_region_area::list_area_ids(&pool, &conversation_id).await?,
            vec![second]
        );

        let (status, response, _) = session
            .put(&app, &url, json!({"area_ids": []}).to_string().into())
            .await?;
        assert_eq!(status, StatusCode::OK, "{response}");
        assert_eq!(response["area_ids"], json!([]));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn conversation_area_intersections_respect_geometry_and_lifecycle(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let coverage = create_area(
            &pool,
            Some(json!({"type": "MultiPolygon", "coordinates": [
                [[[0,0],[10,0],[10,10],[0,10],[0,0]], [[2,2],[2,4],[4,4],[4,2],[2,2]]],
                [[[20,20],[21,20],[21,21],[20,21],[20,20]]]
            ]})),
        )
        .await?;
        conversation_region_area::add_area_link(&pool, &conversation_id, &coverage).await?;
        let mut region_areas = Vec::new();
        for (bounds, expected) in [
            ((5.0, 5.0, 15.0, 15.0), true),
            ((10.0, 0.0, 11.0, 1.0), true),
            ((2.5, 2.5, 3.5, 3.5), false),
            ((20.2, 20.2, 20.8, 20.8), true),
            ((30.0, 30.0, 31.0, 31.0), false),
        ] {
            let (min_x, min_y, max_x, max_y) = bounds;
            let area_id = create_area(
                &pool,
                Some(json!({"type": "Polygon", "coordinates": [[
                    [min_x,min_y],[max_x,min_y],[max_x,max_y],[min_x,max_y],[min_x,min_y]
                ]]})),
            )
            .await?;
            let url = format!("/conversation/{conversation_id}/areas/{area_id}/active");
            let (status, response, _) = session.get(&app, &url).await?;
            assert_eq!(status, StatusCode::OK, "{response}");
            assert_eq!(response, json!(expected));
            region_areas.push(area_id);
        }
        let without_geometry = create_area(&pool, None).await?;
        assert!(
            !conversation_region_area::is_active_in_area(
                &pool,
                &conversation_id,
                &without_geometry
            )
            .await?
        );
        region_areas.push(without_geometry);
        let region = region::create(&pool, &CreateRegion::default(), "en").await?;
        region::set_area_links(&pool, &region.id, &region_areas).await?;
        let second_conversation = get_random_conversation_id(&app, &mut session).await?;
        conversation_region_area::set_area_links(
            &pool,
            &second_conversation,
            &[coverage, region_areas[0]],
        )
        .await?;
        get_random_conversation_id(&app, &mut session).await?;
        let url = format!("/conversation/intersecting/{}", region.id);
        let (status, response, _) = session.get(&app, &format!("{url}?limit=1")).await?;
        assert_eq!(status, StatusCode::OK, "{response}");
        let matches: PaginatedResults<LocalizedConversationDto> = serde_json::from_value(response)?;
        assert_eq!(matches.total, 2);
        assert_eq!(matches.records.len(), 1);

        let mut anonymous = UserSession::new_guest();
        for (is_live, is_complete) in [(false, false), (true, true), (true, false)] {
            conversation::update(
                &pool,
                &conversation_id,
                &PartialConversation {
                    is_live: Some(is_live),
                    is_complete: Some(is_complete),
                    ..Default::default()
                },
            )
            .await?;
            let (status, response, _) = session
                .get(
                    &app,
                    &format!("/conversation/{conversation_id}/areas/{coverage}/active"),
                )
                .await?;
            assert_eq!(status, StatusCode::OK, "{response}");
            assert_eq!(response, json!(is_live && !is_complete));
            let (status, response, _) = session.get(&app, &url).await?;
            assert_eq!(status, StatusCode::OK, "{response}");
            assert_eq!(response["total"], json!(2));
            let (status, response, _) = anonymous.get(&app, &url).await?;
            assert_eq!(status, StatusCode::OK, "{response}");
            assert_eq!(response["total"], json!(if is_live { 2 } else { 1 }));
        }
        conversation::update(
            &pool,
            &conversation_id,
            &PartialConversation {
                is_public: Some(false),
                ..Default::default()
            },
        )
        .await?;
        let (_, response, _) = session.get(&app, &url).await?;
        assert_eq!(response["total"], json!(2));
        let (_, response, _) = anonymous.get(&app, &url).await?;
        assert_eq!(response["total"], json!(1));

        conversation_region_area::set_area_links(&pool, &conversation_id, &[]).await?;
        assert!(
            !conversation_region_area::is_active_in_area(&pool, &conversation_id, &coverage)
                .await?
        );
        let empty_region = region::create(&pool, &CreateRegion::default(), "en").await?;
        let (_, response, _) = session
            .get(
                &app,
                &format!("/conversation/intersecting/{}", empty_region.id),
            )
            .await?;
        assert_eq!(response["total"], json!(0));
        let (status, _, _) = session
            .get(
                &app,
                &format!("/conversation/intersecting/{}", Uuid::new_v4()),
            )
            .await?;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (status, _, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{conversation_id}/areas/{}/active",
                    Uuid::new_v4()
                ),
            )
            .await?;
        assert_eq!(status, StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn conversation_area_writes_require_update_permission(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool.clone()).call()?);
        let (app, mut owner) = setup_app_and_session_with_state(state.clone()).await?;
        let conversation_id = get_random_conversation_id(&app, &mut owner).await?;
        let area_id = create_area(&pool, None).await?;
        let mut observer = UserSession::new_guest();
        observer.signup_guest(&app).await?;
        let owner_id = owner.id.ok_or("owner has no ID")?;
        grant_role(
            &state,
            GrantRoleRequest {
                actor_id: ActorId::User(observer.id.ok_or("observer has no ID")?),
                permission_triplet: Role::Observer.triplet(&conversation_id)?,
                granted_by: &owner_id,
                grant_reason: "Area permission test",
            },
        )
        .await?;
        let url = format!("/conversation/{conversation_id}/areas");
        let (status, _, _) = observer.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        let (status, _, _) = observer
            .put(
                &app,
                &url,
                json!({"area_ids": [area_id]}).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = observer
            .post(&app, &format!("{url}/{area_id}"), "{}".into())
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = observer.delete(&app, &format!("{url}/{area_id}")).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        Ok(())
    }
}
