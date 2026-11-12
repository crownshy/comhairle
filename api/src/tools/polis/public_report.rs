//! Public report data for the Polis section blocks (ADR-0061).
//!
//! One endpoint per embeddable section block, each returning only what that block draws.
//! `PolisGetReportData` stays the admin view and is never called from the public report.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use axum::extract::{Json, Query, State};
use moka::future::Cache;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};
use uuid::Uuid;

use super::{PolisTool, PolisToolConfig};
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::polis_statement_aux::{self, PolisStatementAuxFilterOptions};
use crate::models::wikipoll_conversation_summary;
use crate::routes::auth::RequiredUser;
use crate::tools::public_report_data::anonymous::{
    Count, FacilitatorText, GeneratedText, ModeratedText, OpinionGroupId, PublicReportData, Share,
    StatementId, public_report_data,
};
use crate::tools::public_report_data::themes::latest_theme_collection;
use crate::tools::public_report_data::{serve_public_report_data, serve_report_preview_data};
use crate::wiki_poll_service::ReportScope;
use crate::wiki_poll_service::polis_service::{
    CommentReportData, GroupReportData, GroupVoteCounts, VoteCounts, WikiPollReport,
};

/// Public report data never shows pending statements, so this does not follow the step's
/// `strict_moderation` the way `PolisGetReportData` does.
const PUBLIC_SCOPE: ReportScope = ReportScope::AcceptedOnly;

/// Where the theme collections of a Polis step come from (`sensemaking::harvest`).
const THEME_ORIGIN: &str = "polis";

/// How long a step's theme tags, sensemaker summary and themes are served before reading them
/// again. Every section block of a step shares them, and they only change when a facilitator
/// edits a tag or generates again. Votes stay on the Polis client's shorter report cache.
const CONTEXT_FRESH_FOR: Duration = Duration::from_secs(60);

/// Upper bound on cached steps; well above anything one deployment hosts.
const CONTEXT_CACHE_CAPACITY: u64 = 1_000;

/// Keyed by step and poll, since a step's preview and live configs can point at different polls.
static CONTEXT_CACHE: LazyLock<Cache<(Uuid, String), Arc<StepContext>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(CONTEXT_CACHE_CAPACITY)
        .time_to_live(CONTEXT_FRESH_FOR)
        .build()
});

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
        #[serde(flatten)]
        pub votes: PolisPublicVotes,
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
        pub statement_id: StatementId,
        pub text: ModeratedText,
        pub themes: Vec<FacilitatorText>,
        pub overall_votes: PolisPublicVotes,
        pub group_votes: Vec<PolisPublicGroupVotes>,
    }
}

public_report_data! {
    /// A statement in a ranked list, which can be filtered to seed or participant statements.
    pub struct PolisPublicRankedStatement {
        #[serde(flatten)]
        pub statement: PolisPublicStatement,
        pub is_seed: bool,
    }
}

public_report_data! {
    /// Statements ranked highest first, for the areas of consensus and disagreement.
    pub struct PolisPublicRankedStatements {
        pub statements: Vec<PolisPublicRankedStatement>,
        pub groups: Vec<PolisPublicGroup>,
        /// The sensemaker's summary of this side, once a facilitator has generated one.
        pub summary: Option<GeneratedText>,
    }
}

public_report_data! {
    /// A statement Polis has placed on the consensus continuum.
    pub struct PolisPublicPlacedStatement {
        #[serde(flatten)]
        pub statement: PolisPublicStatement,
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
    pub struct PolisPublicRepresentativeComment {
        pub statement_id: StatementId,
    }
}

public_report_data! {
    /// The sensemaker's name and summary for an opinion group.
    pub struct PolisPublicGroupSummary {
        pub name: GeneratedText,
        pub description: GeneratedText,
    }
}

public_report_data! {
    pub struct PolisPublicOpinionGroup {
        #[serde(flatten)]
        pub group: PolisPublicGroup,
        /// Once a facilitator has generated one, and while it still describes the current
        /// groups.
        pub summary: Option<PolisPublicGroupSummary>,
        /// Points into `PolisPublicOpinionGroups::statements`.
        pub representative_comments: Vec<PolisPublicRepresentativeComment>,
    }
}

public_report_data! {
    pub struct PolisPublicOpinionGroups {
        pub groups: Vec<PolisPublicOpinionGroup>,
        /// Only the statements some group is represented by.
        pub statements: Vec<PolisPublicStatement>,
    }
}

public_report_data! {
    /// A theme with the accepted statements the sensemaker filed under it.
    pub struct PolisPublicTheme {
        pub title: GeneratedText,
        pub description: GeneratedText,
        pub statements: Vec<PolisPublicStatement>,
    }
}

public_report_data! {
    pub struct PolisPublicThemes {
        pub themes: Vec<PolisPublicTheme>,
        pub groups: Vec<PolisPublicGroup>,
    }
}

/// The sensemaker's Polis summary, as `PolisGetLatestSummary` stores it. Every field is
/// optional because the summary format has changed over time.
#[derive(Deserialize, Default)]
struct SensemakerSummary {
    #[serde(default)]
    consensus_description: Option<String>,
    /// The stored key is misspelled.
    #[serde(default, rename = "disagrement_description")]
    disagreement_description: Option<String>,
    #[serde(default)]
    group_descriptions: Vec<GroupDescription>,
}

#[derive(Deserialize)]
struct GroupDescription {
    group_id: SummaryGroupId,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
}

/// Older summaries store a group id as a numeric string, newer ones as a number.
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(try_from = "StoredGroupId")]
struct SummaryGroupId(u32);

#[derive(Deserialize)]
#[serde(untagged)]
enum StoredGroupId {
    Number(u32),
    Text(String),
}

impl TryFrom<StoredGroupId> for SummaryGroupId {
    type Error = std::num::ParseIntError;

    fn try_from(stored: StoredGroupId) -> Result<Self, Self::Error> {
        match stored {
            StoredGroupId::Number(id) => Ok(Self(id)),
            StoredGroupId::Text(id) => Ok(Self(id.parse()?)),
        }
    }
}

/// A theme as the public report needs it: the sensemaker's text and the Polis statement ids
/// filed under it, accepted or not.
#[derive(Debug)]
struct CachedTheme {
    title: String,
    description: String,
    statement_ids: Vec<u32>,
}

/// What a step's section blocks read from Comhairle's own tables, shared through
/// `CONTEXT_CACHE`.
#[derive(Default)]
struct StepContext {
    theme_tags: ThemeTags,
    summary: SensemakerSummary,
    themes: Vec<CachedTheme>,
}

/// One embeddable section block: how the report stores it, where its endpoints live, how the
/// API docs describe it, and how to build what it draws.
trait SectionBlock: Send + Sync + 'static {
    /// The section block type the report stores (`lib/reports/embeds.ts`).
    const TYPE: &'static str;
    /// The last segment of both endpoint paths.
    const PATH: &'static str;
    /// Completes the operation ids `PolisGetPublic…` and `PolisPreview…`.
    const OPERATION: &'static str;
    const SUMMARY: &'static str;
    const DESCRIPTION: &'static str;

    type Data: PublicReportData + JsonSchema + Send;

    fn build(
        source: &ReportSource<'_>,
    ) -> impl Future<Output = Result<Self::Data, ComhairleError>> + Send;
}

struct KeyStats;
struct Consensus;
struct Disagreement;
struct ConsensusContinuum;
struct OpinionGroups;
struct Themes;

impl SectionBlock for KeyStats {
    const TYPE: &'static str = "polis-key-stats";
    const PATH: &'static str = "key_stats";
    const OPERATION: &'static str = "KeyStats";
    const SUMMARY: &'static str = "Key stats";
    const DESCRIPTION: &'static str = "Participant, accepted statement and vote counts.";
    type Data = PolisPublicKeyStats;

    async fn build(source: &ReportSource<'_>) -> Result<Self::Data, ComhairleError> {
        Ok(key_stats(&source.report().await?))
    }
}

impl SectionBlock for Consensus {
    const TYPE: &'static str = "polis-area-consensus";
    const PATH: &'static str = "consensus";
    const OPERATION: &'static str = "Consensus";
    const SUMMARY: &'static str = "Areas of consensus";
    const DESCRIPTION: &'static str =
        "Accepted statements ranked by group informed consensus, highest first.";
    type Data = PolisPublicRankedStatements;

    async fn build(source: &ReportSource<'_>) -> Result<Self::Data, ComhairleError> {
        let (report, context) = source.load().await?;
        let summary = context.summary.consensus_description.clone();
        Ok(ranked_statements(
            &report,
            &context.theme_tags,
            summary,
            |c| c.group_informed_consensus,
        ))
    }
}

impl SectionBlock for Disagreement {
    const TYPE: &'static str = "polis-area-disagreement";
    const PATH: &'static str = "disagreement";
    const OPERATION: &'static str = "Disagreement";
    const SUMMARY: &'static str = "Areas of disagreement";
    const DESCRIPTION: &'static str = "Accepted statements ranked by divisiveness, highest first.";
    type Data = PolisPublicRankedStatements;

    async fn build(source: &ReportSource<'_>) -> Result<Self::Data, ComhairleError> {
        let (report, context) = source.load().await?;
        let summary = context.summary.disagreement_description.clone();
        Ok(ranked_statements(
            &report,
            &context.theme_tags,
            summary,
            |c| c.divisiveness,
        ))
    }
}

impl SectionBlock for ConsensusContinuum {
    const TYPE: &'static str = "polis-consensus-continuum";
    const PATH: &'static str = "consensus_continuum";
    const OPERATION: &'static str = "ConsensusContinuum";
    const SUMMARY: &'static str = "Consensus continuum";
    const DESCRIPTION: &'static str = "Accepted statements Polis has given a divisiveness score.";
    type Data = PolisPublicConsensusContinuum;

    async fn build(source: &ReportSource<'_>) -> Result<Self::Data, ComhairleError> {
        let (report, context) = source.load().await?;
        Ok(consensus_continuum(&report, &context.theme_tags))
    }
}

impl SectionBlock for OpinionGroups {
    const TYPE: &'static str = "polis-opinion-groups";
    const PATH: &'static str = "opinion_groups";
    const OPERATION: &'static str = "OpinionGroups";
    const SUMMARY: &'static str = "Opinion groups";
    const DESCRIPTION: &'static str =
        "Each opinion group's size and accepted representative statements.";
    type Data = PolisPublicOpinionGroups;

    async fn build(source: &ReportSource<'_>) -> Result<Self::Data, ComhairleError> {
        let (report, context) = source.load().await?;
        Ok(opinion_groups(
            &report,
            &context.theme_tags,
            &context.summary,
        ))
    }
}

impl SectionBlock for Themes {
    const TYPE: &'static str = "polis-themes";
    const PATH: &'static str = "themes";
    const OPERATION: &'static str = "Themes";
    const SUMMARY: &'static str = "Themes";
    const DESCRIPTION: &'static str =
        "The step's latest themes with the accepted statements under each.";
    type Data = PolisPublicThemes;

    async fn build(source: &ReportSource<'_>) -> Result<Self::Data, ComhairleError> {
        let (report, context) = source.load().await?;
        Ok(public_themes(&report, &context.theme_tags, &context.themes))
    }
}

pub fn routes() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .merge(section_block_routes::<KeyStats>())
        .merge(section_block_routes::<Consensus>())
        .merge(section_block_routes::<Disagreement>())
        .merge(section_block_routes::<ConsensusContinuum>())
        .merge(section_block_routes::<OpinionGroups>())
        .merge(section_block_routes::<Themes>())
}

/// The public endpoint and its preview twin for one section block.
fn section_block_routes<Block: SectionBlock>() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            &format!("/polis/public_report/{}", Block::PATH),
            get_with(get_public::<Block>, |op| {
                op.id(&format!("PolisGetPublic{}", Block::OPERATION))
                    .tag("Tools")
                    .summary(&format!("{} for a published report", Block::SUMMARY))
                    .description(&format!(
                        "{} 404 unless the conversation's published report embeds this \
                         section block for the step.",
                        Block::DESCRIPTION
                    ))
                    .response::<200, Json<Block::Data>>()
            }),
        )
        .api_route(
            &format!("/polis/report_preview/{}", Block::PATH),
            get_with(get_preview::<Block>, |op| {
                op.id(&format!("PolisPreview{}", Block::OPERATION))
                    .tag("Tools")
                    .summary(&format!(
                        "{} as the published report will show them",
                        Block::SUMMARY
                    ))
                    .description(&format!(
                        "The same data as PolisGetPublic{}, for facilitators who can update \
                         the conversation, before the report is published.",
                        Block::OPERATION
                    ))
                    .response::<200, Json<Block::Data>>()
            }),
        )
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct PublicReportQuery {
    pub workflow_step_id: Uuid,
}

#[instrument(err(Debug), skip(state), fields(section_block = Block::TYPE))]
async fn get_public<Block: SectionBlock>(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<Block::Data>, ComhairleError> {
    let data = serve_public_report_data::<PolisTool, _>(
        &state.db,
        &workflow_step_id,
        Block::TYPE,
        None,
        async |config| Block::build(&ReportSource::new(&state, workflow_step_id, config)).await,
    )
    .await?;
    Ok(Json(data))
}

#[instrument(err(Debug), skip(state), fields(section_block = Block::TYPE))]
async fn get_preview<Block: SectionBlock>(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<Block::Data>, ComhairleError> {
    let data = serve_report_preview_data::<PolisTool, _>(
        &state,
        &user,
        &workflow_step_id,
        async |config| Block::build(&ReportSource::new(&state, workflow_step_id, config)).await,
    )
    .await?;
    Ok(Json(data))
}

/// One Polis step's report at `PUBLIC_SCOPE`, plus what Comhairle stores about it.
struct ReportSource<'a> {
    state: &'a ComhairleState,
    workflow_step_id: Uuid,
    config: PolisToolConfig,
}

/// Facilitator-set theme tags per statement.
type ThemeTags = HashMap<StatementId, Vec<FacilitatorText>>;

impl<'a> ReportSource<'a> {
    fn new(state: &'a ComhairleState, workflow_step_id: Uuid, config: PolisToolConfig) -> Self {
        Self {
            state,
            workflow_step_id,
            config,
        }
    }

    /// Goes through the Polis client's report cache, so the section blocks of one step share
    /// an upstream fetch.
    async fn report(&self) -> Result<WikiPollReport, ComhairleError> {
        Ok(self
            .state
            .wiki_poll_service
            .get_report_data(&self.config.poll_id, PUBLIC_SCOPE)
            .await?)
    }

    /// The report and the step's context, which every section block but the key stats reads.
    async fn load(&self) -> Result<(WikiPollReport, Arc<StepContext>), ComhairleError> {
        Ok((self.report().await?, self.context().await?))
    }

    /// Requests that miss at the same time wait on one load.
    async fn context(&self) -> Result<Arc<StepContext>, ComhairleError> {
        let key = (self.workflow_step_id, self.config.poll_id.clone());
        CONTEXT_CACHE
            .try_get_with(key, async { self.load_context().await.map(Arc::new) })
            .await
            .map_err(|error: Arc<ComhairleError>| {
                ComhairleError::ReportDataUnavailable(error.to_string())
            })
    }

    async fn load_context(&self) -> Result<StepContext, ComhairleError> {
        Ok(StepContext {
            theme_tags: self.theme_tags().await?,
            summary: self.summary().await?,
            themes: self.themes().await?,
        })
    }

    async fn theme_tags(&self) -> Result<ThemeTags, ComhairleError> {
        let rows = polis_statement_aux::list(
            &self.state.db,
            Some(self.workflow_step_id),
            Some(self.config.poll_id.clone()),
            PolisStatementAuxFilterOptions::default(),
        )
        .await?;
        Ok(rows
            .into_iter()
            .filter(|row| !row.themes.is_empty())
            .filter_map(|row| {
                let statement_id = StatementId(u32::try_from(row.polis_statement_id).ok()?);
                let labels = row
                    .themes
                    .into_iter()
                    .map(FacilitatorText::from_facilitator)
                    .collect();
                Some((statement_id, labels))
            })
            .collect())
    }

    /// The latest sensemaker summary, or an empty one when none has been generated or it
    /// can't be read.
    async fn summary(&self) -> Result<SensemakerSummary, ComhairleError> {
        let Some(latest) = wikipoll_conversation_summary::get_latest_by_workflow_step_id(
            &self.state.db,
            self.workflow_step_id,
        )
        .await?
        else {
            return Ok(SensemakerSummary::default());
        };
        Ok(serde_json::from_value(latest.result).unwrap_or_else(|error| {
            warn!(workflow_step_id = %self.workflow_step_id, %error, "unreadable sensemaker summary");
            SensemakerSummary::default()
        }))
    }

    /// The sensemaker's latest theme collection, empty when none has been generated.
    async fn themes(&self) -> Result<Vec<CachedTheme>, ComhairleError> {
        let collection =
            latest_theme_collection(&self.state.db, &self.workflow_step_id, THEME_ORIGIN)
                .await?
                .unwrap_or_default();
        Ok(collection
            .into_iter()
            .map(|theme| CachedTheme {
                title: theme.theme.title,
                description: theme.theme.description,
                statement_ids: theme
                    .statements
                    .iter()
                    .filter_map(|statement| statement.source_sub_id.as_deref()?.parse().ok())
                    .collect(),
            })
            .collect())
    }
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
    themes: &ThemeTags,
    summary: Option<String>,
    score: impl Fn(&CommentReportData) -> Option<f64>,
) -> PolisPublicRankedStatements {
    let mut comments: Vec<&CommentReportData> = report.comments.iter().collect();
    comments.sort_by(|a, b| descending(score(a), score(b)));
    PolisPublicRankedStatements {
        statements: comments
            .into_iter()
            .map(|c| PolisPublicRankedStatement {
                statement: public_statement(c, themes),
                is_seed: c.is_seed,
            })
            .collect(),
        groups: group_sizes(report),
        summary: summary.map(GeneratedText::from_sensemaking),
    }
}

fn descending(a: Option<f64>, b: Option<f64>) -> Ordering {
    b.unwrap_or(0.0).total_cmp(&a.unwrap_or(0.0))
}

fn consensus_continuum(
    report: &WikiPollReport,
    themes: &ThemeTags,
) -> PolisPublicConsensusContinuum {
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
                statement: public_statement(c, themes),
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
fn opinion_groups(
    report: &WikiPollReport,
    themes: &ThemeTags,
    summary: &SensemakerSummary,
) -> PolisPublicOpinionGroups {
    let reported: HashSet<u32> = report.comments.iter().map(|c| c.tid).collect();
    let mut represented: HashSet<u32> = HashSet::new();
    let summaries = group_summaries(summary, report);
    let groups: Vec<PolisPublicOpinionGroup> = report
        .groups
        .iter()
        .map(|g| PolisPublicOpinionGroup {
            group: group_size(g),
            summary: summaries.get(&g.group_id).and_then(|description| {
                Some(PolisPublicGroupSummary {
                    name: GeneratedText::from_sensemaking(description.name.clone()?),
                    description: GeneratedText::from_sensemaking(description.description.clone()?),
                })
            }),
            representative_comments: g
                .representative_comments
                .iter()
                .filter(|r| reported.contains(&r.tid))
                .inspect(|r| {
                    represented.insert(r.tid);
                })
                .map(|r| PolisPublicRepresentativeComment {
                    statement_id: StatementId(r.tid),
                })
                .collect(),
        })
        .collect();
    PolisPublicOpinionGroups {
        statements: report
            .comments
            .iter()
            .filter(|c| represented.contains(&c.tid))
            .map(|c| public_statement(c, themes))
            .collect(),
        groups,
    }
}

/// The summary's group descriptions by group id, or none when the summary describes other
/// groups than Polis currently has, since it is then out of date. Polis can reshuffle members
/// while keeping the same group ids, which this can't see.
fn group_summaries<'a>(
    summary: &'a SensemakerSummary,
    report: &WikiPollReport,
) -> HashMap<u32, &'a GroupDescription> {
    let described: HashMap<u32, &GroupDescription> = summary
        .group_descriptions
        .iter()
        .map(|description| (description.group_id.0, description))
        .collect();
    let current: HashSet<u32> = report.groups.iter().map(|g| g.group_id).collect();
    let describes_current_groups = described.len() == summary.group_descriptions.len()
        && described.keys().copied().collect::<HashSet<u32>>() == current;
    if describes_current_groups {
        described
    } else {
        HashMap::new()
    }
}

/// Themes with the accepted statements under each. A theme collection can hold statements
/// moderation hasn't accepted, so only those in the public report are kept, and a theme the
/// sensemaker left empty still shows with no statements.
fn public_themes(
    report: &WikiPollReport,
    themes: &ThemeTags,
    collection: &[CachedTheme],
) -> PolisPublicThemes {
    let by_statement_id: HashMap<u32, &CommentReportData> =
        report.comments.iter().map(|c| (c.tid, c)).collect();
    PolisPublicThemes {
        themes: collection
            .iter()
            .map(|theme| PolisPublicTheme {
                title: GeneratedText::from_sensemaking(theme.title.clone()),
                description: GeneratedText::from_sensemaking(theme.description.clone()),
                statements: theme
                    .statement_ids
                    .iter()
                    .filter_map(|statement_id| by_statement_id.get(statement_id))
                    .map(|c| public_statement(c, themes))
                    .collect(),
            })
            .collect(),
        groups: group_sizes(report),
    }
}

/// Only valid for reports loaded at `PUBLIC_SCOPE`, which leaves out every statement
/// moderation hasn't accepted.
fn public_statement(comment: &CommentReportData, themes: &ThemeTags) -> PolisPublicStatement {
    PolisPublicStatement {
        statement_id: StatementId(comment.tid),
        text: ModeratedText::from_accepted(comment.text.clone()),
        themes: themes
            .get(&StatementId(comment.tid))
            .cloned()
            .unwrap_or_default(),
        overall_votes: votes(&comment.overall_votes),
        group_votes: comment.group_votes.iter().map(group_votes).collect(),
    }
}

fn votes(counts: &VoteCounts) -> PolisPublicVotes {
    PolisPublicVotes {
        agrees: Count(counts.agrees.into()),
        disagrees: Count(counts.disagrees.into()),
        passes: Count(counts.passes.into()),
    }
}

fn group_votes(counts: &GroupVoteCounts) -> PolisPublicGroupVotes {
    PolisPublicGroupVotes {
        group_id: OpinionGroupId(counts.group_id),
        votes: PolisPublicVotes {
            agrees: Count(counts.agrees.into()),
            disagrees: Count(counts.disagrees.into()),
            passes: Count(counts.passes.into()),
        },
    }
}

fn group_sizes(report: &WikiPollReport) -> Vec<PolisPublicGroup> {
    report.groups.iter().map(group_size).collect()
}

fn group_size(group: &GroupReportData) -> PolisPublicGroup {
    PolisPublicGroup {
        group_id: OpinionGroupId(group.group_id),
        total_members: Count(group.total_members),
    }
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
    use crate::tools::ToolConfig;
    use crate::wiki_poll_service::polis_service::RepresentativeComment;
    use crate::wiki_poll_service::{MockWikiPollService, WikiPoll, WikiPollService};

    /// Each endpoint's path and the section block type it serves.
    const SECTION_BLOCKS: [(&str, &str); 6] = [
        (KeyStats::PATH, KeyStats::TYPE),
        (Consensus::PATH, Consensus::TYPE),
        (Disagreement::PATH, Disagreement::TYPE),
        (ConsensusContinuum::PATH, ConsensusContinuum::TYPE),
        (OpinionGroups::PATH, OpinionGroups::TYPE),
        (Themes::PATH, Themes::TYPE),
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
            json!([{ "statementId": 1 }])
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn facilitators_can_preview_an_event_step(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state()
            .db(pool.clone())
            .wiki_poll_service(polis_service_serving_reports())
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;

        let (_, conversation, _) = admin.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);
        let (_, event, _) = admin.create_random_event(&app, &conversation_id).await?;
        let event_id: String = extract("id", &event);
        let (_, workflow, _) = admin
            .create_random_event_workflow(&app, &conversation_id, &event_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);
        let (status, step, _) = admin
            .post(
                &app,
                &format!(
                    "/conversation/{conversation_id}/events/{event_id}/workflows/{workflow_id}/workflow_steps"
                ),
                json!({
                    "name": "Event Polis step",
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
        assert!(status.is_success(), "event step not created: {step}");
        let step_id: String = extract("id", &step);
        let url = format!("/tools/polis/report_preview/key_stats?workflow_step_id={step_id}");

        let (status, _, _) = admin.get(&app, &url).await?;
        assert_eq!(
            status,
            StatusCode::OK,
            "facilitator couldn't preview an event step"
        );

        let mut reader = UserSession::new_guest();
        let (status, _, _) = reader.get(&app, &url).await?;
        assert!(
            status.is_client_error(),
            "a guest was shown an event step's preview"
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
        let ranked = ranked_statements(&report, &HashMap::new(), None, |c| c.divisiveness);
        let tids: Vec<u32> = ranked
            .statements
            .iter()
            .map(|s| s.statement.statement_id.0)
            .collect();
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
        let tids: Vec<u32> = continuum
            .statements
            .iter()
            .map(|s| s.statement.statement_id.0)
            .collect();
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
        let themes = HashMap::from([(
            StatementId(1),
            vec![FacilitatorText::from_facilitator("Transport".into())],
        )]);
        let groups = opinion_groups(&report, &themes, &SensemakerSummary::default());

        assert_eq!(
            groups.groups[0].representative_comments,
            vec![PolisPublicRepresentativeComment {
                statement_id: StatementId(1)
            }]
        );
        let tids: Vec<u32> = groups.statements.iter().map(|s| s.statement_id.0).collect();
        assert_eq!(tids, vec![1]);
        assert_eq!(
            groups.statements[0].themes,
            vec![FacilitatorText::from_facilitator("Transport".into())]
        );
    }

    #[test]
    fn themes_only_carry_statements_the_public_report_serves() {
        let accepted_only = report_for(ReportScope::AcceptedOnly);
        let collection = [CachedTheme {
            title: "Transport".into(),
            description: "Buses and trains".into(),
            statement_ids: vec![1, 2],
        }];
        let themes = public_themes(&accepted_only, &HashMap::new(), &collection);

        let tids: Vec<u32> = themes.themes[0]
            .statements
            .iter()
            .map(|s| s.statement_id.0)
            .collect();
        assert_eq!(
            tids,
            vec![1],
            "the pending statement 2 came through a theme"
        );
    }

    fn summary_describing(group_ids: serde_json::Value) -> SensemakerSummary {
        let group_descriptions: Vec<_> = group_ids
            .as_array()
            .unwrap()
            .iter()
            .map(|id| json!({ "group_id": id, "name": "A name", "description": "A summary" }))
            .collect();
        serde_json::from_value(json!({ "group_descriptions": group_descriptions })).unwrap()
    }

    #[test]
    fn group_summaries_read_numeric_and_string_group_ids() {
        let report = WikiPollReport {
            comments: vec![],
            groups: vec![group(0, 3, &[]), group(1, 4, &[])],
            participants: vec![],
        };
        let summary = summary_describing(json!([0, "1"]));
        let mut ids: Vec<u32> = group_summaries(&summary, &report).into_keys().collect();
        ids.sort();
        assert_eq!(ids, vec![0, 1]);
    }

    #[test]
    fn group_summaries_are_dropped_once_the_groups_change() {
        let report = WikiPollReport {
            comments: vec![],
            groups: vec![group(0, 3, &[]), group(2, 4, &[])],
            participants: vec![],
        };
        let same_count_other_groups = summary_describing(json!([0, 1]));
        assert!(group_summaries(&same_count_other_groups, &report).is_empty());
    }

    #[test]
    fn group_summary_needs_both_a_name_and_a_description() {
        let report = WikiPollReport {
            comments: vec![],
            groups: vec![group(0, 3, &[])],
            participants: vec![],
        };
        let summary: SensemakerSummary = serde_json::from_value(json!({
            "group_descriptions": [{ "group_id": 0, "name": "Only a name" }]
        }))
        .unwrap();
        let groups = opinion_groups(&report, &HashMap::new(), &summary);
        assert!(groups.groups[0].summary.is_none());
    }

    #[test]
    fn the_misspelled_disagreement_key_is_read() {
        let summary: SensemakerSummary =
            serde_json::from_value(json!({ "disagrement_description": "Split on parking" }))
                .unwrap();
        assert_eq!(
            summary.disagreement_description.as_deref(),
            Some("Split on parking")
        );
    }
}
