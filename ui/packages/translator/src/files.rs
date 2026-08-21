use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

const UNSAFE_LANGUAGES_DIR: &str = "../comhairle/messages/";

pub struct LanguageFiles;
impl LanguageFiles {
    fn dir() -> PathBuf {
        Path::new(UNSAFE_LANGUAGES_DIR).to_path_buf()
    }

    fn path(language: &str) -> PathBuf {
        Path::new(UNSAFE_LANGUAGES_DIR).join(format!("{language}.json"))
    }

    fn languages<'a>()
    -> Result<impl Iterator<Item = Result<Option<OsString>, std::io::Error>>, std::io::Error> {
        let dir = fs::read_dir(Path::new(UNSAFE_LANGUAGES_DIR))?;
        let languages =
            dir.map(|entry| entry.map(|e| e.path().file_prefix().map(|p| p.to_owned())));
        Ok(languages)
    }
}
// Export block
pub fn languages()
-> Result<impl Iterator<Item = Result<Option<OsString>, std::io::Error>>, std::io::Error> {
    LanguageFiles::languages()
}
// End export block

pub fn read(language: &str) -> Result<BTreeMap<String, String>, std::io::Error> {
    let contents = fs::read_to_string(LanguageFiles::path(language))?;
    let json: BTreeMap<String, String> = serde_json::from_str(contents.as_str())?;
    Ok(json)
}

pub fn write(language: &str, language_maps: &BTreeMap<String, String>) -> () {
    let contents = match serde_json::to_string_pretty(&language_maps) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not convert to json: {e:?}");
            return ();
        }
    };

    // to_string_pretty only seems to use spaces, so converting them to tabs here
    let contents = contents
        .lines()
        .map(|line| {
            let spaces = line.chars().take_while(|c| *c == ' ').count();
            let tabs = spaces / 2;
            "\t".repeat(tabs) + &line[spaces..]
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    match fs::write(LanguageFiles::path(language), contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Could not write to file: {e:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_dir() {
        let dir = LanguageFiles::dir();
        let Some(dir) = dir.to_str() else {
            panic!("Couldn't convert dir");
        };
        assert_eq!(dir, "../comhairle/messages/");
    }

    #[test]
    fn should_get_path() {
        let file = LanguageFiles::path("en");
        let Some(file) = file.to_str() else {
            panic!("Couldn't convert path");
        };
        assert_eq!(file, "../comhairle/messages/en.json");
    }

    #[test]
    fn should_get_all_languages() {
        let paths = LanguageFiles::languages();
        let Ok(paths) = paths else {
            panic!("Couldn't read directory");
        };
        let paths: Vec<Result<Option<OsString>, std::io::Error>> = paths.collect();
        assert!(paths.len() > 1);
    }
}
