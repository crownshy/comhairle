use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[schemars(
    description = "A pointer to the utterance in the transcript that supports a finding"
)]
pub struct TranscriptReference {
    /// The zero-based index of the referenced utterance within the transcript's `parts` array
    pub utterance_index: usize,
    /// The speaker_id of the referenced utterance
    pub speaker_id: String,
    /// A short quote taken verbatim from the referenced utterance
    pub quote: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[schemars(description = "A point of friction or unresolved concern in the conversation")]
pub struct TensionPoint {
    /// A description of the tension and why it matters
    pub description: String,
    /// The speakers involved in this tension
    pub speaker_ids: Vec<String>,
    /// Utterances that evidence this tension
    pub references: Vec<TranscriptReference>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[schemars(description = "One speaker's position within a point of disagreement")]
pub struct DisagreementPosition {
    /// The speaker holding this position
    pub speaker_id: String,
    /// A description of the position held
    pub position: String,
    /// Utterances that evidence this position
    pub references: Vec<TranscriptReference>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[schemars(description = "A topic on which two or more speakers held opposing positions")]
pub struct Disagreement {
    /// The topic of disagreement
    pub topic: String,
    /// The opposing positions held, one per speaker
    pub positions: Vec<DisagreementPosition>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[schemars(description = "A moment where a speaker changed their mind during the conversation")]
pub struct PivotalMoment {
    /// The speaker whose view changed
    pub speaker_id: String,
    /// What the speaker's view was before the change
    pub previous_view: String,
    /// What triggered the change (an argument, new information, another speaker's statement)
    pub trigger: String,
    /// What the speaker's view became after the change
    pub new_view: String,
    /// Utterances that evidence the previous view, the trigger, and the new view
    pub references: Vec<TranscriptReference>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[schemars(
    description = "An analysis of a speaker-attributed conversation transcript: a summary, points of tension, disagreements, and moments where a speaker changed their mind"
)]
pub struct ConversationAnalysis {
    /// A concise summary of the conversation
    pub summary: String,
    /// Utterances that support the summary
    pub summary_references: Vec<TranscriptReference>,
    /// Points of friction or unresolved concern in the conversation
    pub tensions: Vec<TensionPoint>,
    /// Topics on which speakers held opposing positions
    pub disagreements: Vec<Disagreement>,
    /// Moments where a speaker changed their mind
    pub pivotal_moments: Vec<PivotalMoment>,
}
