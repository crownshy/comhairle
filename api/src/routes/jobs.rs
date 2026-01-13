use std::sync::Arc;

use aide::axum::{routing::get_with, ApiRouter};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        job::{self, Job, JobFilterOptions, JobOrderOptions},
        pagination::{OrderParams, PageOptions, PaginatedResults},
    },
    routes::auth::RequiredAdminUser,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    OrderParams(order_options): OrderParams<JobOrderOptions>,
    Query(filter_options): Query<JobFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<Job>>), ComhairleError> {
    let jobs = job::list(&state.db, page_options, order_options, filter_options).await?;

    Ok((StatusCode::OK, Json(jobs)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(job_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Job>), ComhairleError> {
    let job = job::get_id_id(&state.db, &job_id).await?;

    Ok((StatusCode::OK, Json(job)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListJobs")
                    .tag("Jobs")
                    .summary("List jobs")
                    .response::<200, Json<PaginatedResults<Job>>>()
            }),
        )
        .api_route(
            "/{job_id}",
            get_with(get, |op| {
                op.id("GetJob")
                    .tag("Jobs")
                    .summary("Get a job by id")
                    .response::<200, Json<Job>>()
            }),
        )
        .with_state(state)
}
