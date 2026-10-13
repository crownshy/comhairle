use language_tags::LanguageTag;
use rig_core::DynModel;
use rig_core::operation::Completion;
use rig_core::providers::ollama::Ollama;

use sensemakar_types::{Statement, Theme};

pub fn test_themes() -> Vec<Theme> {
    vec![
        Theme {
            name: "Transport".into(),
            description: "Statements about transportation".into(),
            id: "1".into(),
        },
        Theme {
            name: "Services".into(),
            description: "Statements about public services".into(),
            id: "2".into(),
        },
    ]
}

pub fn test_statements() -> Vec<Statement> {
    let en = LanguageTag::parse("en").unwrap();
    vec![
        Statement::new(
            "1",
            "The bus never turns up on time and I end up late for work.",
            en.clone(),
        ),
        Statement::new(
            "2",
            "There is nowhere safe to lock up my bike near the station.",
            en.clone(),
        ),
        Statement::new(
            "3",
            "Trams are packed solid every morning during rush hour.",
            en.clone(),
        ),
        Statement::new(
            "4",
            "The playground on the estate has been broken for two years.",
            en.clone(),
        ),
        Statement::new(
            "5",
            "We need more trees along the main road, it is bleak.",
            en.clone(),
        ),
        Statement::new(
            "6",
            "Rubbish collection has been missed three weeks running.",
            en.clone(),
        ),
        Statement::new(
            "7",
            "I do not feel safe walking home at night, the street lights are out.",
            en.clone(),
        ),
        Statement::new(
            "8",
            "Rent has gone up so much that my friends have all moved away.",
            en,
        ),
    ]
}

pub fn test_model() -> DynModel<Completion> {
    Ollama::new().completion("qwen3-long").erase()
}
