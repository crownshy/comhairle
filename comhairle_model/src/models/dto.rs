//! DTOs the model layer itself embeds.
//!
//! Most DTOs live next to their routes (`routes/<domain>/dto.rs`). The ones here
//! are different: the translation machinery (`TranslationDto`,
//! `JsonFieldWithTranslations`, the `Translatable` derives) embeds them, so they
//! must live at or below the model layer for the crate split. The route module
//! re-exports them, so route-layer imports are unchanged.

pub mod translations;
