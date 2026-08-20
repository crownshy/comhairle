use core::error;
use std::{collections::BTreeMap, env, fs, io, path::Path};

fn main() -> Result<(), Box<dyn error::Error>> {
    let main_language = get_main_language();
    let file = get_translation(main_language.as_str())?;

    for key in file.keys() {
        println!("{}: {}", key, file[key]);
    }

    Ok(())
}

fn get_main_language() -> String {
    env::var("MAIN_LANGUAGE").unwrap_or(String::from("en"))
}

fn get_translation(language: &str) -> Result<BTreeMap<String, String>, io::Error> {
    let contents = fs::read_to_string(Path::new(&format!("../comhairle/messages/{language}.json")))?;
    let json: BTreeMap<String, String> = serde_json::from_str(contents.as_str())?;
    Ok(json)
}
