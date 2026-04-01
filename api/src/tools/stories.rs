use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use async_trait::async_trait;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ComhairleError, ComhairleState};

use super::{ToolConfigSanitize, ToolImpl};

#[derive(Debug, Default, JsonSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct StoriesToolConfig {
    pub max_time: i32,
    pub to_see: i32,
}

#[derive(Debug, Default, JsonSchema, Serialize, Deserialize, Clone)]
pub struct StoriesReport;

#[derive(Debug, Default, JsonSchema, Serialize, Deserialize, Clone)]
pub struct StoriesToolSetup {
    pub max_time: i32,
    pub to_see: i32,
}

async fn stories_setup(config: &StoriesToolSetup) -> Result<StoriesToolConfig, ComhairleError> {
    Ok(StoriesToolConfig {
        max_time: config.max_time,
        to_see: config.to_see,
    })
}

// Keep public function for backwards compatibility
pub async fn setup(config: &StoriesToolSetup) -> Result<StoriesToolConfig, ComhairleError> {
    stories_setup(config).await
}

impl ToolConfigSanitize for StoriesToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

/// Zero-sized marker type for Stories tool implementation
pub struct StoriesTool;

#[async_trait]
impl ToolImpl for StoriesTool {
    type Config = StoriesToolConfig;
    type Setup = StoriesToolSetup;
    type Report = StoriesReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        stories_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Stories tool is cloneable as-is (no external state)
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        routes(state.clone())
    }
}

/// Helper function to create routes (kept for backwards compatibility)
pub fn routes(state: Arc<ComhairleState>) -> ApiRouter {
    stories_routes(state)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Story {
    pub id: Uuid,
    pub video_id: Uuid,
    pub user_id: Uuid,
    pub transcript_id: Option<Uuid>,
    pub workflow_step_id: Uuid,
}

async fn get_stories(
    State(_state): State<Arc<ComhairleState>>,
    Path(_workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Story>>), ComhairleError> {
    Ok((StatusCode::OK, Json(vec![])))
}

async fn save_story(
    State(_state): State<Arc<ComhairleState>>,
    Path(_workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Story>>), ComhairleError> {
    Ok((StatusCode::OK, Json(vec![])))
}

async fn get_story(
    State(_state): State<Arc<ComhairleState>>,
    Path(_workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Story>>), ComhairleError> {
    Ok((StatusCode::OK, Json(vec![])))
}

fn stories_routes(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/stories/workflow_step/{workflow_step_id}",
            get_with(get_stories, |op| {
                op.id("GetStories")
                    .tag("Tools")
                    .summary(
                        "Returns stories for the current workflow step if it is a stories endpoint",
                    )
                    .description(
                        "Returns stories for the current workflow step if it is a stories endpoint",
                    )
                    .response::<201, Json<Vec<Story>>>()
            }),
        )
        .api_route(
            "/stories/workflow_step/{workflow_step_id}",
            post_with(save_story, |op| {
                op.id("SaveStory")
                    .tag("Tools")
                    .summary("Record a user story for the current user and workflow step")
                    .description("Record a user story for the current user and workflow step")
            }),
        )
        .api_route(
            "/stories/{story_id}",
            get_with(get_story, |op| {
                op.id("GetStory")
                    .tag("Tools")
                    .summary("Get a story by id")
                    .description("Returns a story by id")
                    .response::<201, Json<Story>>()
            }),
        )
        .with_state(state)
}
