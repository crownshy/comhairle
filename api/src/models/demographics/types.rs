use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::translations::TextContentId;

use super::responses::ConsentState;
use super::{DatePrecision, LocationFormat, QuestionType, VersionStatus};

// ---------------------------------------------------------------------------
// Inputs
//
// Define the expected input structures for demographic forms and questions.
// ---------------------------------------------------------------------------

/// The input for creating a question or replacing the content of its draft version.
///
/// See [`QuestionDto`] and [`QuestionVersionDto`] for the full representations.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuestionDraftInput {
    /// The text of the question.
    pub text: String,
    /// The type of the question.
    pub question_type: QuestionType,
    /// The precision of the date, for date questions.
    #[serde(default)]
    pub date_precision: Option<DatePrecision>,
    /// The format of the location, for location questions.
    #[serde(default)]
    pub location_format: Option<LocationFormat>,
    /// The country of the location, if applicable.
    #[serde(default)]
    pub location_country: Option<String>,
    /// The maximum number of selections allowed, for multi select questions.
    #[serde(default)]
    pub max_selections: Option<i32>,
    /// Whether an "Other" option is allowed.
    #[serde(default)]
    pub allow_other: bool,
    /// Whether a "Prefer not to say" option is allowed.
    #[serde(default)]
    pub prefer_not_to_say: bool,
    /// The labels of the options, in order, for select questions.
    #[serde(default)]
    pub options: Vec<String>,
    /// The JSON schema answers are validated against, for custom questions.
    #[serde(default)]
    pub json_schema: Option<Value>,
    /// Whether answering requires explicit consent.
    #[serde(default)]
    pub special_category: bool,
    /// The consent text shown for special category questions.
    #[serde(default)]
    pub consent_text: Option<String>,
    /// The tags to assign to the question.
    #[serde(default)]
    pub tag_ids: Vec<Uuid>,
}

/// Used for deserializing to `true` by default.
fn true_by_default() -> bool {
    true
}

/// One question within a form draft.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DemographicFormEntryInput {
    /// The question to ask.
    pub question_id: Uuid,
    /// The position of the question within the form.
    pub position: i32,
    /// Whether the question must be answered.
    #[serde(default = "true_by_default")]
    pub required: bool,
}

/// The input for creating a form or replacing the content of its draft version.
///
/// See [`DemographicFormEntryInput`] for the individual entries.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DemographicFormDraftInput {
    /// The name of the form.
    pub name: String,
    /// The questions in the form.
    #[serde(default)]
    pub entries: Vec<DemographicFormEntryInput>,
}

/// The input for answering a question.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ResponseInput {
    /// The answer, whose shape depends on the question type.
    pub value: Value,
    /// The conversation the answer was given in, if any.
    pub conversation_id: Option<Uuid>,
}

/// The input for granting or declining consent to share a special category answer.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ConsentInput {
    /// Whether the user agrees to share the answer.
    pub granted: bool,
}

/// The demographic answers collected by the signup form.
#[derive(Debug, Default)]
pub struct SignupAnswers {
    pub age: Option<i32>,
    pub ethnicity: Option<String>,
    pub gender: Option<String>,
    pub zipcode: Option<String>,
    pub political_party: Option<String>,
}

/// The answer to a postcode location question.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PostcodeAnswer {
    pub(super) postcode: String,
}

/// The answer to a latitude and longitude location question.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct LatLongAnswer {
    pub(super) latitude: f64,
    pub(super) longitude: f64,
}

/// The answer to a country location question.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CountryAnswer {
    pub(super) country: String,
}

/// The answer to a single or multi select question.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionAnswer {
    /// The chosen options.
    #[serde(default)]
    pub option_ids: Vec<Uuid>,
    /// Free text for the "Other" option.
    #[serde(default)]
    pub other_text: Option<String>,
    /// Whether the user chose "Prefer not to say".
    #[serde(default)]
    pub prefer_not_to_say: bool,
}

// ---------------------------------------------------------------------------
// Database tables
//
// Define the expected structure of the database tables for demographic questions and forms.
// ---------------------------------------------------------------------------

/// A row of `demographics_question`.
#[derive(Debug, Clone, Serialize, JsonSchema, sqlx::FromRow)]
#[enum_def(table_name = "demographics_question")]
pub struct QuestionRow {
    /// The unique identifier of the question.
    pub id: Uuid,
    /// The organization that owns the question. System defaults have none.
    pub organization_id: Option<Uuid>,
    /// The question this question was forked from, if any.
    pub forked_from_id: Option<Uuid>,
    /// When the question was archived, if applicable.
    pub archived_at: Option<DateTime<Utc>>,
    /// When the question was created.
    pub created_at: DateTime<Utc>,
}

/// A row of `demographics_question_version`. The text content ids are internal, the localized
/// text is returned by [`QuestionVersionDto`].
#[derive(Debug, Clone, Serialize, JsonSchema, sqlx::FromRow)]
#[enum_def(table_name = "demographics_question_version")]
pub struct DemographicQuestionVersion {
    /// The unique identifier of the question version.
    pub id: Uuid,
    /// The question this version belongs to.
    pub question_id: Uuid,
    /// The version number, which never repeats within a question.
    pub version_number: i32,
    /// Whether the version is a draft or published.
    pub status: VersionStatus,
    /// When the version was published, if applicable.
    pub published_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    #[schemars(skip)]
    pub text_content_id: TextContentId,
    /// The type of the question.
    pub question_type: QuestionType,
    /// The precision of the date, for date questions.
    pub date_precision: Option<DatePrecision>,
    /// The format of the location, for location questions.
    pub location_format: Option<LocationFormat>,
    /// The country of the location, if applicable.
    pub location_country: Option<String>,
    /// The maximum number of selections allowed, for multi select questions.
    pub max_selections: Option<i32>,
    /// Whether an "Other" option is allowed.
    pub allow_other: bool,
    /// Whether a "Prefer not to say" option is allowed.
    pub prefer_not_to_say: bool,
    /// The JSON schema answers are validated against, for custom questions.
    pub json_schema: Option<Value>,
    /// Whether answering requires explicit consent.
    pub special_category: bool,
    #[serde(skip)]
    #[schemars(skip)]
    pub consent_text_content_id: Option<TextContentId>,
    /// When the version was last updated, which is frozen at publication.
    pub updated_at: DateTime<Utc>,
}

/// A row of `demographics_question_option`.
#[derive(sqlx::FromRow)]
#[enum_def(table_name = "demographics_question_option")]
pub(super) struct OptionRow {
    pub(super) id: Uuid,
    pub(super) question_version_id: Uuid,
    pub(super) position: i32,
    pub(super) label_text_content_id: TextContentId,
}

/// A row of `demographics_question_tag`, which assigns a tag to a question.
#[derive(sqlx::FromRow)]
#[enum_def(table_name = "demographics_question_tag")]
#[allow(dead_code, reason = "Only used to derive `QuestionTagIden`.")]
pub(super) struct QuestionTag {
    pub(super) question_id: Uuid,
    pub(super) tag_id: Uuid,
}

/// A row of `demographics_tag`. A tag without an organization is a system tag.
#[derive(Debug, Clone, Serialize, JsonSchema, sqlx::FromRow)]
#[enum_def(table_name = "demographics_tag")]
pub struct DemographicTag {
    /// The unique identifier of the tag.
    pub id: Uuid,
    /// The organization this tag belongs to, if any.
    pub organization_id: Option<Uuid>,
    /// The name of the tag.
    pub name: String,
}

/// A row of `demographics_form`, a structured set of questions asked together.
#[derive(Debug, Clone, Serialize, JsonSchema, sqlx::FromRow)]
#[enum_def(table_name = "demographics_form")]
pub struct DemographicForm {
    /// The unique identifier of the form.
    pub id: Uuid,
    /// The organization that owns the form.
    pub organization_id: Uuid,
    /// When the form was archived, if applicable.
    pub archived_at: Option<DateTime<Utc>>,
    /// When the form was created.
    pub created_at: DateTime<Utc>,
}

/// A row of `demographics_form_version`, one iteration of a form.
#[derive(Debug, Clone, Serialize, JsonSchema, sqlx::FromRow)]
#[enum_def(table_name = "demographics_form_version")]
pub struct DemographicFormVersion {
    /// The unique identifier of the form version.
    pub id: Uuid,
    /// The form this version belongs to.
    pub form_id: Uuid,
    /// The version number, which never repeats within a form.
    pub version_number: i32,
    /// Whether the version is a draft or published.
    pub status: VersionStatus,
    /// The name of the form in this version.
    pub name: String,
    /// When the version was published, if applicable.
    pub published_at: Option<DateTime<Utc>>,
    /// When the version was last updated, which is frozen at publication.
    pub updated_at: DateTime<Utc>,
}

/// A row of `demographics_form_version_question`, one question within a form version.
#[derive(Debug, Clone, Serialize, JsonSchema, sqlx::FromRow)]
#[enum_def(table_name = "demographics_form_version_question")]
pub struct DemographicFormVersionQuestion {
    /// The form version the entry belongs to.
    pub form_version_id: Uuid,
    /// The form the entry belongs to.
    pub form_id: Uuid,
    /// The question to ask.
    pub question_id: Uuid,
    /// The question version pinned by a published form, if any.
    pub question_version_id: Option<Uuid>,
    /// The position of the question within the form.
    pub position: i32,
    /// Whether the question must be answered.
    pub required: bool,
}

/// A row of `demographics_response`. Reads go through [`ResponseDto`].
#[derive(sqlx::FromRow)]
#[enum_def(table_name = "demographics_response")]
#[allow(dead_code, reason = "Only used to derive `DemographicResponseIden`.")]
pub(super) struct DemographicResponse {
    pub(super) id: Uuid,
    pub(super) question_version_id: Uuid,
    pub(super) user_id: Option<Uuid>,
    pub(super) conversation_id: Option<Uuid>,
    pub(super) value: Value,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

/// A row of `demographics_consent`, where a declined consent is kept so the question is skipped.
#[derive(sqlx::FromRow)]
#[enum_def(table_name = "demographics_consent")]
#[allow(dead_code, reason = "Only used to derive `DemographicConsentIden`.")]
pub(super) struct DemographicConsent {
    pub(super) id: Uuid,
    pub(super) question_version_id: Uuid,
    pub(super) user_id: Option<Uuid>,
    pub(super) granted: bool,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Query projections
//
// Define the query projections for demographic forms and questions.
// ---------------------------------------------------------------------------

/// A tag joined to the question it is assigned to.
#[derive(sqlx::FromRow)]
pub(super) struct QuestionTagRow {
    pub(super) question_id: Uuid,
    #[sqlx(flatten)]
    pub(super) tag: DemographicTag,
}

/// A form entry joined to the text content of the question it asks.
#[derive(sqlx::FromRow)]
pub(super) struct DemographicFormEntryWithText {
    #[sqlx(flatten)]
    pub(super) entry: DemographicFormVersionQuestion,
    pub(super) display_text_content_id: Option<TextContentId>,
}

/// A question with the text content of its latest published version.
#[derive(sqlx::FromRow)]
pub(super) struct QuestionTextRow {
    pub(super) question_id: Uuid,
    pub(super) text_content_id: TextContentId,
}

/// The number of participants who gave one answer to one question.
#[derive(sqlx::FromRow)]
pub(super) struct ReportRow {
    #[sqlx(flatten)]
    pub(super) question: QuestionTextRow,
    pub(super) value: Value,
    pub(super) count: i64,
}

/// A participant's latest answer to one question, for the export.
#[derive(sqlx::FromRow)]
pub(super) struct ExportAnswerRow {
    pub(super) user_id: Uuid,
    pub(super) question_id: Uuid,
    pub(super) value: Value,
}
/// A consenting participant of a conversation, for the export.
#[derive(sqlx::FromRow)]
pub(super) struct ExportParticipantRow {
    pub(super) user_id: Uuid,
    pub(super) profile_created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// DTOs
//
// Define the data transfer objects for demographic forms and questions.
// ---------------------------------------------------------------------------

/// A lightweight view of a version, used in version lists of questions and forms.
#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VersionSummary {
    /// The unique identifier of the version.
    pub id: Uuid,
    /// The version number.
    pub version_number: i32,
    /// Whether the version is a draft or published.
    pub status: VersionStatus,
    /// When the version was published, if applicable.
    pub published_at: Option<DateTime<Utc>>,
}

impl From<&DemographicFormVersion> for VersionSummary {
    fn from(version: &DemographicFormVersion) -> Self {
        Self {
            id: version.id,
            version_number: version.version_number,
            status: version.status,
            published_at: version.published_at,
        }
    }
}

impl From<&DemographicQuestionVersion> for VersionSummary {
    fn from(version: &DemographicQuestionVersion) -> Self {
        Self {
            id: version.id,
            version_number: version.version_number,
            status: version.status,
            published_at: version.published_at,
        }
    }
}

/// An option of a select question, with its label in the requested locale.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct OptionDto {
    /// The unique identifier of the option.
    pub id: Uuid,
    /// The position of the option in the list of options.
    pub position: i32,
    /// The label of the option.
    pub label: String,
}

/// A question version with its text in the requested locale.
///
/// See [`DemographicQuestionVersion`] for the underlying row.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct QuestionVersionDto {
    /// The question version.
    #[serde(flatten)]
    pub version: DemographicQuestionVersion,
    /// The text of the question.
    pub text: String,
    /// The options of the question, for select questions.
    pub options: Vec<OptionDto>,
    /// The consent text shown for special category questions.
    pub consent_text: Option<String>,
}

/// A question with its tags and versions.
///
/// See [`QuestionRow`] for the underlying row.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct QuestionDto {
    /// The question.
    #[serde(flatten)]
    pub question: QuestionRow,
    /// The tags assigned to the question.
    pub tags: Vec<DemographicTag>,
    /// Every version of the question.
    pub versions: Vec<VersionSummary>,
    /// The latest published version, if any.
    pub latest_published: Option<QuestionVersionDto>,
    /// The draft version, if any.
    pub draft: Option<QuestionVersionDto>,
}

/// A form with its versions and draft.
///
/// See [`DemographicForm`] for the underlying row.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct DemographicFormDto {
    /// The form.
    #[serde(flatten)]
    pub form: DemographicForm,
    /// Every version of the form.
    pub versions: Vec<VersionSummary>,
    /// The latest published version, if any.
    pub latest_published: Option<DemographicFormVersionDto>,
    /// The draft version, if any.
    pub draft: Option<DemographicFormVersionDto>,
}

/// A form version with its entries.
///
/// See [`DemographicFormVersion`] for the underlying row.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct DemographicFormVersionDto {
    /// The form version.
    #[serde(flatten)]
    pub version: DemographicFormVersion,
    /// The questions in the form version.
    pub entries: Vec<DemographicFormEntryDto>,
}

/// A form entry with the text of the question it asks.
///
/// See [`DemographicFormVersionQuestion`] for the underlying row.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct DemographicFormEntryDto {
    /// The form entry.
    #[serde(flatten)]
    pub entry: DemographicFormVersionQuestion,
    /// The text of the question in the requested locale.
    pub question_text: String,
}

/// A response with the question its version belongs to, which is the only thing a
/// [`DemographicResponse`] row lacks.
#[derive(Debug, Clone, Serialize, JsonSchema, sqlx::FromRow)]
pub struct ResponseDto {
    /// The identifier of this answer.
    pub id: Uuid,
    /// The question version that was answered.
    pub question_version_id: Uuid,
    /// The question that was answered.
    pub question_id: Uuid,
    /// The answer, whose shape depends on the question type.
    pub value: Value,
    /// The conversation the answer was given in, if any.
    pub conversation_id: Option<Uuid>,
    /// When the answer was given.
    pub created_at: DateTime<Utc>,
}

/// One of the participant's saved answers with the question version it answers.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct MyDemographicAnswer {
    /// The question version the answer was given to.
    pub version: QuestionVersionDto,
    /// The saved answer, reused the next time the question is asked.
    pub answer: ResponseDto,
    /// The participant's consent to share a special category answer.
    pub consent: ConsentState,
}

/// A question a participant is asked by a demographic step, with what the server already knows
/// about their answer so the client can skip what is reusable or declined.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct PendingQuestion {
    /// The version of the question to ask.
    pub version: QuestionVersionDto,
    /// Whether the question must be answered.
    pub required: bool,
    /// The participant's consent to share a special category answer.
    pub consent: ConsentState,
    /// The existing answer when it can be reused for this exact version.
    pub answer: Option<ResponseDto>,
    /// True when the participant declined consent, so the question is not asked.
    pub skipped: bool,
}

/// The answers of a workflow's participants to the demographic questions it asks.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct WorkflowDemographics {
    /// The number of participants in the workflow.
    pub total_participants: i64,
    /// Answer counts keyed by question id.
    pub questions: HashMap<Uuid, QuestionCounts>,
}

/// How often each answer was given to one question.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct QuestionCounts {
    /// The question the counts are for.
    #[serde(flatten)]
    pub question: QuestionText,
    /// The answers given, most common first.
    pub answers: Vec<AnswerCount>,
}

/// The number of participants who gave one answer.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AnswerCount {
    /// The answer.
    pub value: Value,
    /// The number of participants who gave it.
    pub count: i64,
}

/// A question with the text of its latest published version in the requested locale.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct QuestionText {
    /// The question.
    pub question_id: Uuid,
    /// The text of the question in the requested locale.
    pub text: String,
}

/// A participant row in the conversation export.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ExportParticipant {
    /// The participant.
    pub user_id: Uuid,
    /// When the participant's profile was created.
    pub profile_created_at: DateTime<Utc>,
    /// Latest answer value per question id. Questions without an answer are absent.
    pub answers: HashMap<Uuid, Value>,
}

/// The demographic answers of a conversation's consenting participants.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ConversationDemographicsExport {
    /// Ordered by question text, so the CSV columns are stable.
    pub questions: Vec<QuestionText>,
    /// The consenting participants, newest profile first.
    pub participants: Vec<ExportParticipant>,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// The text content written for a draft. Text content is immutable, so every save writes
/// new rows for the text fields.
pub(super) struct DraftText {
    pub(super) text: TextContentId,
    pub(super) consent: Option<TextContentId>,
    pub(super) options: Vec<TextContentId>,
}
