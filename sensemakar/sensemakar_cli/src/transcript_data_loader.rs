use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use jiff::Timestamp;
use sensemakar_types::{Transcript, Utterance};
use serde::Deserialize;
use uuid::Uuid;

/// A single row of a transcript CSV file.
///
/// Expected columns: `speaker_id`, `text`. Rows are assumed to be in the
/// order they were spoken; no timing information is captured.
#[derive(Debug, Deserialize)]
struct CsvRow {
    speaker_id: String,
    text: String,
}

fn load_csv(path: &Path) -> Result<Transcript> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV file {}", path.display()))?;

    let mut parts = Vec::new();
    for record in reader.deserialize() {
        let row: CsvRow =
            record.with_context(|| format!("failed to parse row in {}", path.display()))?;
        parts.push(Utterance {
            speaker_id: row.speaker_id,
            text: row.text,
            started_offset: 0,
            ended_offset: 0,
        });
    }

    Ok(Transcript {
        id: Uuid::new_v4().to_string(),
        parts,
        started_at: Timestamp::now(),
    })
}

fn load_json(path: &Path) -> Result<Transcript> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    let transcript: Transcript = serde_json::from_str(&data)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))?;
    Ok(transcript)
}

/// Loads a transcript from a CSV or JSON file, inferring the format from the
/// file extension (`.csv` or `.json`). CSV rows carry only `speaker_id` and
/// `text`, in spoken order; JSON is expected to already carry a full
/// `Transcript`.
pub fn load_transcript(path: &Path) -> Result<Transcript> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn loads_csv() {
        let mut file = tempfile::NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(file, "speaker_id,text").unwrap();
        writeln!(file, "alice,Bikes are great").unwrap();
        writeln!(file, "bob,Buses are late").unwrap();

        let transcript = load_transcript(file.path()).unwrap();
        assert_eq!(transcript.parts.len(), 2);
        assert_eq!(transcript.parts[0].speaker_id, "alice");
        assert_eq!(transcript.parts[1].text, "Buses are late");
    }

    #[test]
    fn loads_json() {
        let mut file = tempfile::NamedTempFile::with_suffix(".json").unwrap();
        write!(
            file,
            r#"{{"id":"t1","started_at":"2024-01-01T00:00:00Z","parts":[{{"speaker_id":"alice","text":"Bikes are great","started_offset":0,"ended_offset":1}}]}}"#
        )
        .unwrap();

        let transcript = load_transcript(file.path()).unwrap();
        assert_eq!(transcript.parts.len(), 1);
        assert_eq!(transcript.parts[0].text, "Bikes are great");
    }
}
