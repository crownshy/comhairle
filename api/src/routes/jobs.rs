use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        job::{self, CreateJob, Job, JobFilterOptions, JobOrderOptions},
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
    RequiredAdminUser(_user): RequiredAdminUser,
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

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(body): Json<CreateJob>,
) -> Result<(StatusCode, Json<Job>), ComhairleError> {
    let job = job::create(&state.db, body).await?;

    Ok((StatusCode::CREATED, Json(job)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(job_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = job::delete(&state.db, &job_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListJobs")
                    .tag("Jobs")
                    .summary("List jobs")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<Job>>>()
            }),
        )
        .api_route(
            "/{job_id}",
            get_with(get, |op| {
                op.id("GetJob")
                    .tag("Jobs")
                    .summary("Get a job by id")
                    .security_requirement("JWT")
                    .response::<200, Json<Job>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateJob")
                    .tag("Jobs")
                    .summary("Create a new job")
                    .security_requirement("JWT")
                    .response::<200, Json<Job>>()
            }),
        )
        .api_route(
            "/{job_id}",
            delete_with(delete, |op| {
                op.id("DeleteJob")
                    .tag("Jobs")
                    .summary("Delete a job by id")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;
    use serde_json::json;
    use std::error::Error;

    use crate::{
        models::job::CreateJob,
        setup_server,
        test_helpers::{test_state, UserSession},
    };

    #[sqlx::test]
    async fn should_create_new_job(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job_1 = CreateJob {
            step: Some("initialize".to_string()),
            ..Default::default()
        };

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (status, value, _) = admin_session.create_job(&app, json!(job_1)).await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            value.get("step").and_then(|v| v.as_str()).unwrap(),
            "initialize",
            "incorrect response json"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_job_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job_1 = CreateJob {
            step: Some("initialize".to_string()),
            progress: Some(0.85),
        };

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (_, value, _) = admin_session.create_job(&app, json!(job_1)).await?;
        let id = value.get("id").and_then(|v| v.as_str()).unwrap();

        let (status, value, _) = admin_session.get(&app, &format!("/jobs/{id}")).await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            value.get("progress").and_then(|v| v.as_f64()).unwrap(),
            0.85,
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_retrieve_a_list_of_jobs(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job_1 = CreateJob {
            step: Some("initialize".to_string()),
            progress: Some(0.85),
        };
        let job_2 = CreateJob {
            step: Some("initialize".to_string()),
            progress: Some(0.65),
        };
        let job_3 = CreateJob {
            step: Some("initialize".to_string()),
            progress: Some(0.45),
        };

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let _ = admin_session.create_job(&app, json!(job_1)).await?;
        let _ = admin_session.create_job(&app, json!(job_2)).await?;
        let _ = admin_session.create_job(&app, json!(job_3)).await?;

        let (status, value, _) = admin_session.get(&app, "/jobs").await?;
        let jobs = value.get("records").and_then(|v| v.as_array()).unwrap();

        assert!(status.is_success(), "error response status");
        assert_eq!(jobs.len(), 3, "incorrect number of jobs");
        assert_eq!(
            jobs[0].get("progress").and_then(|v| v.as_f64()).unwrap(),
            0.85,
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_retrieve_jobs_list_filtered_by_step(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let job_1 = CreateJob {
            step: Some("find".to_string()),
            ..Default::default()
        };
        let job_2 = CreateJob {
            step: Some("find".to_string()),
            ..Default::default()
        };
        let job_3 = CreateJob {
            step: Some("don't find".to_string()),
            ..Default::default()
        };
        let job_4 = CreateJob {
            step: Some("don't find".to_string()),
            ..Default::default()
        };

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let _ = admin_session.create_job(&app, json!(job_1)).await?;
        let _ = admin_session.create_job(&app, json!(job_2)).await?;
        let _ = admin_session.create_job(&app, json!(job_3)).await?;
        let _ = admin_session.create_job(&app, json!(job_4)).await?;

        let (status, value, _) = admin_session.get(&app, "/jobs?step=find").await?;
        let jobs = value.get("records").and_then(|v| v.as_array()).unwrap();

        assert!(status.is_success(), "error response status");
        assert_eq!(jobs.len(), 2, "incorrect number of jobs");
        assert!(
            jobs.iter()
                .all(|j| j.get("step").and_then(|v| v.as_str()).unwrap() == "find"),
            "jobs not filtered"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_a_job_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job_1 = CreateJob {
            step: Some("initialize".to_string()),
            progress: Some(0.85),
        };
        let job_2 = CreateJob {
            step: Some("initialize".to_string()),
            progress: Some(0.65),
        };
        let job_3 = CreateJob {
            step: Some("initialize".to_string()),
            progress: Some(0.45),
        };

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (_, value, _) = admin_session.create_job(&app, json!(job_1)).await?;
        let _ = admin_session.create_job(&app, json!(job_2)).await?;
        let _ = admin_session.create_job(&app, json!(job_3)).await?;

        let deleted_id = value.get("id").and_then(|v| v.as_str()).unwrap();

        let (delete_status, _, _) = admin_session
            .delete(&app, &format!("/jobs/{deleted_id}"))
            .await?;

        let (_, value, _) = admin_session.get(&app, "/jobs").await?;
        let remaining_jobs = value.get("records").and_then(|v| v.as_array()).unwrap();

        assert!(delete_status.is_success(), "error response status");
        assert_eq!(remaining_jobs.len(), 2, "incorrect number of jobs");
        assert!(
            !remaining_jobs
                .iter()
                .any(|j| j.get("id").and_then(|v| v.as_str()).unwrap() == deleted_id),
            "deleted job found in remaining jobs"
        );

        Ok(())
    }
}
