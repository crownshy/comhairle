//! Public report data for the Polis section blocks (ADR-0061).
//!
//! One endpoint per embeddable section block, each returning only what that block draws.
//! `PolisGetReportData` stays the admin view and is never called from the public report.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use axum::extract::{Json, Query, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use super::PolisToolConfig;
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::polis_statement_aux::{self, PolisStatementAuxFilterOptions};
use crate::models::users::User;
use crate::routes::auth::RequiredUser;
use crate::tools::ToolConfig;
use crate::tools::public_report_data::anonymous::{
    Count, FacilitatorText, ModeratedText, OpinionGroupId, PublicReportData, Share, StatementId,
    public_report_data,
};
use crate::tools::public_report_data::{serve_public_report_data, serve_report_data_preview};
use crate::wiki_poll_service::ReportScope;
use crate::wiki_poll_service::polis_service::{
    CommentReportData, GroupVoteCounts, VoteCounts, WikiPollReport,
};

/// Public report data never shows pending statements, so this does not follow the step's
/// `strict_moderation` the way `PolisGetReportData` does.
const PUBLIC_SCOPE: ReportScope = ReportScope::AcceptedOnly;

/// Section block types, as the report stores them (`lib/reports/embeds.ts`).
const KEY_STATS: &str = "polis-key-stats";
const CONSENSUS: &str = "polis-area-consensus";
const DISAGREEMENT: &str = "polis-area-disagreement";
const CONSENSUS_CONTINUUM: &str = "polis-consensus-continuum";
const OPINION_GROUPS: &str = "polis-opinion-groups";

public_report_data! {
    #[derive(PartialEq)]
    pub struct PolisPublicKeyStats {
        /// People Polis has placed in an opinion group.
        pub participants: Count,
        pub statements: Count,
        pub votes: Count,
    }
}

public_report_data! {
    pub struct PolisPublicVotes {
        pub agrees: Count,
        pub disagrees: Count,
        pub passes: Count,
    }
}

public_report_data! {
    pub struct PolisPublicGroupVotes {
        pub group_id: OpinionGroupId,
        pub agrees: Count,
        pub disagrees: Count,
        pub passes: Count,
    }
}

public_report_data! {
    /// An opinion group's size, which the vote bars are drawn as a share of.
    pub struct PolisPublicGroup {
        pub group_id: OpinionGroupId,
        pub total_members: Count,
    }
}

public_report_data! {
    /// A statement with its vote bars.
    pub struct PolisPublicStatement {
        pub tid: StatementId,
        pub text: ModeratedText,
        pub themes: Vec<FacilitatorText>,
        pub overall_votes: PolisPublicVotes,
        pub group_votes: Vec<PolisPublicGroupVotes>,
    }
}

public_report_data! {
    /// A statement in a ranked list, which can be filtered to seed or participant statements.
    pub struct PolisPublicRankedStatement {
        pub tid: StatementId,
        pub text: ModeratedText,
        pub themes: Vec<FacilitatorText>,
        pub overall_votes: PolisPublicVotes,
        pub group_votes: Vec<PolisPublicGroupVotes>,
        pub is_seed: bool,
    }
}

public_report_data! {
    /// Statements ranked highest first, for the areas of consensus and disagreement.
    pub struct PolisPublicRankedStatements {
        pub statements: Vec<PolisPublicRankedStatement>,
        pub groups: Vec<PolisPublicGroup>,
    }
}

public_report_data! {
    /// A statement Polis has placed on the consensus continuum.
    pub struct PolisPublicPlacedStatement {
        pub tid: StatementId,
        pub text: ModeratedText,
        pub themes: Vec<FacilitatorText>,
        pub overall_votes: PolisPublicVotes,
        pub group_votes: Vec<PolisPublicGroupVotes>,
        /// Divisiveness as a share of the most divisive statement's, which is all the
        /// continuum's axis needs.
        pub divisiveness: Share,
    }
}

public_report_data! {
    pub struct PolisPublicConsensusContinuum {
        pub statements: Vec<PolisPublicPlacedStatement>,
        pub groups: Vec<PolisPublicGroup>,
    }
}

public_report_data! {
    #[derive(PartialEq)]
    pub struct PolisPublicRepresentativeStatement {
        pub tid: StatementId,
    }
}

public_report_data! {
    pub struct PolisPublicOpinionGroup {
        pub group_id: OpinionGroupId,
        pub total_members: Count,
        /// Points into `PolisPublicOpinionGroups::statements`.
        pub representative_comments: Vec<PolisPublicRepresentativeStatement>,
    }
}

public_report_data! {
    pub struct PolisPublicOpinionGroups {
        pub groups: Vec<PolisPublicOpinionGroup>,
        /// Only the statements some group is represented by.
        pub statements: Vec<PolisPublicStatement>,
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct PublicReportQuery {
    pub workflow_step_id: Uuid,
}

pub fn routes() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/polis/public_report/key_stats",
            get_with(get_key_stats, |op| {
                op.id("PolisGetPublicKeyStats")
                    .tag("Tools")
                    .summary("Key stats for a published report")
                    .description(
                        "Participant, accepted statement and vote counts for a Polis step. 404 unless the conversation's published report embeds this \
                         section block for the step.",
                    )
                    .response::<200, Json<PolisPublicKeyStats>>()
            }),
        )
        .api_route(
            "/polis/report_preview/key_stats",
            get_with(preview_key_stats, |op| {
                op.id("PolisPreviewKeyStats")
                    .tag("Tools")
                    .summary("Key stats as the published report will show them")
                    .description(
                        "The same data as PolisGetPublicKeyStats, for facilitators who can \
                         update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PolisPublicKeyStats>>()
            }),
        )
        .api_route(
            "/polis/public_report/consensus",
            get_with(get_consensus, |op| {
                op.id("PolisGetPublicConsensus")
                    .tag("Tools")
                    .summary("Areas of consensus for a published report")
                    .description(
                        "Accepted statements ranked by group informed consensus, highest first. 404 unless the conversation's published report embeds this \
                         section block for the step.",
                    )
                    .response::<200, Json<PolisPublicRankedStatements>>()
            }),
        )
        .api_route(
            "/polis/report_preview/consensus",
            get_with(preview_consensus, |op| {
                op.id("PolisPreviewConsensus")
                    .tag("Tools")
                    .summary("Areas of consensus as the published report will show them")
                    .description(
                        "The same data as PolisGetPublicConsensus, for facilitators who can \
                         update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PolisPublicRankedStatements>>()
            }),
        )
        .api_route(
            "/polis/public_report/disagreement",
            get_with(get_disagreement, |op| {
                op.id("PolisGetPublicDisagreement")
                    .tag("Tools")
                    .summary("Areas of disagreement for a published report")
                    .description(
                        "Accepted statements ranked by divisiveness, highest first. 404 unless the conversation's published report embeds this \
                         section block for the step.",
                    )
                    .response::<200, Json<PolisPublicRankedStatements>>()
            }),
        )
        .api_route(
            "/polis/report_preview/disagreement",
            get_with(preview_disagreement, |op| {
                op.id("PolisPreviewDisagreement")
                    .tag("Tools")
                    .summary("Areas of disagreement as the published report will show them")
                    .description(
                        "The same data as PolisGetPublicDisagreement, for facilitators who can \
                         update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PolisPublicRankedStatements>>()
            }),
        )
        .api_route(
            "/polis/public_report/consensus_continuum",
            get_with(get_consensus_continuum, |op| {
                op.id("PolisGetPublicConsensusContinuum")
                    .tag("Tools")
                    .summary("Consensus continuum for a published report")
                    .description(
                        "Accepted statements Polis has given a divisiveness score. 404 unless the conversation's published report embeds this \
                         section block for the step.",
                    )
                    .response::<200, Json<PolisPublicConsensusContinuum>>()
            }),
        )
        .api_route(
            "/polis/report_preview/consensus_continuum",
            get_with(preview_consensus_continuum, |op| {
                op.id("PolisPreviewConsensusContinuum")
                    .tag("Tools")
                    .summary("Consensus continuum as the published report will show them")
                    .description(
                        "The same data as PolisGetPublicConsensusContinuum, for facilitators who can \
                         update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PolisPublicConsensusContinuum>>()
            }),
        )
        .api_route(
            "/polis/public_report/opinion_groups",
            get_with(get_opinion_groups, |op| {
                op.id("PolisGetPublicOpinionGroups")
                    .tag("Tools")
                    .summary("Opinion groups for a published report")
                    .description(
                        "Each opinion group's size and accepted representative statements. 404 unless the conversation's published report embeds this \
                         section block for the step.",
                    )
                    .response::<200, Json<PolisPublicOpinionGroups>>()
            }),
        )
        .api_route(
            "/polis/report_preview/opinion_groups",
            get_with(preview_opinion_groups, |op| {
                op.id("PolisPreviewOpinionGroups")
                    .tag("Tools")
                    .summary("Opinion groups as the published report will show them")
                    .description(
                        "The same data as PolisGetPublicOpinionGroups, for facilitators who can \
                         update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PolisPublicOpinionGroups>>()
            }),
        )
}

#[instrument(err(Debug), skip(state))]
async fn get_key_stats(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicKeyStats>, ComhairleError> {
    serve_public(&state, workflow_step_id, KEY_STATS, build_key_stats).await
}

#[instrument(err(Debug), skip(state))]
async fn preview_key_stats(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicKeyStats>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_key_stats).await
}

#[instrument(err(Debug), skip(state))]
async fn get_consensus(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicRankedStatements>, ComhairleError> {
    serve_public(&state, workflow_step_id, CONSENSUS, build_consensus).await
}

#[instrument(err(Debug), skip(state))]
async fn preview_consensus(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicRankedStatements>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_consensus).await
}

#[instrument(err(Debug), skip(state))]
async fn get_disagreement(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicRankedStatements>, ComhairleError> {
    serve_public(&state, workflow_step_id, DISAGREEMENT, build_disagreement).await
}

#[instrument(err(Debug), skip(state))]
async fn preview_disagreement(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicRankedStatements>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_disagreement).await
}

#[instrument(err(Debug), skip(state))]
async fn get_consensus_continuum(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicConsensusContinuum>, ComhairleError> {
    serve_public(
        &state,
        workflow_step_id,
        CONSENSUS_CONTINUUM,
        build_consensus_continuum,
    )
    .await
}

#[instrument(err(Debug), skip(state))]
async fn preview_consensus_continuum(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicConsensusContinuum>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_consensus_continuum).await
}

#[instrument(err(Debug), skip(state))]
async fn get_opinion_groups(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicOpinionGroups>, ComhairleError> {
    serve_public(
        &state,
        workflow_step_id,
        OPINION_GROUPS,
        build_opinion_groups,
    )
    .await
}

#[instrument(err(Debug), skip(state))]
async fn preview_opinion_groups(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<PolisPublicOpinionGroups>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_opinion_groups).await
}

async fn serve_public<Data: PublicReportData>(
    state: &ComhairleState,
    workflow_step_id: Uuid,
    section_block: &str,
    build: impl AsyncFnOnce(&ComhairleState, &Uuid, PolisToolConfig) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_public_report_data(
        &state.db,
        &workflow_step_id,
        section_block,
        None,
        polis_config,
        async |config| build(state, &workflow_step_id, config).await,
    )
    .await?;
    Ok(Json(data))
}

async fn serve_preview<Data: PublicReportData>(
    state: &Arc<ComhairleState>,
    user: &User,
    workflow_step_id: Uuid,
    build: impl AsyncFnOnce(&ComhairleState, &Uuid, PolisToolConfig) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_report_data_preview(
        state,
        user,
        &workflow_step_id,
        polis_config,
        async |config| build(state, &workflow_step_id, config).await,
    )
    .await?;
    Ok(Json(data))
}

async fn build_key_stats(
    state: &ComhairleState,
    _workflow_step_id: &Uuid,
    config: PolisToolConfig,
) -> Result<PolisPublicKeyStats, ComhairleError> {
    Ok(key_stats(&load_report(state, &config).await?))
}

async fn build_consensus(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: PolisToolConfig,
) -> Result<PolisPublicRankedStatements, ComhairleError> {
    let (report, themes) = load_report_and_themes(state, workflow_step_id, &config).await?;
    Ok(ranked_statements(&report, &themes, |c| {
        c.group_informed_consensus
    }))
}

async fn build_disagreement(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: PolisToolConfig,
) -> Result<PolisPublicRankedStatements, ComhairleError> {
    let (report, themes) = load_report_and_themes(state, workflow_step_id, &config).await?;
    Ok(ranked_statements(&report, &themes, |c| c.divisiveness))
}

async fn build_consensus_continuum(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: PolisToolConfig,
) -> Result<PolisPublicConsensusContinuum, ComhairleError> {
    let (report, themes) = load_report_and_themes(state, workflow_step_id, &config).await?;
    Ok(consensus_continuum(&report, &themes))
}

async fn build_opinion_groups(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: PolisToolConfig,
) -> Result<PolisPublicOpinionGroups, ComhairleError> {
    let (report, themes) = load_report_and_themes(state, workflow_step_id, &config).await?;
    Ok(opinion_groups(&report, &themes))
}

fn polis_config(config: ToolConfig) -> Option<PolisToolConfig> {
    match config {
        ToolConfig::Polis(polis) => Some(polis),
        _ => None,
    }
}

/// The step's report at `PUBLIC_SCOPE`. Goes through the Polis client's report cache, so the
/// section blocks of one step share an upstream fetch.
async fn load_report(
    state: &ComhairleState,
    config: &PolisToolConfig,
) -> Result<WikiPollReport, ComhairleError> {
    Ok(state
        .wiki_poll_service
        .get_report_data(&config.poll_id, PUBLIC_SCOPE)
        .await?)
}

async fn load_report_and_themes(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: &PolisToolConfig,
) -> Result<(WikiPollReport, Themes), ComhairleError> {
    let report = load_report(state, config).await?;
    let themes = load_themes(state, workflow_step_id, config).await?;
    Ok((report, themes))
}

/// Facilitator-set theme tags per statement id.
type Themes = HashMap<u32, Vec<String>>;

async fn load_themes(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: &PolisToolConfig,
) -> Result<Themes, ComhairleError> {
    let rows = polis_statement_aux::list(
        &state.db,
        Some(*workflow_step_id),
        Some(config.poll_id.clone()),
        PolisStatementAuxFilterOptions::default(),
    )
    .await?;
    Ok(rows
        .into_iter()
        .filter(|row| !row.themes.is_empty())
        .filter_map(|row| Some((u32::try_from(row.polis_statement_id).ok()?, row.themes)))
        .collect())
}

fn key_stats(report: &WikiPollReport) -> PolisPublicKeyStats {
    PolisPublicKeyStats {
        participants: Count(report.groups.iter().map(|g| g.total_members).sum()),
        statements: Count(report.comments.len() as u64),
        votes: Count(
            report
                .comments
                .iter()
                .map(|c| {
                    let votes = &c.overall_votes;
                    u64::from(votes.agrees) + u64::from(votes.disagrees) + u64::from(votes.passes)
                })
                .sum(),
        ),
    }
}

/// Every statement, highest `score` first. A statement Polis hasn't scored sorts as zero.
fn ranked_statements(
    report: &WikiPollReport,
    themes: &Themes,
    score: impl Fn(&CommentReportData) -> Option<f64>,
) -> PolisPublicRankedStatements {
    let mut comments: Vec<&CommentReportData> = report.comments.iter().collect();
    comments.sort_by(|a, b| descending(score(a), score(b)));
    PolisPublicRankedStatements {
        statements: comments
            .into_iter()
            .map(|c| PolisPublicRankedStatement {
                tid: StatementId(c.tid),
                text: accepted_text(c),
                themes: themes_of(themes, c.tid),
                overall_votes: votes(&c.overall_votes),
                group_votes: group_votes(&c.group_votes),
                is_seed: c.is_seed,
            })
            .collect(),
        groups: group_sizes(report),
    }
}

fn descending(a: Option<f64>, b: Option<f64>) -> Ordering {
    b.unwrap_or(0.0).total_cmp(&a.unwrap_or(0.0))
}

fn consensus_continuum(report: &WikiPollReport, themes: &Themes) -> PolisPublicConsensusContinuum {
    let scored: Vec<(&CommentReportData, f64)> = report
        .comments
        .iter()
        .filter_map(|c| Some((c, c.divisiveness.filter(|d| d.is_finite() && *d >= 0.0)?)))
        .collect();
    let most_divisive = scored.iter().map(|(_, d)| *d).fold(0.0, f64::max);
    PolisPublicConsensusContinuum {
        statements: scored
            .into_iter()
            .map(|(c, divisiveness)| PolisPublicPlacedStatement {
                tid: StatementId(c.tid),
                text: accepted_text(c),
                themes: themes_of(themes, c.tid),
                overall_votes: votes(&c.overall_votes),
                group_votes: group_votes(&c.group_votes),
                divisiveness: Share(if most_divisive > 0.0 {
                    divisiveness / most_divisive
                } else {
                    0.0
                }),
            })
            .collect(),
        groups: group_sizes(report),
    }
}

/// Representative statements are checked against the report's own statements again, so one
/// the scope left out can't come back through a group.
fn opinion_groups(report: &WikiPollReport, themes: &Themes) -> PolisPublicOpinionGroups {
    let reported: HashSet<u32> = report.comments.iter().map(|c| c.tid).collect();
    let mut represented: HashSet<u32> = HashSet::new();
    let groups: Vec<PolisPublicOpinionGroup> = report
        .groups
        .iter()
        .map(|g| PolisPublicOpinionGroup {
            group_id: OpinionGroupId(g.group_id),
            total_members: Count(g.total_members),
            representative_comments: g
                .representative_comments
                .iter()
                .filter(|r| reported.contains(&r.tid))
                .inspect(|r| {
                    represented.insert(r.tid);
                })
                .map(|r| PolisPublicRepresentativeStatement {
                    tid: StatementId(r.tid),
                })
                .collect(),
        })
        .collect();
    PolisPublicOpinionGroups {
        statements: report
            .comments
            .iter()
            .filter(|c| represented.contains(&c.tid))
            .map(|c| PolisPublicStatement {
                tid: StatementId(c.tid),
                text: accepted_text(c),
                themes: themes_of(themes, c.tid),
                overall_votes: votes(&c.overall_votes),
                group_votes: group_votes(&c.group_votes),
            })
            .collect(),
        groups,
    }
}

/// Only valid for reports loaded at `PUBLIC_SCOPE`, which leaves out every statement
/// moderation hasn't accepted.
fn accepted_text(comment: &CommentReportData) -> ModeratedText {
    ModeratedText::from_accepted(comment.text.clone())
}

fn votes(counts: &VoteCounts) -> PolisPublicVotes {
    PolisPublicVotes {
        agrees: Count(counts.agrees.into()),
        disagrees: Count(counts.disagrees.into()),
        passes: Count(counts.passes.into()),
    }
}

fn group_votes(counts: &[GroupVoteCounts]) -> Vec<PolisPublicGroupVotes> {
    counts
        .iter()
        .map(|g| PolisPublicGroupVotes {
            group_id: OpinionGroupId(g.group_id),
            agrees: Count(g.agrees.into()),
            disagrees: Count(g.disagrees.into()),
            passes: Count(g.passes.into()),
        })
        .collect()
}

fn group_sizes(report: &WikiPollReport) -> Vec<PolisPublicGroup> {
    report
        .groups
        .iter()
        .map(|g| PolisPublicGroup {
            group_id: OpinionGroupId(g.group_id),
            total_members: Count(g.total_members),
        })
        .collect()
}

fn themes_of(themes: &Themes, tid: u32) -> Vec<FacilitatorText> {
    themes
        .get(&tid)
        .into_iter()
        .flatten()
        .cloned()
        .map(FacilitatorText::from_facilitator)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;

    use super::*;
    use crate::models::report::{self, PartialReport};
    use crate::models::translations::{
        UpdateTextTranslation, get_text_translation_by_content_and_locale, update_text_translation,
    };
    use crate::models::workflow_step::{self, PartialWorkflowStep};
    use crate::setup_server;
    use crate::test_helpers::{UserSession, extract, polis_tool_config, test_state};
    use crate::wiki_poll_service::polis_service::{GroupReportData, RepresentativeComment};
    use crate::wiki_poll_service::{MockWikiPollService, WikiPoll, WikiPollService};

    /// Each endpoint's path and the section block type it serves.
    const SECTION_BLOCKS: [(&str, &str); 5] = [
        ("key_stats", KEY_STATS),
        ("consensus", CONSENSUS),
        ("disagreement", DISAGREEMENT),
        ("consensus_continuum", CONSENSUS_CONTINUUM),
        ("opinion_groups", OPINION_GROUPS),
    ];
    const PENDING_TEXT: &str = "a pending statement";

    fn statement(tid: u32, text: &str, divisiveness: Option<f64>) -> CommentReportData {
        CommentReportData {
            tid,
            text: text.into(),
            overall_votes: VoteCounts {
                agrees: 2,
                disagrees: 1,
                passes: 1,
            },
            group_votes: vec![],
            group_informed_consensus: divisiveness.map(|d| 1.0 - d),
            divisiveness,
            is_seed: false,
        }
    }

    fn group(group_id: u32, total_members: u64, representative_tids: &[u32]) -> GroupReportData {
        GroupReportData {
            group_id,
            representative_comments: representative_tids
                .iter()
                .map(|&tid| RepresentativeComment {
                    tid,
                    text: format!("statement {tid}"),
                })
                .collect(),
            members: vec![],
            total_members,
        }
    }

    /// What the Polis client returns: the pending statement only comes back, as a statement
    /// and as group 0's representative, when the scope admits it.
    fn report_for(scope: ReportScope) -> WikiPollReport {
        let mut comments = vec![statement(1, "an accepted statement", Some(0.2))];
        let mut representatives = vec![1];
        if scope == ReportScope::AcceptedAndPending {
            comments.push(statement(2, PENDING_TEXT, Some(0.9)));
            representatives.push(2);
        }
        WikiPollReport {
            comments,
            groups: vec![group(0, 3, &representatives)],
            participants: vec![],
        }
    }

    fn polis_service_serving_reports() -> Arc<dyn WikiPollService> {
        let mut service = MockWikiPollService::new();
        service
            .expect_create_random_admin_user()
            .returning(|| Box::pin(async { Ok(("u@mock.com".to_string(), "pw".to_string())) }));
        service
            .expect_login()
            .returning(|_| Box::pin(async { Ok("cookie".to_string()) }));
        service
            .expect_create_poll()
            .returning(|_| Box::pin(async { Ok("test_poll_id".to_string()) }));
        service
            .expect_update_poll_config()
            .returning(|_, _, _| Box::pin(async { Ok(WikiPoll::default()) }));
        service.expect_get_report_data().returning(|_, scope| {
            let report = report_for(scope);
            Box::pin(async move { Ok(report) })
        });
        Arc::new(service)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn pending_statements_never_reach_the_public_report(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state()
            .db(pool.clone())
            .wiki_poll_service(polis_service_serving_reports())
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;

        let (_, conversation, _) = admin.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);
        let (_, workflow, _) = admin.create_random_workflow(&app, &conversation_id).await?;
        let workflow_id: String = extract("id", &workflow);
        let (_, step, _) = admin
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config(),
                })
                .to_string()
                .into(),
            )
            .await?;
        let step_id = Uuid::parse_str(&extract::<String>("id", &step))?;
        let workflow_id = Uuid::parse_str(&workflow_id)?;
        let conversation_id = Uuid::parse_str(&conversation_id)?;

        // With strict moderation off, PolisGetReportData shows pending statements.
        let step = workflow_step::get_by_id(&pool, &step_id).await?;
        let ToolConfig::Polis(config) = step.preview_tool_config else {
            panic!("step should run Polis");
        };
        let open_moderation = ToolConfig::Polis(PolisToolConfig {
            strict_moderation: Some(false),
            ..config
        });
        workflow_step::update(
            &pool,
            &step_id,
            &workflow_id,
            &PartialWorkflowStep {
                tool_config: Some(open_moderation.clone()),
                preview_tool_config: Some(open_moderation),
                ..Default::default()
            },
        )
        .await?;

        let mut reader = UserSession::new_guest();
        let url =
            |block: &str| format!("/tools/polis/public_report/{block}?workflow_step_id={step_id}");

        let (status, _, _) = reader.get(&app, &url("key_stats")).await?;
        assert_eq!(status, StatusCode::NOT_FOUND, "served before publishing");

        let preview_url =
            |block: &str| format!("/tools/polis/report_preview/{block}?workflow_step_id={step_id}");
        for (block, _) in SECTION_BLOCKS {
            let (status, body, _) = admin.get(&app, &preview_url(block)).await?;
            assert_eq!(status, StatusCode::OK, "{block} preview failed");
            assert!(
                !body.to_string().contains(PENDING_TEXT),
                "{block} preview showed a pending statement"
            );
        }
        let (status, _, _) = reader.get(&app, &preview_url("key_stats")).await?;
        assert!(
            status.is_client_error(),
            "a guest was shown the unpublished preview"
        );

        report::create_for_conversation(&pool, conversation_id, "en").await?;
        report::update(
            &pool,
            conversation_id,
            PartialReport {
                is_public: Some(true),
                conversation_id: None,
                body: None,
                section_configs: None,
            },
        )
        .await?;

        let (status, _, _) = reader.get(&app, &url("key_stats")).await?;
        assert_eq!(
            status,
            StatusCode::NOT_FOUND,
            "served before the report embeds it"
        );

        embed_every_section_block(&pool, conversation_id, &step_id).await?;
        for (block, _) in SECTION_BLOCKS {
            let (status, body, _) = reader.get(&app, &url(block)).await?;
            assert_eq!(status, StatusCode::OK, "{block} failed");
            assert!(
                !body.to_string().contains(PENDING_TEXT),
                "{block} served a pending statement"
            );
        }

        let (_, body, _) = reader.get(&app, &url("key_stats")).await?;
        assert_eq!(body["statements"], 1);

        let (_, body, _) = reader.get(&app, &url("opinion_groups")).await?;
        assert_eq!(
            body["groups"][0]["representativeComments"],
            json!([{ "tid": 1 }])
        );

        Ok(())
    }

    async fn embed_every_section_block(
        pool: &PgPool,
        conversation_id: Uuid,
        workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError> {
        let report = report::get_for_conversation(pool, conversation_id).await?;
        let summary =
            get_text_translation_by_content_and_locale(pool, &report.summary, "en").await?;
        let embeds: Vec<_> = SECTION_BLOCKS
            .iter()
            .map(|(_, section_block)| {
                json!({
                    "type": "reportComponentEmbed",
                    "attrs": { "toolStepId": workflow_step_id, "componentType": section_block }
                })
            })
            .collect();
        update_text_translation(
            pool,
            &summary.id,
            &UpdateTextTranslation {
                content: Some(json!({ "type": "doc", "content": embeds }).to_string()),
                ..Default::default()
            },
        )
        .await?;
        Ok(())
    }

    #[test]
    fn key_stats_count_group_members_statements_and_votes() {
        let report = WikiPollReport {
            comments: vec![statement(1, "one", None), statement(2, "two", None)],
            groups: vec![group(0, 3, &[]), group(1, 4, &[])],
            participants: vec![],
        };
        assert_eq!(
            key_stats(&report),
            PolisPublicKeyStats {
                participants: Count(7),
                statements: Count(2),
                votes: Count(8),
            }
        );
    }

    #[test]
    fn ranked_statements_put_the_highest_score_first_and_unscored_last() {
        let report = WikiPollReport {
            comments: vec![
                statement(1, "low", Some(0.1)),
                statement(2, "unscored", None),
                statement(3, "high", Some(0.8)),
            ],
            groups: vec![],
            participants: vec![],
        };
        let ranked = ranked_statements(&report, &HashMap::new(), |c| c.divisiveness);
        let tids: Vec<u32> = ranked.statements.iter().map(|s| s.tid.0).collect();
        assert_eq!(tids, vec![3, 1, 2]);
    }

    #[test]
    fn consensus_continuum_only_places_scored_statements() {
        let report = WikiPollReport {
            comments: vec![
                statement(1, "scored", Some(0.4)),
                statement(2, "unscored", None),
                statement(3, "not a number", Some(f64::NAN)),
            ],
            groups: vec![],
            participants: vec![],
        };
        let continuum = consensus_continuum(&report, &HashMap::new());
        let tids: Vec<u32> = continuum.statements.iter().map(|s| s.tid.0).collect();
        assert_eq!(tids, vec![1]);
    }

    #[test]
    fn consensus_continuum_scales_divisiveness_to_the_most_divisive_statement() {
        let report = WikiPollReport {
            comments: vec![
                statement(1, "half", Some(1.5)),
                statement(2, "most", Some(3.0)),
            ],
            groups: vec![],
            participants: vec![],
        };
        let continuum = consensus_continuum(&report, &HashMap::new());
        let shares: Vec<Share> = continuum
            .statements
            .iter()
            .map(|s| s.divisiveness)
            .collect();
        assert_eq!(shares, vec![Share(0.5), Share(1.0)]);
    }

    #[test]
    fn opinion_groups_only_carry_represented_reported_statements() {
        let report = WikiPollReport {
            comments: vec![
                statement(1, "represented", None),
                statement(2, "not represented", None),
            ],
            groups: vec![group(0, 3, &[1, 9])],
            participants: vec![],
        };
        let themes = HashMap::from([(1, vec!["Transport".to_string()])]);
        let groups = opinion_groups(&report, &themes);

        assert_eq!(
            groups.groups[0].representative_comments,
            vec![PolisPublicRepresentativeStatement {
                tid: StatementId(1)
            }]
        );
        let tids: Vec<u32> = groups.statements.iter().map(|s| s.tid.0).collect();
        assert_eq!(tids, vec![1]);
        assert_eq!(
            groups.statements[0].themes,
            vec![FacilitatorText::from_facilitator("Transport".into())]
        );
    }
}
