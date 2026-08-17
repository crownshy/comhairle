use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        SqlxResultExt,
        proposal_section::{self, ProposalSection},
        translations::{TextContentId, TextFormat, new_translation},
    },
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "proposal_evaluation_proposal")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct Proposal {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub workflow_step_id: Uuid,
    pub title: TextContentId,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ProposalIden; 5] = [
    ProposalIden::Id,
    ProposalIden::WorkflowStepId,
    ProposalIden::Title,
    ProposalIden::CreatedAt,
    ProposalIden::UpdatedAt,
];

#[derive(Deserialize, JsonSchema, Debug)]
pub struct CreateProposal {
    pub title: String,
    /// Ordered list of section bodies (rich text). Each becomes a `ProposalSection`.
    pub sections: Vec<String>,
}

/// Creates a proposal and its ordered sections, returning both.
#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    workflow_step_id: &Uuid,
    new_proposal: &CreateProposal,
    locale: &str,
) -> Result<(Proposal, Vec<ProposalSection>), ComhairleError> {
    let title = new_translation(db, locale, &new_proposal.title, TextFormat::Plain).await?;

    let (sql, values) = Query::insert()
        .into_table(ProposalIden::Table)
        .columns([ProposalIden::WorkflowStepId, ProposalIden::Title])
        .values([(*workflow_step_id).into(), title.id.into()])?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let proposal: Proposal = query_as_with(&sql, values).fetch_one(db).await?;

    let mut sections = Vec::with_capacity(new_proposal.sections.len());
    for (index, body) in new_proposal.sections.iter().enumerate() {
        let section =
            proposal_section::create(db, &proposal.id, index as i32, body, locale).await?;
        sections.push(section);
    }

    Ok((proposal, sections))
}

#[instrument(err(Debug))]
pub async fn list_localized(
    db: &PgPool,
    workflow_step_id: &Uuid,
    locale: &str,
) -> Result<Vec<LocalizedProposal>, ComhairleError> {
    let query = Query::select()
        .from(ProposalIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ProposalIden::Table, col)))
        .and_where(
            Expr::col((ProposalIden::Table, ProposalIden::WorkflowStepId))
                .eq(workflow_step_id.to_owned()),
        )
        .order_by(
            (ProposalIden::Table, ProposalIden::CreatedAt),
            sea_query::Order::Asc,
        )
        .to_owned();

    let query = LocalizedProposal::query_to_localisation(query, locale);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let proposals = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(proposals)
}

#[instrument(err(Debug))]
pub async fn list(db: &PgPool, workflow_step_id: &Uuid) -> Result<Vec<Proposal>, ComhairleError> {
    let (sql, values) = Query::select()
        .from(ProposalIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ProposalIden::WorkflowStepId).eq(workflow_step_id.to_owned()))
        .order_by(ProposalIden::CreatedAt, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let proposals = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(proposals)
}

#[instrument(err(Debug))]
pub async fn list_with_translations(
    db: &PgPool,
    workflow_step_id: &Uuid,
    locale: &str,
) -> Result<Vec<ProposalWithTranslations>, ComhairleError> {
    let proposals = list(db, workflow_step_id).await?;
    let mut out = Vec::with_capacity(proposals.len());
    for proposal in proposals {
        out.push(ProposalWithTranslations::from_original(db, proposal, locale).await?);
    }
    Ok(out)
}

#[instrument(err(Debug))]
pub async fn get_localized_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedProposal, ComhairleError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (ProposalIden::Table, col)))
        .from(ProposalIden::Table)
        .and_where(Expr::col((ProposalIden::Table, ProposalIden::Id)).eq(id.to_owned()))
        .to_owned();

    let query = LocalizedProposal::query_to_localisation(query, locale);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let proposal = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Proposal")?;

    Ok(proposal)
}

/// Get a proposal by ID (original struct, not localized). Used by the sealed gate on
/// proposal responses to find the step, and from it the workflow, the seal is evaluated for.
#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Proposal, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (ProposalIden::Table, col)))
        .from(ProposalIden::Table)
        .and_where(Expr::col((ProposalIden::Table, ProposalIden::Id)).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let proposal = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Proposal")?;

    Ok(proposal)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Proposal, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ProposalIden::Table)
        .and_where(Expr::col(ProposalIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let proposal = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(proposal)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        models::model_test_helpers::{
            get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::prioritization_tool_config,
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_new_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;

        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal = CreateProposal {
            title: "Test proposal".to_string(),
            sections: vec!["Test proposal".to_string()],
        };
        let (proposal, sections) = create(&pool, &workflow_step.id, &create_proposal, "en").await?;

        assert_eq!(
            proposal.workflow_step_id, workflow_step.id,
            "incorrect workflow_step_id"
        );
        assert_eq!(sections.len(), 1, "expected one section");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_localized_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;

        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal = CreateProposal {
            title: "Test proposal".to_string(),
            sections: vec!["Test proposal body".to_string()],
        };
        let (new_proposal, _) = create(&pool, &workflow_step.id, &create_proposal, "en").await?;

        let proposal = get_localized_by_id(&pool, &new_proposal.id, "en").await?;

        assert_eq!(
            proposal.title,
            "Test proposal".to_string(),
            "incorrect title"
        );

        let sections = proposal_section::list_localized(&pool, &new_proposal.id, "en").await?;
        assert_eq!(sections.len(), 1, "incorrect number of sections");
        assert_eq!(
            sections[0].body,
            "Test proposal body".to_string(),
            "incorrect body"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_proposals(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;

        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal_1 = CreateProposal {
            title: "Proposal A".to_string(),
            sections: vec!["Proposal A".to_string()],
        };
        let create_proposal_2 = CreateProposal {
            title: "Proposal B".to_string(),
            sections: vec!["Proposal B".to_string()],
        };
        let create_proposal_3 = CreateProposal {
            title: "Proposal C".to_string(),
            sections: vec!["Proposal C".to_string()],
        };
        create(&pool, &workflow_step.id, &create_proposal_1, "en").await?;
        create(&pool, &workflow_step.id, &create_proposal_2, "en").await?;
        create(&pool, &workflow_step.id, &create_proposal_3, "en").await?;

        let proposals = list_localized(&pool, &workflow_step.id, "en").await?;

        assert_eq!(proposals.len(), 3, "incorrect number of proposals");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;

        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let create_proposal = CreateProposal {
            title: "Test proposal".to_string(),
            sections: vec!["Test proposal".to_string()],
        };
        let (proposal, _) = create(&pool, &workflow_step.id, &create_proposal, "en").await?;

        delete(&pool, &proposal.id).await?;

        let err = get_localized_by_id(&pool, &proposal.id, "en")
            .await
            .unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(message) => {
                assert!(message.contains("Proposal"))
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
