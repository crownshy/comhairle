use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use language_tags::LanguageTag;
use sensemakar_types::Statement;
use serde::Deserialize;

/// A single row of a statement CSV file.
///
/// Expected columns: `id`, `text`, plus optional `speaker_id` and `lang`
/// (BCP-47 language tag, e.g. `en`). When `lang` is omitted the `--lang`
/// CLI default is used.
#[derive(Debug, Deserialize)]
struct CsvRow {
    id: String,
    text: String,
    speaker_id: Option<String>,
    lang: Option<String>,
}

fn load_csv(path: &Path, default_lang: &LanguageTag) -> Result<Vec<Statement>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV file {}", path.display()))?;

    let mut statements = Vec::new();
    for record in reader.deserialize() {
        let row: CsvRow = record.with_context(|| format!("failed to parse row in {}", path.display()))?;
        let lang = match row.lang {
            Some(tag) => LanguageTag::parse(&tag)
                .with_context(|| format!("invalid language tag '{tag}' for statement {}", row.id))?,
            None => default_lang.clone(),
        };
        statements.push(Statement {
            id: row.id,
            speaker_id: row.speaker_id,
            statement_type: None,
            text: row.text,
            lang,
        });
    }

    Ok(statements)
}

fn load_json(path: &Path) -> Result<Vec<Statement>> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    let statements: Vec<Statement> = serde_json::from_str(&data)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))?;
    Ok(statements)
}

/// Loads statements from a CSV or JSON file, inferring the format from the
/// file extension (`.csv` or `.json`). CSV rows without a `lang` column fall
/// back to `default_lang`; JSON is expected to already carry a full
/// `Statement` per entry.
pub fn load_statements(path: &Path, default_lang: &LanguageTag) -> Result<Vec<Statement>> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("csv") => load_csv(path, default_lang),
        Some("json") => load_json(path),
        other => bail!(
            "unsupported file extension {:?} for {}, expected .csv or .json",
            other,
            path.display()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn en() -> LanguageTag {
        LanguageTag::parse("en").unwrap()
    }

    #[test]
    fn loads_csv_with_default_lang() {
        let mut file = tempfile::NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(file, "id,text,speaker_id,lang").unwrap();
        writeln!(file, "1,Bikes are great,alice,").unwrap();
        writeln!(file, "2,Buses are late,,fr").unwrap();

        let statements = load_statements(file.path(), &en()).unwrap();
        assert_eq!(statements.len(), 2);
        assert_eq!(statements[0].text, "Bikes are great");
        assert_eq!(statements[0].speaker_id, Some("alice".to_string()));
        assert_eq!(statements[0].lang, en());
        assert_eq!(statements[1].lang, LanguageTag::parse("fr").unwrap());
    }

    #[test]
    fn loads_json() {
        let mut file = tempfile::NamedTempFile::with_suffix(".json").unwrap();
        write!(
            file,
            r#"[{{"id":"1","speaker_id":null,"statement_type":null,"text":"Bikes are great","lang":"en"}}]"#
        )
        .unwrap();

        let statements = load_statements(file.path(), &en()).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].text, "Bikes are great");
    }
}
