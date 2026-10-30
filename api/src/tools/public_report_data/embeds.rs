//! Which section blocks a report embeds, read from the report's own text so it can't drift from
//! what readers see (ADR-0061).

use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::report::Report;
use crate::models::translations::get_text_translations_by_content_id;

/// The TipTap node type the report editor stores for an embed (`reportComponentEmbed.ts`).
const EMBED_NODE_TYPE: &str = "reportComponentEmbed";

#[derive(Serialize, JsonSchema, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedSectionBlock {
    pub workflow_step_id: Uuid,
    /// The section block type, as listed in the frontend registry (`lib/reports/embeds.ts`).
    pub section_block: String,
}

/// Every section block embedded in the report's summary or body, in any language, without
/// duplicates.
pub async fn embedded_section_blocks(
    db: &PgPool,
    report: &Report,
) -> Result<Vec<EmbeddedSectionBlock>, ComhairleError> {
    let mut documents = get_text_translations_by_content_id(db, &report.summary).await?;
    if let Some(body) = &report.body {
        documents.extend(get_text_translations_by_content_id(db, body).await?);
    }

    let mut section_blocks: Vec<EmbeddedSectionBlock> = documents
        .iter()
        .flat_map(|translation| section_blocks_in_document(&translation.content))
        .collect();
    section_blocks.sort();
    section_blocks.dedup();
    Ok(section_blocks)
}

/// The embeds in one stored TipTap document. Text that isn't TipTap JSON (older plain-text
/// summaries) has no embeds, so nothing from it is served.
pub fn section_blocks_in_document(document: &str) -> Vec<EmbeddedSectionBlock> {
    let Ok(root) = serde_json::from_str::<Value>(document) else {
        return vec![];
    };
    let mut section_blocks = vec![];
    collect_embeds(&root, &mut section_blocks);
    section_blocks
}

fn collect_embeds(node: &Value, section_blocks: &mut Vec<EmbeddedSectionBlock>) {
    if node["type"] == EMBED_NODE_TYPE {
        let attributes = &node["attrs"];
        let workflow_step_id = attributes["toolStepId"]
            .as_str()
            .and_then(|id| Uuid::parse_str(id).ok());
        let section_block = attributes["componentType"].as_str();
        if let (Some(workflow_step_id), Some(section_block)) = (workflow_step_id, section_block) {
            section_blocks.push(EmbeddedSectionBlock {
                workflow_step_id,
                section_block: section_block.to_owned(),
            });
        }
    }
    if let Some(children) = node["content"].as_array() {
        for child in children {
            collect_embeds(child, section_blocks);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn embed(step: &str, section_block: &str) -> Value {
        json!({
            "type": EMBED_NODE_TYPE,
            "attrs": { "toolStepId": step, "componentType": section_block, "config": {} }
        })
    }

    #[test]
    fn should_find_embeds_at_any_depth() {
        let step = Uuid::new_v4();
        let document = json!({
            "type": "doc",
            "content": [
                { "type": "paragraph", "content": [{ "type": "text", "text": "Intro" }] },
                embed(&step.to_string(), "polis-key-stats"),
                { "type": "blockquote", "content": [embed(&step.to_string(), "polis-opinion-groups")] }
            ]
        });

        assert_eq!(
            section_blocks_in_document(&document.to_string()),
            vec![
                EmbeddedSectionBlock {
                    workflow_step_id: step,
                    section_block: "polis-key-stats".into()
                },
                EmbeddedSectionBlock {
                    workflow_step_id: step,
                    section_block: "polis-opinion-groups".into()
                },
            ]
        );
    }

    #[test]
    fn should_skip_embeds_with_a_missing_or_invalid_reference() {
        let document = json!({
            "type": "doc",
            "content": [
                embed("not-a-uuid", "polis-key-stats"),
                { "type": EMBED_NODE_TYPE, "attrs": { "toolStepId": Uuid::new_v4().to_string() } }
            ]
        });

        assert!(section_blocks_in_document(&document.to_string()).is_empty());
    }

    #[test]
    fn should_find_nothing_in_text_that_is_not_tiptap_json() {
        assert!(section_blocks_in_document("A plain summary").is_empty());
        assert!(section_blocks_in_document("").is_empty());
    }
}
