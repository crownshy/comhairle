use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, PgPool, Postgres,
    encode::IsNull,
    prelude::{FromRow, Type},
    query_as, query_as_with,
};
use sqlx_postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

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
        serde_json::to_value(self)
            .map(|json| <serde_json::Value as Encode<Postgres>>::size_hint(&json))
            .unwrap_or(0)
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
    pub value: ResponseValue,
}

/// A response value — either a numeric rating (likert / continuous) or
/// free-text. JSON wire format is untagged: `"value": 4.5` or `"value": "hi"`.
#[derive(PartialEq, Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseValue {
    Number(f64),
    Text(String),
}

impl From<f64> for ResponseValue {
    fn from(v: f64) -> Self {
        ResponseValue::Number(v)
    }
}

impl From<String> for ResponseValue {
    fn from(v: String) -> Self {
        ResponseValue::Text(v)
    }
}

const DEFAULT_COLUMNS: [ProposalResponseIden; 6] = [
    ProposalResponseIden::Id,
    ProposalResponseIden::ProposalId,
    ProposalResponseIden::UserId,
    ProposalResponseIden::Response,
    ProposalResponseIden::CreatedAt,
    ProposalResponseIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CreateResponse {
    pub question_responses: Vec<Response>,
}

/// Upsert a participant's response for a proposal. There is at most one row
/// per (proposal_id, user_id), so re-submitting overwrites the previous answer
/// instead of stacking duplicate rows.
#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    proposal_id: &Uuid,
    user_id: &Uuid,
    create_response: &CreateResponse,
) -> Result<ProposalResponse, ComhairleError> {
    let question_responses = serde_json::to_value(QuestionResponses(
        create_response.question_responses.clone(),
    ))?;

    let sql = r#"
        INSERT INTO proposal_evalution_proposal_response (proposal_id, user_id, response)
        VALUES ($1, $2, $3)
        ON CONFLICT (proposal_id, user_id) DO UPDATE
            SET response = EXCLUDED.response,
                updated_at = NOW()
        RETURNING id, proposal_id, user_id, response, created_at, updated_at
    "#;

    let response = query_as::<_, ProposalResponse>(sql)
        .bind(proposal_id)
        .bind(user_id)
        .bind(question_responses)
        .fetch_one(db)
        .await?;

    Ok(response)
}

#[derive(Deserialize, JsonSchema, Debug, Clone, Default)]
pub struct ProposalResponseFilterOptions;

#[derive(Deserialize, JsonSchema, Debug, Clone, Default)]
pub struct ProposalResponseOrderOptions;

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    proposal_id: &Uuid,
    filter_options: ProposalResponseFilterOptions,
    order_options: ProposalResponseOrderOptions,
) -> Result<Vec<ProposalResponse>, ComhairleError> {
    let query = Query::select()
        .from(ProposalResponseIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ProposalResponseIden::ProposalId).eq(proposal_id.to_owned()))
        .to_owned();

    // TODO: filtering and ordering

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let responses = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(responses)
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
            proposal::{self, CreateProposal},
            users,
        },
        routes::user::dto::UserDto,
        test_helpers::TEST_PASSWORD,
        tools::ToolConfig,
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_new_proposal_response(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
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
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_proposal_responses(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
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
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                },
            ],
        };
        let create_response_a_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5_f64.into(),
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2_f64.into(),
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_a.id, &create_response_a_a).await?;
        create(&pool, &proposal_b.id, &user_a.id, &create_response_a_b).await?;

        let user_b = users::create_annon_user(&pool).await?;
        let create_response_b_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                },
            ],
        };
        let create_response_b_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5_f64.into(),
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2_f64.into(),
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_b.id, &create_response_b_a).await?;
        create(&pool, &proposal_b.id, &user_b.id, &create_response_b_b).await?;

        let user_c = users::create_annon_user(&pool).await?;
        let create_response_c_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                },
            ],
        };
        let create_response_c_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5_f64.into(),
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2_f64.into(),
                },
            ],
        };
        create(&pool, &proposal_a.id, &user_c.id, &create_response_c_a).await?;
        create(&pool, &proposal_b.id, &user_c.id, &create_response_c_b).await?;

        let filter_options = ProposalResponseFilterOptions;
        let order_options = ProposalResponseOrderOptions;
        let proposal_a_responses = list(
            &pool,
            &proposal_a.id,
            filter_options.clone(),
            order_options.clone(),
        )
        .await?;

        let proposal_b_responses = list(
            &pool,
            &proposal_b.id,
            filter_options.clone(),
            order_options.clone(),
        )
        .await?;

        assert_eq!(proposal_a_responses.len(), 3, "incorrect a total");
        assert_eq!(proposal_b_responses.len(), 3, "incorrect b total");

        Ok(())
    }
}
