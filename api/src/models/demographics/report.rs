use std::collections::{HashMap, HashSet};

use sea_query::extension::postgres::PgExpr;
use sea_query::{
    Alias, Condition, Expr, Func, IntoColumnRef, JoinType, Order, PostgresQueryBuilder, Query,
    SelectStatement, SimpleExpr, UnionType,
};
use sea_query_binder::SqlxBinder;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::translations::{TextContentId, localize_translations};
use crate::models::user_participation::UserParticipationIden;
use crate::models::user_profile::UserProfileIden;
use crate::models::workflow::WorkflowIden;
use crate::models::workflow_step::WorkflowStepIden;

use super::VersionStatus;
use super::types::*;

/// The `type` tags of the demographic tool configs stored in `workflow_step.tool_config`.
const QUESTION_STEP_TYPE: &str = "demographicquestion";
const FORM_STEP_TYPE: &str = "demographicform";

/// Alias of the lateral subquery holding a question's latest published text.
fn latest() -> Alias {
    Alias::new("latest")
}

fn tool_config() -> Expr {
    Expr::col((WorkflowStepIden::Table, WorkflowStepIden::ToolConfig))
}

fn as_uuid(expr: SimpleExpr) -> SimpleExpr {
    expr.cast_as(Alias::new("uuid"))
}

/// The text content of the latest published version of the question whose id is `question_id`
/// in the enclosing query.
fn latest_published_text(question_id: impl IntoColumnRef) -> SelectStatement {
    let version = Alias::new("lv");
    Query::select()
        .column((
            version.clone(),
            DemographicQuestionVersionIden::TextContentId,
        ))
        .from_as(DemographicQuestionVersionIden::Table, version.clone())
        .and_where(
            Expr::col((version.clone(), DemographicQuestionVersionIden::QuestionId))
                .equals(question_id),
        )
        .and_where(
            Expr::col((version.clone(), DemographicQuestionVersionIden::Status))
                .eq(VersionStatus::Published),
        )
        .order_by(
            (version, DemographicQuestionVersionIden::VersionNumber),
            Order::Desc,
        )
        .limit(1)
        .to_owned()
}

/// The highest published version number of the form that `demographics_form_version` is
/// currently joined on.
fn latest_published_form_version_number() -> SimpleExpr {
    let latest_version = Alias::new("latest_version");
    let max_number = Query::select()
        .expr(Func::max(Expr::col((
            latest_version.clone(),
            DemographicFormVersionIden::VersionNumber,
        ))))
        .from_as(DemographicFormVersionIden::Table, latest_version.clone())
        .and_where(
            Expr::col((latest_version.clone(), DemographicFormVersionIden::FormId)).equals((
                DemographicFormVersionIden::Table,
                DemographicFormVersionIden::FormId,
            )),
        )
        .and_where(
            Expr::col((latest_version, DemographicFormVersionIden::Status))
                .eq(VersionStatus::Published),
        )
        .to_owned();
    SimpleExpr::SubQuery(None, Box::new(max_number.into_sub_query_statement()))
}

/// Question ids asked by any of the workflows. Form steps contribute the questions of the
/// version they pin, or of the latest published form version, minus questions the step disables.
///
/// The step configuration is JSONB, so its fields are read with the `->` and `->>` operators.
fn asked_question_ids(workflow_ids: &[Uuid]) -> SelectStatement {
    let in_workflows = || {
        Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId))
            .is_in(workflow_ids.to_vec())
    };

    let from_question_steps = Query::select()
        .expr_as(
            as_uuid(tool_config().cast_json_field("question_id")),
            Alias::new("question_id"),
        )
        .from(WorkflowStepIden::Table)
        .and_where(in_workflows())
        .and_where(tool_config().cast_json_field("type").eq(QUESTION_STEP_TYPE))
        .to_owned();

    let version_policy = || tool_config().get_json_field("version");
    let form_version = |column| (DemographicFormVersionIden::Table, column);
    let selects_form_version = Condition::all()
        .add(
            Expr::col(form_version(DemographicFormVersionIden::FormId))
                .eq(as_uuid(tool_config().cast_json_field("form_id"))),
        )
        .add(
            Expr::col(form_version(DemographicFormVersionIden::Status))
                .eq(VersionStatus::Published),
        )
        .add(
            Condition::any()
                .add(
                    Condition::all()
                        .add(version_policy().cast_json_field("type").eq("pinned"))
                        .add(
                            Expr::col(form_version(DemographicFormVersionIden::Id))
                                .eq(as_uuid(version_policy().cast_json_field("version_id"))),
                        ),
                )
                .add(
                    Condition::all()
                        .add(
                            Expr::expr(Func::coalesce([
                                version_policy().cast_json_field("type"),
                                Expr::val("latest").into(),
                            ]))
                            .eq("latest"),
                        )
                        .add(
                            Expr::col(form_version(DemographicFormVersionIden::VersionNumber))
                                .eq(latest_published_form_version_number()),
                        ),
                ),
        );

    let override_ = Alias::new("step_question");
    let disabled_override = Query::select()
        .expr(Expr::val(1))
        .from_function(
            Func::cust(Alias::new("jsonb_array_elements")).arg(Func::coalesce([
                tool_config().get_json_field("questions"),
                Expr::val(serde_json::json!([])).into(),
            ])),
            override_.clone(),
        )
        .and_where(
            as_uuid(Expr::col(override_.clone()).cast_json_field("question_id")).eq(Expr::col((
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::QuestionId,
            ))),
        )
        .and_where(
            Expr::col(override_)
                .cast_json_field("enabled")
                .cast_as(Alias::new("boolean"))
                .eq(false),
        )
        .to_owned();

    let from_form_steps = Query::select()
        .column((
            DemographicFormVersionQuestionIden::Table,
            DemographicFormVersionQuestionIden::QuestionId,
        ))
        .from(WorkflowStepIden::Table)
        .inner_join(DemographicFormVersionIden::Table, selects_form_version)
        .inner_join(
            DemographicFormVersionQuestionIden::Table,
            Expr::col((
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::FormVersionId,
            ))
            .equals(form_version(DemographicFormVersionIden::Id)),
        )
        .and_where(in_workflows())
        .and_where(tool_config().cast_json_field("type").eq(FORM_STEP_TYPE))
        .and_where(Expr::exists(disabled_override).not())
        .to_owned();

    let mut asked = from_question_steps;
    asked.union(UnionType::Distinct, from_form_steps);
    asked
}

/// The conversation a workflow belongs to. Answers count only in the conversation they were given in.
fn workflow_conversation(workflow_id: &Uuid) -> SelectStatement {
    Query::select()
        .column(WorkflowIden::ConversationId)
        .from(WorkflowIden::Table)
        .and_where(Expr::col(WorkflowIden::Id).eq(*workflow_id))
        .to_owned()
}

/// Counts answers from consented participants to the demographic questions a workflow asks.
/// Only answers to published versions of those questions are counted.
pub async fn for_workflow(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
) -> Result<WorkflowDemographics, ComhairleError> {
    let (sql, values) = Query::select()
        .expr(Func::count_distinct(Expr::col(
            UserParticipationIden::UserId,
        )))
        .from(UserParticipationIden::Table)
        .and_where(Expr::col(UserParticipationIden::WorkflowId).eq(*workflow_id))
        .build_sqlx(PostgresQueryBuilder);
    let total_participants: i64 = sqlx::query_scalar_with(&sql, values).fetch_one(db).await?;

    let participation = |column| (UserParticipationIden::Table, column);
    let response = |column| (DemographicResponseIden::Table, column);
    let version = |column| (DemographicQuestionVersionIden::Table, column);
    let text_content_id = (latest(), DemographicQuestionVersionIden::TextContentId);
    let count = Alias::new("count");
    let (sql, values) = Query::select()
        .expr_as(
            Expr::col(version(DemographicQuestionVersionIden::QuestionId)),
            Alias::new("question_id"),
        )
        .expr_as(
            Expr::col(text_content_id.clone()),
            Alias::new("text_content_id"),
        )
        .expr_as(
            Expr::col(response(DemographicResponseIden::Value)),
            Alias::new("value"),
        )
        .expr_as(
            Func::count_distinct(Expr::col(participation(UserParticipationIden::UserId))),
            count.clone(),
        )
        .from(UserParticipationIden::Table)
        .inner_join(
            UserProfileIden::Table,
            Expr::col((UserProfileIden::Table, UserProfileIden::UserId))
                .equals(participation(UserParticipationIden::UserId))
                .and(Expr::col((UserProfileIden::Table, UserProfileIden::Consented)).eq(true)),
        )
        .inner_join(
            DemographicResponseIden::Table,
            Expr::col(response(DemographicResponseIden::UserId))
                .equals(participation(UserParticipationIden::UserId)),
        )
        .inner_join(
            DemographicQuestionVersionIden::Table,
            Expr::col(version(DemographicQuestionVersionIden::Id))
                .equals(response(DemographicResponseIden::QuestionVersionId))
                .and(
                    Expr::col(version(DemographicQuestionVersionIden::Status))
                        .eq(VersionStatus::Published),
                ),
        )
        .join_lateral(
            JoinType::InnerJoin,
            latest_published_text(version(DemographicQuestionVersionIden::QuestionId)),
            latest(),
            SimpleExpr::from(Expr::val(true)),
        )
        .and_where(Expr::col(participation(UserParticipationIden::WorkflowId)).eq(*workflow_id))
        .and_where(
            Expr::col(response(DemographicResponseIden::ConversationId))
                .in_subquery(workflow_conversation(workflow_id)),
        )
        .and_where(
            Expr::col(version(DemographicQuestionVersionIden::QuestionId))
                .in_subquery(asked_question_ids(&[*workflow_id])),
        )
        .add_group_by([
            Expr::col(version(DemographicQuestionVersionIden::QuestionId)).into(),
            Expr::col(text_content_id).into(),
            Expr::col(response(DemographicResponseIden::Value)).into(),
        ])
        .order_by(Alias::new("question_id"), Order::Asc)
        .order_by(count, Order::Desc)
        .build_sqlx(PostgresQueryBuilder);
    let rows: Vec<ReportRow> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    let text_ids: Vec<TextContentId> = rows
        .iter()
        .map(|row| row.question.text_content_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let texts = localize_translations(db, &text_ids, locale).await?;

    let mut questions: HashMap<Uuid, QuestionCounts> = HashMap::new();
    for row in rows {
        let text = texts
            .get(&row.question.text_content_id)
            .cloned()
            .unwrap_or_default();
        let entry = questions
            .entry(row.question.question_id)
            .or_insert_with(|| QuestionCounts {
                question: QuestionText {
                    question_id: row.question.question_id,
                    text,
                },
                answers: Vec::new(),
            });
        entry.answers.push(AnswerCount {
            value: row.value,
            count: row.count,
        });
    }

    Ok(WorkflowDemographics {
        total_participants,
        questions,
    })
}

/// Exports the answers of consented participants across all workflows of a conversation.
pub async fn export_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
    locale: &str,
) -> Result<ConversationDemographicsExport, ComhairleError> {
    let (sql, values) = Query::select()
        .column(WorkflowIden::Id)
        .from(WorkflowIden::Table)
        .and_where(Expr::col(WorkflowIden::ConversationId).eq(*conversation_id))
        .build_sqlx(PostgresQueryBuilder);
    let workflow_ids: Vec<Uuid> = sqlx::query_scalar_with(&sql, values).fetch_all(db).await?;

    let (sql, values) = Query::select()
        .distinct()
        .column((UserProfileIden::Table, UserProfileIden::UserId))
        .expr_as(
            Expr::col((UserProfileIden::Table, UserProfileIden::CreatedAt)),
            Alias::new("profile_created_at"),
        )
        .from(UserProfileIden::Table)
        .inner_join(
            UserParticipationIden::Table,
            Expr::col((UserParticipationIden::Table, UserParticipationIden::UserId))
                .equals((UserProfileIden::Table, UserProfileIden::UserId)),
        )
        .and_where(
            Expr::col((
                UserParticipationIden::Table,
                UserParticipationIden::WorkflowId,
            ))
            .is_in(workflow_ids.clone()),
        )
        .and_where(Expr::col((UserProfileIden::Table, UserProfileIden::Consented)).eq(true))
        .order_by(
            (UserProfileIden::Table, UserProfileIden::CreatedAt),
            Order::Desc,
        )
        .build_sqlx(PostgresQueryBuilder);
    let participants: Vec<ExportParticipantRow> =
        sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    let question_id = (QuestionRowIden::Table, QuestionRowIden::Id);
    let (sql, values) = Query::select()
        .expr_as(Expr::col(question_id), Alias::new("question_id"))
        .column((latest(), DemographicQuestionVersionIden::TextContentId))
        .from(QuestionRowIden::Table)
        .join_lateral(
            JoinType::InnerJoin,
            latest_published_text(question_id),
            latest(),
            SimpleExpr::from(Expr::val(true)),
        )
        .and_where(Expr::col(question_id).in_subquery(asked_question_ids(&workflow_ids)))
        .build_sqlx(PostgresQueryBuilder);
    let question_rows: Vec<QuestionTextRow> =
        sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    // The latest answer wins when a user has answered several versions of one question.
    // Special category answers only count while consent is still granted.
    let profile = |column| (UserProfileIden::Table, column);
    let response = |column| (DemographicResponseIden::Table, column);
    let version = |column| (DemographicQuestionVersionIden::Table, column);
    let consent = |column| (DemographicConsentIden::Table, column);
    let (sql, values) = Query::select()
        .distinct_on([
            profile(UserProfileIden::UserId).into_column_ref(),
            version(DemographicQuestionVersionIden::QuestionId).into_column_ref(),
        ])
        .column(profile(UserProfileIden::UserId))
        .column(version(DemographicQuestionVersionIden::QuestionId))
        .column(response(DemographicResponseIden::Value))
        .from(UserProfileIden::Table)
        .inner_join(
            UserParticipationIden::Table,
            Expr::col((UserParticipationIden::Table, UserParticipationIden::UserId))
                .equals(profile(UserProfileIden::UserId)),
        )
        .inner_join(
            DemographicResponseIden::Table,
            Expr::col(response(DemographicResponseIden::UserId))
                .equals(profile(UserProfileIden::UserId)),
        )
        .inner_join(
            DemographicQuestionVersionIden::Table,
            Expr::col(version(DemographicQuestionVersionIden::Id))
                .equals(response(DemographicResponseIden::QuestionVersionId))
                .and(
                    Expr::col(version(DemographicQuestionVersionIden::Status))
                        .eq(VersionStatus::Published),
                ),
        )
        .join(
            JoinType::LeftJoin,
            DemographicConsentIden::Table,
            Expr::col(consent(DemographicConsentIden::QuestionVersionId))
                .equals(response(DemographicResponseIden::QuestionVersionId))
                .and(
                    Expr::col(consent(DemographicConsentIden::UserId))
                        .equals(response(DemographicResponseIden::UserId)),
                ),
        )
        .and_where(
            Expr::col((
                UserParticipationIden::Table,
                UserParticipationIden::WorkflowId,
            ))
            .is_in(workflow_ids.clone()),
        )
        .and_where(Expr::col(profile(UserProfileIden::Consented)).eq(true))
        .and_where(
            Expr::col(response(DemographicResponseIden::ConversationId)).eq(*conversation_id),
        )
        .and_where(
            Expr::col(version(DemographicQuestionVersionIden::QuestionId))
                .in_subquery(asked_question_ids(&workflow_ids)),
        )
        .and_where(
            Expr::col(version(DemographicQuestionVersionIden::SpecialCategory))
                .not()
                .or(Expr::col(consent(DemographicConsentIden::Granted)).eq(true)),
        )
        .order_by(profile(UserProfileIden::UserId), Order::Asc)
        .order_by(
            version(DemographicQuestionVersionIden::QuestionId),
            Order::Asc,
        )
        .order_by(response(DemographicResponseIden::CreatedAt), Order::Desc)
        .build_sqlx(PostgresQueryBuilder);
    let answer_rows: Vec<ExportAnswerRow> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    let text_ids: Vec<TextContentId> = question_rows.iter().map(|q| q.text_content_id).collect();
    let texts = localize_translations(db, &text_ids, locale).await?;

    let mut questions: Vec<QuestionText> = question_rows
        .iter()
        .map(|q| QuestionText {
            question_id: q.question_id,
            text: texts.get(&q.text_content_id).cloned().unwrap_or_default(),
        })
        .collect();
    questions.sort_by(|a, b| a.text.cmp(&b.text).then(a.question_id.cmp(&b.question_id)));

    let mut answers_by_user: HashMap<Uuid, HashMap<Uuid, serde_json::Value>> = HashMap::new();
    for row in answer_rows {
        answers_by_user
            .entry(row.user_id)
            .or_default()
            .insert(row.question_id, row.value);
    }

    let participants = participants
        .into_iter()
        .map(|p| ExportParticipant {
            answers: answers_by_user.remove(&p.user_id).unwrap_or_default(),
            user_id: p.user_id,
            profile_created_at: p.profile_created_at,
        })
        .collect();

    Ok(ConversationDemographicsExport {
        questions,
        participants,
    })
}
