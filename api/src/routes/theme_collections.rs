use std::collections::HashMap;
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::middleware::permissions::{PermissionRequirement, authorize};
use crate::models::job::CreateJob;
use crate::models::permissions::ConversationResource;
use crate::models::permissions::conversation::Action;
use crate::models::theme::{self, CreateThemeCollection, NewTheme, UpdateTheme, UpdateThemePinned};
use crate::models::{statement, statement_theme};
use crate::routes::auth::RequiredUser;
use crate::routes::theme_collections::dto::{ThemeCollectionDto, ThemeDto};
use crate::sensemaking::harvest;
use crate::worker_service::process_theme_extraction::ThemeExtractionJob;
use crate::{ComhairleError, ComhairleState};

pub mod dto;

fn path_param(params: &HashMap<String, Uuid>, name: &str) -> Result<Uuid, ComhairleError> {
    params
        .get(name)
        .copied()
        .ok_or_else(|| ComhairleError::ResourceNotFound(format!("Path must contain {name}")))
}

#[instrument(err(Debug), skip(state))]
async fn create_collection(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    Json(payload): Json<CreateThemeCollection>,
) -> Result<(StatusCode, Json<ThemeCollectionDto>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;

    let collection = theme::create_collection(&state.db, workflow_step_id, &payload).await?;

    Ok((StatusCode::CREATED, Json(collection.into())))
}

#[derive(Deserialize, Debug, JsonSchema)]
struct LatestThemeCollectionQuery {
    #[serde(default)]
    source_tool: Option<String>,
    /// Narrows to a single proposal/section ("proposal:{id}") or heyform question
    /// ("question:{id}"); ignored by tools that don't use sub scopes.
    #[serde(default)]
    sub_scope: Option<String>,
}

/// The most recently generated theme collection for a workflow step (optionally
/// narrowed to one source tool and/or sub scope), so the UI can restore previously
/// generated themes after a page refresh instead of only ever seeing what was
/// generated this session.
#[instrument(err(Debug), skip(state))]
async fn get_latest_theme_collection(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    Query(query): Query<LatestThemeCollectionQuery>,
) -> Result<(StatusCode, Json<ThemeCollectionDto>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;

    let collection = theme::get_latest_for_workflow_step(
        &state.db,
        workflow_step_id,
        query.source_tool.as_deref(),
        query.sub_scope.as_deref(),
    )
    .await?
    .ok_or_else(|| ComhairleError::ResourceNotFound("Theme Collection".into()))?;

    let (collection, themes) =
        theme::list_themes(&state.db, workflow_step_id, collection.id).await?;
    let statements = statement::list_for_theme_collection(&state.db, collection.id).await?;
    let assignments = statement_theme::list_for_theme_collection(&state.db, collection.id).await?;

    Ok((
        StatusCode::OK,
        Json(ThemeCollectionDto::from_parts_with_statements(
            collection,
            themes,
            statements,
            assignments,
        )),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn list_themes(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
) -> Result<(StatusCode, Json<ThemeCollectionDto>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;
    let theme_collection_id = path_param(&params, "theme_collection_id")?;

    let (collection, themes) =
        theme::list_themes(&state.db, workflow_step_id, theme_collection_id).await?;
    let statements = statement::list_for_theme_collection(&state.db, theme_collection_id).await?;
    let assignments =
        statement_theme::list_for_theme_collection(&state.db, theme_collection_id).await?;

    Ok((
        StatusCode::OK,
        Json(ThemeCollectionDto::from_parts_with_statements(
            collection,
            themes,
            statements,
            assignments,
        )),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn delete_collection(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
) -> Result<StatusCode, ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;
    let theme_collection_id = path_param(&params, "theme_collection_id")?;

    theme::delete_collection(&state.db, workflow_step_id, theme_collection_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn add_theme(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    Json(payload): Json<NewTheme>,
) -> Result<(StatusCode, Json<ThemeDto>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;
    let theme_collection_id = path_param(&params, "theme_collection_id")?;

    let theme =
        theme::add_theme(&state.db, workflow_step_id, theme_collection_id, &payload).await?;

    Ok((StatusCode::CREATED, Json(theme.into())))
}

#[instrument(err(Debug), skip(state))]
async fn update_theme(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    Json(payload): Json<UpdateTheme>,
) -> Result<(StatusCode, Json<ThemeDto>), ComhairleError> {
    let theme_collection_id = path_param(&params, "theme_collection_id")?;
    let theme_id = path_param(&params, "theme_id")?;

    let theme = theme::update_theme(&state.db, theme_collection_id, theme_id, &payload).await?;

    Ok((StatusCode::OK, Json(theme.into())))
}

#[instrument(err(Debug), skip(state))]
async fn set_theme_pinned(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    Json(payload): Json<UpdateThemePinned>,
) -> Result<(StatusCode, Json<ThemeDto>), ComhairleError> {
    let theme_collection_id = path_param(&params, "theme_collection_id")?;
    let theme_id = path_param(&params, "theme_id")?;

    let theme = theme::set_pinned(&state.db, theme_collection_id, theme_id, payload.pinned).await?;

    Ok((StatusCode::OK, Json(theme.into())))
}

#[instrument(err(Debug), skip(state))]
async fn delete_theme(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
) -> Result<StatusCode, ComhairleError> {
    let theme_collection_id = path_param(&params, "theme_collection_id")?;
    let theme_id = path_param(&params, "theme_id")?;

    theme::delete_theme(&state.db, theme_collection_id, theme_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GenerateThemeCollectionRequest {
    /// Which tool to harvest statements from: "heyform", "polis", "thinking_space",
    /// or "prioritization".
    pub source_tool: String,
    /// Narrows prioritization to a single proposal or section ("proposal:{id}" or
    /// "proposal:{id}/section:{id}"), or heyform to a single question
    /// ("question:{id}"). Ignored by other tools.
    #[serde(default)]
    pub sub_scope: Option<String>,
    pub context: String,
    #[serde(default)]
    pub additional_instructions: Option<String>,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct GenerateThemeCollectionResponse {
    pub job_id: Uuid,
    pub theme_collection_id: Uuid,
}

/// Harvests the source tool's free-text data into `statement` rows, creates the
/// theme collection those statements will be grouped under, and queues a
/// background job that runs the sensemaker model and stores the themes and
/// statement assignments. Progress is tracked through the returned job id via
/// the existing `GET /jobs/{job_id}`.
#[instrument(err(Debug), skip(state))]
async fn generate_theme_collection(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    RequiredUser(user): RequiredUser,
    Json(request): Json<GenerateThemeCollectionRequest>,
) -> Result<(StatusCode, Json<GenerateThemeCollectionResponse>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;

    let harvested = harvest::harvest(
        &state,
        workflow_step_id,
        &request.source_tool,
        request.sub_scope.as_deref(),
    )
    .await?;

    let collection = theme::create_collection(
        &state.db,
        workflow_step_id,
        &CreateThemeCollection {
            context: request.context.clone(),
            origin_tool: request.source_tool,
            origin_sub_id: request.sub_scope,
            requested_by_user_id: Some(user.id),
        },
    )
    .await?;

    let created_job = crate::models::job::create(
        &state.db,
        CreateJob {
            step: Some("theme_extraction".to_string()),
            ..Default::default()
        },
    )
    .await?;

    let worker_service = state.required_worker_service()?;
    worker_service
        .push_theme_extraction_job(ThemeExtractionJob {
            job_id: created_job.id,
            workflow_step_id,
            theme_collection_id: collection.id,
            statement_ids: harvested.iter().map(|s| s.id).collect(),
            context: Some(request.context),
            additional_instructions: request.additional_instructions,
        })
        .await?;

    Ok((
        StatusCode::ACCEPTED,
        Json(GenerateThemeCollectionResponse {
            job_id: created_job.id,
            theme_collection_id: collection.id,
        }),
    ))
}

pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_collection, |op| {
                op.id("CreateThemeCollection")
                    .tag("ThemeCollections")
                    .summary("Create a theme collection for a workflow step")
                    .security_requirement("JWT")
                    .response::<201, Json<ThemeCollectionDto>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/generate",
            post_with(generate_theme_collection, |op| {
                op.id("GenerateThemeCollection")
                    .tag("ThemeCollections")
                    .summary("Harvest a tool's statements and generate themes for them")
                    .security_requirement("JWT")
                    .response::<202, Json<GenerateThemeCollectionResponse>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/latest",
            get_with(get_latest_theme_collection, |op| {
                op.id("GetLatestThemeCollection")
                    .tag("ThemeCollections")
                    .summary("Get the most recently generated theme collection for a workflow step")
                    .security_requirement("JWT")
                    .response::<200, Json<ThemeCollectionDto>>()
            }),
        )
        .api_route(
            "/{theme_collection_id}",
            get_with(list_themes, |op| {
                op.id("ListThemeCollectionThemes")
                    .tag("ThemeCollections")
                    .summary("Get a theme collection and its themes")
                    .security_requirement("JWT")
                    .response::<200, Json<ThemeCollectionDto>>()
            }),
        )
        .api_route(
            "/{theme_collection_id}",
            delete_with(delete_collection, |op| {
                op.id("DeleteThemeCollection")
                    .tag("ThemeCollections")
                    .summary("Delete a theme collection and its themes")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/{theme_collection_id}/themes",
            post_with(add_theme, |op| {
                op.id("AddTheme")
                    .tag("ThemeCollections")
                    .summary("Add a theme to a theme collection")
                    .security_requirement("JWT")
                    .response::<201, Json<ThemeDto>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/{theme_collection_id}/themes/{theme_id}",
            put_with(update_theme, |op| {
                op.id("UpdateTheme")
                    .tag("ThemeCollections")
                    .summary("Update a theme's title and description")
                    .security_requirement("JWT")
                    .response::<200, Json<ThemeDto>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/{theme_collection_id}/themes/{theme_id}/pin",
            put_with(set_theme_pinned, |op| {
                op.id("SetThemePinned")
                    .tag("ThemeCollections")
                    .summary("Pin or unpin a theme for future generation runs")
                    .security_requirement("JWT")
                    .response::<200, Json<ThemeDto>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/{theme_collection_id}/themes/{theme_id}",
            delete_with(delete_theme, |op| {
                op.id("DeleteTheme")
                    .tag("ThemeCollections")
                    .summary("Delete a theme")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .route_layer(from_fn_with_state(
            PermissionRequirement::<ConversationResource>::new(Action::Read),
            authorize::<ConversationResource>,
        ))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
    };
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::test_helpers::{UserSession, learn_tool_config};

    async fn create_step_and_url(
        app: &crate::App,
        session: &mut UserSession,
    ) -> Result<(Uuid, String), Box<dyn Error>> {
        let conversation_id = get_random_conversation_id(app, session).await?;
        let workflow_id = get_random_workflow_id(app, session, &conversation_id).await?;

        let (status, value, _) = session
            .create_workflow_step(
                app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A step",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": learn_tool_config(),
                }),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "step not created: {value}");
        let step: WorkflowStepDto = serde_json::from_value(value)?;

        Ok((
            conversation_id,
            format!(
                "/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step/{}/theme_collections",
                step.id
            ),
        ))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_collection_add_theme_and_list(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, collections_url) = create_step_and_url(&app, &mut session).await?;

        let (status, res, _) = session
            .post(
                &app,
                &collections_url,
                json!({ "context": "Transport statements" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "collection not created: {res}");
        let collection: ThemeCollectionDto = serde_json::from_value(res)?;

        let (status, res, _) = session
            .post(
                &app,
                &format!("{collections_url}/{}/themes", collection.id),
                json!({ "title": "Buses", "description": "More buses", "createdByAi": true })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "theme not added: {res}");
        let added: ThemeDto = serde_json::from_value(res)?;
        assert!(added.created_by_ai, "created_by_ai not saved");

        let (status, res, _) = session
            .get(&app, &format!("{collections_url}/{}", collection.id))
            .await?;
        assert_eq!(status, StatusCode::OK, "collection not fetched: {res}");
        let fetched: ThemeCollectionDto = serde_json::from_value(res)?;
        assert_eq!(fetched.themes.len(), 1, "incorrect theme count");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_and_delete_theme_then_collection(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, collections_url) = create_step_and_url(&app, &mut session).await?;

        let (_, res, _) = session
            .post(
                &app,
                &collections_url,
                json!({ "context": "Context" }).to_string().into(),
            )
            .await?;
        let collection: ThemeCollectionDto = serde_json::from_value(res)?;

        let (_, res, _) = session
            .post(
                &app,
                &format!("{collections_url}/{}/themes", collection.id),
                json!({ "title": "Original", "description": "Original description" })
                    .to_string()
                    .into(),
            )
            .await?;
        let theme: ThemeDto = serde_json::from_value(res)?;
        let theme_url = format!("{collections_url}/{}/themes/{}", collection.id, theme.id);

        let (status, res, _) = session
            .put(
                &app,
                &theme_url,
                json!({ "title": "Renamed", "description": "New description" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "theme not updated: {res}");
        let updated: ThemeDto = serde_json::from_value(res)?;
        assert_eq!(updated.title, "Renamed", "title not updated");

        let (status, _, _) = session.delete(&app, &theme_url).await?;
        assert_eq!(status, StatusCode::NO_CONTENT, "theme not deleted");

        let collection_url = format!("{collections_url}/{}", collection.id);
        let (status, _, _) = session.delete(&app, &collection_url).await?;
        assert_eq!(status, StatusCode::NO_CONTENT, "collection not deleted");

        let (status, _, _) = session.get(&app, &collection_url).await?;
        assert_eq!(status, StatusCode::NOT_FOUND, "collection still exists");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_pin_and_unpin_theme(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, collections_url) = create_step_and_url(&app, &mut session).await?;

        let (_, res, _) = session
            .post(
                &app,
                &collections_url,
                json!({ "context": "Context" }).to_string().into(),
            )
            .await?;
        let collection: ThemeCollectionDto = serde_json::from_value(res)?;

        let (_, res, _) = session
            .post(
                &app,
                &format!("{collections_url}/{}/themes", collection.id),
                json!({ "title": "Buses", "description": "More buses", "createdByAi": true })
                    .to_string()
                    .into(),
            )
            .await?;
        let theme: ThemeDto = serde_json::from_value(res)?;
        assert!(!theme.pinned, "themes should default unpinned");

        let pin_url = format!(
            "{collections_url}/{}/themes/{}/pin",
            collection.id, theme.id
        );
        let (status, res, _) = session
            .put(&app, &pin_url, json!({ "pinned": true }).to_string().into())
            .await?;
        assert_eq!(status, StatusCode::OK, "theme not pinned: {res}");
        let pinned: ThemeDto = serde_json::from_value(res)?;
        assert!(pinned.pinned, "pinned flag not saved");

        let (status, res, _) = session
            .put(
                &app,
                &pin_url,
                json!({ "pinned": false }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "theme not unpinned: {res}");
        let unpinned: ThemeDto = serde_json::from_value(res)?;
        assert!(!unpinned.pinned, "unpinned flag not saved");

        Ok(())
    }
}
