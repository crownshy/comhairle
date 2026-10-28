//! The only types a public report-data response may contain (ADR-0061).
//!
//! A response type is declared with [`public_report_data!`], which refuses to compile if any
//! field isn't [`PublicReportData`]. Ids, pids, emails and raw strings never are, so they can't
//! reach a public endpoint by accident. Adding a new building block here needs a reviewer to
//! agree it can't identify a person.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        unused_macros,
        unused_imports,
        reason = "first users are the Polis public endpoints (#1253)"
    )
)]

use schemars::JsonSchema;
use serde::Serialize;

/// Data that is safe to serve to anyone once a section block is public. Implemented by the
/// building blocks in this module and by structs declared with [`public_report_data!`], never by
/// hand.
pub trait PublicReportData: sealed::Sealed + Serialize {}

#[doc(hidden)]
pub mod sealed {
    /// Keeps [`super::PublicReportData`] to this module and the macro.
    pub trait Sealed {}
}

/// How many of something there are, such as participants, statements or votes.
#[derive(Serialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(transparent)]
pub struct Count(pub u64);

/// A proportion between 0 and 1, such as the share of a group that agreed.
#[derive(Serialize, JsonSchema, Debug, Clone, Copy, PartialEq)]
#[serde(transparent)]
pub struct Share(pub f64);

/// Participant-written text that moderation has accepted. Free text can carry anything a
/// participant typed, so this is the one door for it: build it only from accepted content, never
/// from rejected or pending content.
#[derive(Serialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct ModeratedText(String);

impl ModeratedText {
    pub fn from_accepted(text: String) -> Self {
        Self(text)
    }
}

macro_rules! building_blocks {
    ($($building_block:ty),*) => {
        $(
            impl sealed::Sealed for $building_block {}
            impl PublicReportData for $building_block {}
        )*
    };
}

building_blocks!(Count, Share, ModeratedText, bool, &'static str);

impl<T: PublicReportData> sealed::Sealed for Option<T> {}
impl<T: PublicReportData> PublicReportData for Option<T> {}
impl<T: PublicReportData> sealed::Sealed for Vec<T> {}
impl<T: PublicReportData> PublicReportData for Vec<T> {}

/// Declare a public report-data response struct. Every field must itself be
/// [`PublicReportData`], or the crate doesn't compile:
///
/// ```ignore
/// public_report_data! {
///     pub struct KeyStats {
///         pub participants: Count,
///         pub statements: Count,
///     }
/// }
/// ```
macro_rules! public_report_data {
    (
        $(#[$struct_attribute:meta])*
        $visibility:vis struct $name:ident {
            $($(#[$field_attribute:meta])* $field_visibility:vis $field:ident : $field_type:ty),* $(,)?
        }
    ) => {
        $(#[$struct_attribute])*
        #[derive(Debug, serde::Serialize, schemars::JsonSchema)]
        #[serde(rename_all = "camelCase")]
        $visibility struct $name {
            $($(#[$field_attribute])* $field_visibility $field: $field_type),*
        }

        impl $crate::tools::public_report_data::anonymous::sealed::Sealed for $name {}

        // A field type that isn't public report data fails this bound at compile time.
        impl $crate::tools::public_report_data::anonymous::PublicReportData for $name
        where
            $($field_type: $crate::tools::public_report_data::anonymous::PublicReportData),*
        {}
    };
}

pub(crate) use public_report_data;

#[cfg(test)]
mod tests {
    use super::*;

    public_report_data! {
        struct ExampleSectionBlock {
            participants: Count,
            agreed: Option<Share>,
            statements: Vec<ModeratedText>,
            group: &'static str,
        }
    }

    #[test]
    fn should_serialize_building_blocks_as_plain_values() {
        let data = ExampleSectionBlock {
            participants: Count(12),
            agreed: Some(Share(0.5)),
            statements: vec![ModeratedText::from_accepted("More buses".into())],
            group: "A",
        };

        assert_eq!(
            serde_json::to_value(&data).unwrap(),
            serde_json::json!({
                "participants": 12,
                "agreed": 0.5,
                "statements": ["More buses"],
                "group": "A"
            })
        );
    }
}
