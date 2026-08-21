use crate::files;
use std::{
    collections::{BTreeMap, HashMap},
    env,
};

pub fn get_main_language() -> String {
    env::var("MAIN_LANGUAGE").unwrap_or(String::from("en"))
}

pub fn get_language_maps() -> Result<HashMap<String, BTreeMap<String, String>>, std::io::Error> {
    let mut languages: HashMap<String, BTreeMap<String, String>> = HashMap::new();

    for language in files::languages()? {
        let language = match language {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        let Some(language) = language else {
            eprintln!("Could not find file name");
            continue;
        };

        let language = match language.into_string() {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{e:?}");
                continue;
            }
        };

        let file = files::read(&language);
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{e:?}");
                continue;
            }
        };

        languages.insert(language, file);
    }

    Ok(languages)
}
pub fn write_language_maps(language_maps: &HashMap<String, BTreeMap<String, String>>) -> () {
    for language in language_maps.keys() {
        files::write(&language, &language_maps[language]);
    }
}
