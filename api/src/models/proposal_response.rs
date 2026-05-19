use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    prelude::{FromRow, Type},
    query_as_with, Decode, Encode, PgPool, Postgres,
};
use sqlx_postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::pagination::{PageOptions, PaginatedResults},
};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "proposal_evalution_proposal_response")]
pub struct ProposalResponse {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub user_id: Uuid,
    pub response: QuestionResponses,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
pub struct QuestionResponses(pub Vec<Response>);

impl Type<Postgres> for QuestionResponses {
    fn type_info() -> PgTypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

impl PgHasArrayType for QuestionResponses {
    fn array_type_info() -> PgTypeInfo {
        <serde_json::Value as PgHasArrayType>::array_type_info()
    }
}

impl<'q> Encode<'q, Postgres> for QuestionResponses {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let json = serde_json::to_value(self)?;
        <serde_json::Value as Encode<Postgres>>::encode(json, buf)
    }

    fn size_hint(&self) -> usize {
        let json = serde_json::to_value(self).unwrap(); // TODO:
        <serde_json::Value as Encode<Postgres>>::size_hint(&json)
    }
}

impl<'r> Decode<'r, Postgres> for QuestionResponses {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let json: serde_json::Value = Decode::<Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

#[derive(PartialEq, Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct Response {
    pub question_id: Uuid,
    pub value: f64,
}

const DEFAULT_COLUMNS: [ProposalResponseIden; 6] = [
    ProposalResponseIden::Id,
    ProposalResponseIden::ProposalId,
    ProposalResponseIden::UserId,
    ProposalResponseIden::Response,
    ProposalResponseIden::CreatedAt,
    ProposalResponseIden::UpdatedAt,
];

#[derive(Deserialize, JsonSchema, Debug)]
pub struct CreateResponse {
    pub questions: Vec<Response>,
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    proposal_id: &Uuid,
    user_id: &Uuid,
    create_response: &CreateResponse,
) -> Result<ProposalResponse, ComhairleError> {
    let columns = vec![
        ProposalResponseIden::ProposalId,
        ProposalResponseIden::UserId,
        ProposalResponseIden::Response,
    ];

    let question_responses =
        serde_json::to_value(QuestionResponses(create_response.questions.clone()))?;

    let values = vec![
        (*proposal_id).into(),
        (*user_id).into(),
        question_responses.into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(ProposalResponseIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let response = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(response)
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct ProposalResponseFilterOptions;

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct ProposalResponseOrderOptions;

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    proposal_id: &Uuid,
    page_options: PageOptions,
    filter_options: ProposalResponseFilterOptions,
    order_options: ProposalResponseOrderOptions,
) -> Result<PaginatedResults<ProposalResponse>, ComhairleError> {
    let query = Query::select()
        .from(ProposalResponseIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ProposalResponseIden::ProposalId).eq(proposal_id.to_owned()))
        .to_owned();

    // TODO: filtering and ordering

    let responses = page_options.fetch_paginated_results(db, query).await?;

    Ok(responses)
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use serde_json::json;

    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
            proposal::{self, CreateProposal},
            users,
        },
        routes::{user::dto::UserDto, workflow_steps::dto::WorkflowStepDto},
        test_helpers::{UserSession, TEST_PASSWORD},
        tools::ToolConfig,
    };

    use super::*;

    use std::error::Error;

    async fn create_prioritization_workflow_step(
        app: &Router,
        session: &mut UserSession,
        conversation_id: &Uuid,
        workflow_id: &Uuid,
    ) -> Result<WorkflowStepDto, Box<dyn Error>> {
        let (_, value, _) = session
            .create_workflow_step(
                app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": {
                        "type": "prioritization",
                        "randomize_order": false,
                        "questions": [
                            {
                                "text": "How much do you agree?",
                                "type": {
                                    "likert_scale": {
                                        "categories": [
                                            { "value": -1.0, "label": "Strongly disagree" },
                                            { "value": -0.5, "label": "Disagree" },
                                            { "value": 0.0, "label": "Neutral" },
                                            { "value": 0.5, "label": "Agree" },
                                            { "value": 1.0, "label": "Strongly agree" },
                                        ]
                                    }
                                }
                            },
                            {
                                "text": "How much do you care?",
                                "type": {
                                    "continuous": {
                                        "label": "Care?",
                                        "sub_steps": 10
                                    }
                                }
                            },
                        ]
                    }
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        Ok(workflow_step)
    }

    #[sqlx::test]
    async fn should_create_new_proposal_response(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step =
            create_prioritization_workflow_step(&app, &mut session, &conversation_id, &workflow_id)
                .await?;

        let proposal = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "A new proposal".to_string(),
                body: "Test proposal".to_string(),
            },
            "en",
        )
        .await?;

        let (_, value, _) = session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;
        let user: UserDto = serde_json::from_value(value)?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::Prioritization(config) => config,
            _ => panic!("Incorrect tool_config type"),
        };

        let create_response = CreateResponse {
            questions: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: -1.0,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5,
                },
            ],
        };

        let proposal_response = create(&pool, &proposal.id, &user.id, &create_response).await?;

        assert_eq!(
            proposal_response.proposal_id, proposal.id,
            "incorrect proposal_id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_proposal_responses(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step =
            create_prioritization_workflow_step(&app, &mut session, &conversation_id, &workflow_id)
                .await?;

        let proposal_a = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal A".to_string(),
                body: "Proposal A".to_string(),
            },
            "en",
        )
        .await?;
        let proposal_b = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal B".to_string(),
                body: "Proposal B".to_string(),
            },
            "en",
        )
        .await?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::Prioritization(config) => config,
            _ => panic!("Incorrect tool_config type"),
        };

        let user_a = users::create_annon_user(&pool).await?;
        let create_response_a_a = CreateResponse {
            questions: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: -1.0,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5,
                },
            ],
        };
        let create_response_a_b = CreateResponse {
            questions: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2,
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_a.id, &create_response_a_a).await?;
        create(&pool, &proposal_b.id, &user_a.id, &create_response_a_b).await?;

        let user_b = users::create_annon_user(&pool).await?;
        let create_response_b_a = CreateResponse {
            questions: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: -1.0,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5,
                },
            ],
        };
        let create_response_b_b = CreateResponse {
            questions: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2,
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_b.id, &create_response_b_a).await?;
        create(&pool, &proposal_b.id, &user_b.id, &create_response_b_b).await?;

        let user_c = users::create_annon_user(&pool).await?;
        let create_response_c_a = CreateResponse {
            questions: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: -1.0,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5,
                },
            ],
        };
        let create_response_c_b = CreateResponse {
            questions: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2,
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_c.id, &create_response_c_a).await?;
        create(&pool, &proposal_b.id, &user_c.id, &create_response_c_b).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = ProposalResponseFilterOptions;
        let order_options = ProposalResponseOrderOptions;
        let proposal_a_responses = list(
            &pool,
            &proposal_a.id,
            page_options.clone(),
            filter_options.clone(),
            order_options.clone(),
        )
        .await?;

        let proposal_b_responses = list(
            &pool,
            &proposal_b.id,
            page_options.clone(),
            filter_options.clone(),
            order_options.clone(),
        )
        .await?;

        assert_eq!(proposal_a_responses.total, 3, "incorrect a total");
        assert_eq!(proposal_b_responses.total, 3, "incorrect b total");

        Ok(())
    }
}
