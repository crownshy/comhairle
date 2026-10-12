use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use uuid::Uuid;

use super::questions::{QuestionDraftInput, QuestionVersionDto};
use super::{DatePrecision, LocationFormat, QuestionType};
use crate::error::ComhairleError;

pub fn invalid(message: impl Into<String>) -> ComhairleError {
    ComhairleError::BadRequest(message.into())
}

fn ensure(condition: bool, message: &str) -> Result<(), ComhairleError> {
    if condition {
        Ok(())
    } else {
        Err(invalid(message))
    }
}

pub fn validate_question_draft(input: &QuestionDraftInput) -> Result<(), ComhairleError> {
    ensure(!input.text.trim().is_empty(), "question text is required")?;
    validate_type_fields(input)?;
    validate_options(input)?;
    validate_consent(input)
}

fn validate_type_fields(input: &QuestionDraftInput) -> Result<(), ComhairleError> {
    let kind = input.question_type;
    ensure(
        (kind == QuestionType::Date) == input.date_precision.is_some(),
        "date_precision is required for date questions and only allowed there",
    )?;
    ensure(
        (kind == QuestionType::Location) == input.location_format.is_some(),
        "location_format is required for location questions and only allowed there",
    )?;
    ensure(
        input.location_country.is_none() || kind == QuestionType::Location,
        "location_country is only allowed for location questions",
    )?;
    ensure(
        input
            .location_country
            .as_deref()
            .is_none_or(|country| !country.trim().is_empty()),
        "location_country must not be blank",
    )?;
    ensure(
        input.max_selections.is_none()
            || (kind == QuestionType::MultiSelect
                && input.max_selections.is_some_and(|max| max >= 1)),
        "max_selections is only allowed for multi_select questions and must be at least 1",
    )?;
    ensure(
        kind.is_select() || (!input.allow_other && !input.prefer_not_to_say),
        "allow_other and prefer_not_to_say are only allowed for select questions",
    )?;
    ensure(
        (kind == QuestionType::Custom) == input.json_schema.is_some(),
        "json_schema is required for custom questions and only allowed there",
    )?;
    input
        .json_schema
        .as_ref()
        .map_or(Ok(()), validate_json_schema)
}

fn validate_options(input: &QuestionDraftInput) -> Result<(), ComhairleError> {
    ensure(
        input.question_type.is_select() != input.options.is_empty(),
        "select questions need at least one option, other questions must not have options",
    )?;
    ensure(
        input.options.iter().all(|label| !label.trim().is_empty()),
        "option labels must not be blank",
    )
}

fn validate_consent(input: &QuestionDraftInput) -> Result<(), ComhairleError> {
    ensure(
        input.special_category == input.consent_text.is_some(),
        "consent_text is required for special category questions and only allowed there",
    )?;
    ensure(
        input
            .consent_text
            .as_deref()
            .is_none_or(|text| !text.trim().is_empty()),
        "consent_text must not be blank",
    )
}

pub fn validate_json_schema(schema: &Value) -> Result<(), ComhairleError> {
    jsonschema::validator_for(schema)
        .map(|_| ())
        .map_err(|error| invalid(format!("json_schema is not a valid JSON schema: {error}")))
}

/// Checks an answer against the shape the version's question type requires.
pub fn validate_answer(version: &QuestionVersionDto, value: &Value) -> Result<(), ComhairleError> {
    match version.question_type {
        QuestionType::Number => ensure(value.is_number(), "answer must be a number"),
        QuestionType::Date => validate_date(version.date_precision, value),
        QuestionType::AnyString => validate_non_blank_string(value),
        QuestionType::Location => validate_location(version.location_format, value),
        QuestionType::SingleSelect | QuestionType::MultiSelect => {
            validate_selection(version, value)
        }
        QuestionType::Custom => validate_custom(version.json_schema.as_ref(), value),
    }
}

fn validate_date(precision: Option<DatePrecision>, value: &Value) -> Result<(), ComhairleError> {
    let text = value
        .as_str()
        .ok_or_else(|| invalid("date answer must be a string"))?;
    let parts: Vec<&str> = text.split('/').collect();
    let valid = match (precision, parts.as_slice()) {
        (Some(DatePrecision::Year), [year]) => is_calendar_date(year, None, None),
        (Some(DatePrecision::Month), [year, month]) => is_calendar_date(year, Some(month), None),
        (Some(DatePrecision::Day), [year, month, day]) => {
            is_calendar_date(year, Some(month), Some(day))
        }
        _ => false,
    };
    ensure(valid, "date answer does not match the question's precision")
}

fn is_calendar_date(year: &str, month: Option<&str>, day: Option<&str>) -> bool {
    let (Some(year), Some(month), Some(day)) = (
        fixed_digits(year, 4),
        month.map_or(Some(1), |m| fixed_digits(m, 2)),
        day.map_or(Some(1), |d| fixed_digits(d, 2)),
    ) else {
        return false;
    };
    NaiveDate::from_ymd_opt(year as i32, month, day).is_some()
}

fn fixed_digits(text: &str, length: usize) -> Option<u32> {
    if text.len() == length && text.bytes().all(|b| b.is_ascii_digit()) {
        text.parse().ok()
    } else {
        None
    }
}

fn validate_non_blank_string(value: &Value) -> Result<(), ComhairleError> {
    let text = value
        .as_str()
        .ok_or_else(|| invalid("answer must be a string"))?;
    ensure(!text.trim().is_empty(), "answer must not be blank")
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PostcodeAnswer {
    postcode: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LatLongAnswer {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CountryAnswer {
    country: String,
}

fn validate_location(format: Option<LocationFormat>, value: &Value) -> Result<(), ComhairleError> {
    match format.ok_or_else(|| invalid("location question has no format"))? {
        LocationFormat::Postcode => {
            let answer: PostcodeAnswer = parse_answer(value)?;
            ensure(
                !answer.postcode.trim().is_empty(),
                "postcode must not be blank",
            )
        }
        LocationFormat::LatLong => {
            let answer: LatLongAnswer = parse_answer(value)?;
            ensure(
                (-90.0..=90.0).contains(&answer.latitude),
                "latitude must be between -90 and 90",
            )?;
            ensure(
                (-180.0..=180.0).contains(&answer.longitude),
                "longitude must be between -180 and 180",
            )
        }
        LocationFormat::Country => {
            let answer: CountryAnswer = parse_answer(value)?;
            ensure(
                !answer.country.trim().is_empty(),
                "country must not be blank",
            )
        }
    }
}

fn parse_answer<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T, ComhairleError> {
    serde_json::from_value(value.clone())
        .map_err(|error| invalid(format!("invalid answer: {error}")))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionAnswer {
    #[serde(default)]
    pub option_ids: Vec<Uuid>,
    #[serde(default)]
    pub other_text: Option<String>,
    #[serde(default)]
    pub prefer_not_to_say: bool,
}

fn validate_selection(version: &QuestionVersionDto, value: &Value) -> Result<(), ComhairleError> {
    let answer: SelectionAnswer = parse_answer(value)?;
    let known: HashSet<Uuid> = version.options.iter().map(|option| option.id).collect();
    let chosen: HashSet<Uuid> = answer.option_ids.iter().copied().collect();

    ensure(
        chosen.len() == answer.option_ids.len(),
        "option_ids must not repeat",
    )?;
    ensure(
        chosen.is_subset(&known),
        "option_ids must belong to this question version",
    )?;
    match version.question_type {
        QuestionType::SingleSelect => ensure(
            answer.option_ids.len() <= 1,
            "single select answers allow at most one option",
        )?,
        QuestionType::MultiSelect => {
            if let Some(max) = version.max_selections {
                ensure(
                    answer.option_ids.len() <= max as usize,
                    "too many options selected",
                )?;
            }
        }
        _ => {}
    }

    let other = answer.other_text.as_deref();
    if let Some(text) = other {
        ensure(
            version.allow_other,
            "other text is not allowed for this question",
        )?;
        ensure(!text.trim().is_empty(), "other text must not be blank")?;
    }

    if answer.prefer_not_to_say {
        ensure(
            version.prefer_not_to_say,
            "prefer not to say is not allowed for this question",
        )?;
        ensure(
            answer.option_ids.is_empty() && other.is_none(),
            "prefer not to say cannot be combined with options or other text",
        )
    } else {
        ensure(
            !answer.option_ids.is_empty() || other.is_some(),
            "select an option, other text, or prefer not to say",
        )
    }
}

fn validate_custom(schema: Option<&Value>, value: &Value) -> Result<(), ComhairleError> {
    let schema = schema.ok_or_else(|| invalid("custom question has no json_schema"))?;
    let validator = jsonschema::validator_for(schema)
        .map_err(|error| invalid(format!("invalid json_schema: {error}")))?;
    validator
        .validate(value)
        .map_err(|error| invalid(format!("answer does not match the schema: {error}")))
}
