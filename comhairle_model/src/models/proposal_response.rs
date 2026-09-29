use crate::models::error::ModelError;
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
    /// When set, this answer is for a section question about the given section.
    /// Absent (proposal-level) answers apply to the proposal as a whole. Kept
    /// optional so responses stored before per-section questions still decode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_id: Option<Uuid>,
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
#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    proposal_id: &Uuid,
    user_id: &Uuid,
    create_response: &CreateResponse,
) -> Result<ProposalResponse, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    proposal_id: &Uuid,
    filter_options: ProposalResponseFilterOptions,
    order_options: ProposalResponseOrderOptions,
) -> Result<Vec<ProposalResponse>, ModelError> {
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
