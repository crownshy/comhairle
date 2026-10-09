use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result, bail};
use language_tags::LanguageTag;
use sensemakar_types::{Statement, WikiPollStatement, WikiPollVoteSummary};

/// Parses a Polis-style vote CSV export.
///
/// Expected columns: `statement`, `agree`, `disagree`, `pass`, plus any number
/// of per-group columns named `<group>_agree`, `<group>_disagree`, `<group>_pass`
/// (e.g. `A_agree`, `A_disagree`, `A_pass`, `B_agree`, ...).
fn load_csv(path: &Path) -> Result<Vec<WikiPollStatement>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV file {}", path.display()))?;
    let headers = reader.headers()?.clone();

    let mut statements = Vec::new();
    for record in reader.records() {
        let record = record.with_context(|| format!("failed to parse row in {}", path.display()))?;

        let mut statement = None;
        let mut total_votes = WikiPollVoteSummary { agree: 0, disagree: 0, pass: 0 };
        let mut group_votes: HashMap<String, WikiPollVoteSummary> = HashMap::new();

        for (header, value) in headers.iter().zip(record.iter()) {
            match header {
                "statement" => statement = Some(value.to_string()),
                "agree" | "disagree" | "pass" => {
                    let parsed: u32 = value
                        .parse()
                        .with_context(|| format!("column '{header}' is not a valid vote count"))?;
                    match header {
                        "agree" => total_votes.agree = parsed,
                        "disagree" => total_votes.disagree = parsed,
                        _ => total_votes.pass = parsed,
                    }
                }
                other => {
                    let Some((group, field)) = other.rsplit_once('_') else {
                        continue;
                    };
                    if !matches!(field, "agree" | "disagree" | "pass") {
                        bail!("unrecognised column '{other}'");
                    }
                    let parsed: u32 = value
                        .parse()
                        .with_context(|| format!("column '{other}' is not a valid vote count"))?;
                    let summary = group_votes.entry(group.to_string()).or_insert(WikiPollVoteSummary {
                        agree: 0,
                        disagree: 0,
                        pass: 0,
                    });
                    match field {
                        "agree" => summary.agree = parsed,
                        "disagree" => summary.disagree = parsed,
                        _ => summary.pass = parsed,
                    }
                }
            }
        }

        let statement = statement.context("CSV row is missing a 'statement' column")?;
        statements.push(WikiPollStatement {
            statement,
            total_votes,
            group_votes,
        });
    }

    Ok(statements)
}

fn load_json(path: &Path) -> Result<Vec<WikiPollStatement>> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    let statements: Vec<WikiPollStatement> = serde_json::from_str(&data)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))?;
    Ok(statements)
}

/// Loads Polis statements from a CSV or JSON file, inferring the format from
/// the file extension (`.csv` or `.json`).
pub fn load_statements(path: &Path) -> Result<Vec<WikiPollStatement>> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("csv") => load_csv(path),
        Some("json") => load_json(path),
        other => bail!(
            "unsupported file extension {:?} for {}, expected .csv or .json",
            other,
            path.display()
        ),
    }
}

/// Converts WikiPoll statements (plain text + votes, no stable id) into the
/// generic `Statement` shape used by the theme extraction and assignment
/// tasks. The statement's position in the input is used as its id, since
/// WikiPoll statements don't otherwise carry one; vote data is dropped.
pub fn to_statements(statements: Vec<WikiPollStatement>, lang: &LanguageTag) -> Vec<Statement> {
    statements
        .into_iter()
        .enumerate()
        .map(|(index, statement)| Statement {
            id: index.to_string(),
            speaker_id: None,
            statement_type: None,
            text: statement.statement,
            lang: lang.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn loads_csv_with_group_votes() {
        let mut file = tempfile::NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(
            file,
            "statement,agree,disagree,pass,A_agree,A_disagree,A_pass,B_agree,B_disagree,B_pass"
        )
        .unwrap();
        writeln!(file, "\"Bikes are great\",10,2,1,5,1,0,5,1,1").unwrap();

        let statements = load_statements(file.path()).unwrap();
        assert_eq!(statements.len(), 1);
        let statement = &statements[0];
        assert_eq!(statement.statement, "Bikes are great");
        assert_eq!(statement.total_votes.agree, 10);
        assert_eq!(statement.group_votes["A"].agree, 5);
        assert_eq!(statement.group_votes["B"].disagree, 1);
    }

    #[test]
    fn loads_json() {
        let mut file = tempfile::NamedTempFile::with_suffix(".json").unwrap();
        write!(
            file,
            r#"[{{"statement":"Bikes are great","total_votes":{{"agree":10,"disagree":2,"pass":1}},"group_votes":{{}}}}]"#
        )
        .unwrap();

        let statements = load_statements(file.path()).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].statement, "Bikes are great");
    }

    #[test]
    fn converts_to_statements_with_positional_ids() {
        let statements = vec![
            WikiPollStatement {
                statement: "Bikes are great".into(),
                total_votes: WikiPollVoteSummary { agree: 10, disagree: 2, pass: 1 },
                group_votes: HashMap::new(),
            },
            WikiPollStatement {
                statement: "Buses are late".into(),
                total_votes: WikiPollVoteSummary { agree: 5, disagree: 1, pass: 0 },
                group_votes: HashMap::new(),
            },
        ];
        let lang = LanguageTag::parse("en").unwrap();

        let converted = to_statements(statements, &lang);
        assert_eq!(converted.len(), 2);
        assert_eq!(converted[0].id, "0");
        assert_eq!(converted[0].text, "Bikes are great");
        assert_eq!(converted[1].id, "1");
        assert_eq!(converted[1].lang, lang);
    }
}
