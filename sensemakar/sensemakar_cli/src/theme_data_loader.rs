use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use sensemakar_types::Theme;

/// Loads themes from a CSV or JSON file, inferring the format from the file
/// extension (`.csv` or `.json`).
///
/// CSV columns: `id`, `name`, `description`. JSON is a `Theme` array.
pub fn load_themes(path: &Path) -> Result<Vec<Theme>> {
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

fn load_csv(path: &Path) -> Result<Vec<Theme>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV file {}", path.display()))?;

    let mut themes = Vec::new();
    for record in reader.deserialize() {
        let theme: Theme = record.with_context(|| format!("failed to parse row in {}", path.display()))?;
        themes.push(theme);
    }

    Ok(themes)
}

fn load_json(path: &Path) -> Result<Vec<Theme>> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    let themes: Vec<Theme> = serde_json::from_str(&data)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))?;
    Ok(themes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn loads_csv() {
        let mut file = tempfile::NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(file, "id,name,description").unwrap();
        writeln!(file, "1,Transport,Statements about transportation").unwrap();

        let themes = load_themes(file.path()).unwrap();
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].name, "Transport");
    }

    #[test]
    fn loads_json() {
        let mut file = tempfile::NamedTempFile::with_suffix(".json").unwrap();
        write!(
            file,
            r#"[{{"id":"1","name":"Transport","description":"Statements about transportation"}}]"#
        )
        .unwrap();

        let themes = load_themes(file.path()).unwrap();
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].name, "Transport");
    }
}
