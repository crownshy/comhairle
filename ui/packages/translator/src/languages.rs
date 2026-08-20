use std::env;

pub fn get_main_language() -> String {
    env::var("MAIN_LANGUAGE").unwrap_or(String::from("en"))
}
