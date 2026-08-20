use std::{collections::BTreeMap, env, fs, path::Path};

pub fn get_main_language() -> String {
    env::var("MAIN_LANGUAGE").unwrap_or(String::from("en"))
}

pub fn get_file(language: &str) -> Result<BTreeMap<String, String>, std::io::Error> {
    let contents =
        fs::read_to_string(Path::new(&format!("../comhairle/messages/{language}.json")))?;
    let json: BTreeMap<String, String> = serde_json::from_str(contents.as_str())?;
    Ok(json)
}
