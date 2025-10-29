use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ComhairleError, ComhairleState};

use super::ToolConfigSanitize;

#[derive(Debug, Default, JsonSchema, Serialize, Deserialize, Clone)]
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

pub async fn setup(config: &StoriesToolSetup) -> Result<StoriesToolConfig, ComhairleError> {
    Ok(StoriesToolConfig {
        max_time: config.max_time,
        to_see: config.to_see,
    })
}

impl ToolConfigSanitize for StoriesToolConfig {
    fn sanatize(&self) -> Self {
        self.clone()
    }
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
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Story>>), ComhairleError> {
    Ok((StatusCode::OK, Json(vec![])))
}

async fn save_story(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Story>>), ComhairleError> {
    Ok((StatusCode::OK, Json(vec![])))
}

async fn get_story(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Story>>), ComhairleError> {
    Ok((StatusCode::OK, Json(vec![])))
}

pub fn routes(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/tools/stories/workflow_step/{workflow_step_id}",
            get_with(get_stories, |op| {
                op.id("GetStories")
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
            "/tools/stories/workflow_step/{workflow_step_id}",
            post_with(save_story, |op| {
                op.id("SaveStory")
                    .summary("Record a user story for the current user and workflow step")
                    .description("Record a user story for the current user and workflow step")
            }),
        )
        .api_route(
            "tools/stories/{story_id}",
            get_with(get_story, |op| {
                op.id("GetStory")
                    .summary("Get a story by id")
                    .description("Returns a story by id")
                    .response::<201, Json<Story>>()
            }),
        )
        .with_state(state)
}
