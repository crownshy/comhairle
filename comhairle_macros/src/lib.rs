mod translation_helpers;

use translation_helpers::{
    derive_translatable_enum_struct, derive_translatable_json_struct,
    is_optional_text_content_id_type, is_text_content_id_type, passthrough_serde_attrs,
    snake_case_to_pascal,
};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Macro used to allow an enum to be
/// saved as jsonb in the database
#[proc_macro_derive(DbJsonBEnum)]
pub fn derive_db_enum_jsonb(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl sqlx::Type<sqlx::Postgres> for #name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <serde_json::Value as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        impl sqlx::postgres::PgHasArrayType for #name {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <serde_json::Value as sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for #name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
                let json = serde_json::to_value(self).unwrap();
                <serde_json::Value as sqlx::Encode<sqlx::Postgres>>::encode(json, buf)
            }

            fn size_hint(&self) -> usize {
                let json = serde_json::to_value(self).unwrap();
                <serde_json::Value as sqlx::Encode<sqlx::Postgres>>::size_hint(&json)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for #name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                let json: serde_json::Value = sqlx::Decode::<sqlx::Postgres>::decode(value)?;
                Ok(serde_json::from_value(json)?)
            }
        }

        impl Into<sea_query::SimpleExpr> for #name{
            fn into(self) -> sea_query::SimpleExpr {
                serde_json::to_value(self).unwrap().into()
            }
        }

        impl Into<sea_query::SimpleExpr> for &#name{
            fn into(self) -> sea_query::SimpleExpr {
                serde_json::to_value(self).unwrap().into()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Macro used to allow an enum to be
/// saved as a string in the database
#[proc_macro_derive(DbStringEnum)]
pub fn db_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let data_enum = match input.data {
        Data::Enum(e) => e,
        _ => panic!("#[derive(DbStringEnum)] only works on enums"),
    };

    let variants: Vec<_> = data_enum.variants.into_iter().collect();

    let idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let names: Vec<String> = idents
        .iter()
        .map(|id| id.to_string().to_lowercase())
        .collect();

    let r#gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(Self::#idents => write!(f, "{}", #names),)*
                }
            }
        }

        impl std::str::FromStr for #name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#names => Ok(Self::#idents),)*
                    _ => Err(format!("Invalid {}: {}", stringify!(#name), s)),
                }
            }
        }

        impl sqlx::Type<sqlx::Postgres> for #name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <String as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for #name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                <String as sqlx::Encode<sqlx::Postgres>>::encode(self.to_string(), buf)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for #name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                Ok(s.parse()?)
            }
        }
        impl Into<sea_query::SimpleExpr> for #name{
            fn into(self) -> sea_query::SimpleExpr {
                self.to_string().into()
            }
        }

        impl Into<sea_query::SimpleExpr> for &#name{
            fn into(self) -> sea_query::SimpleExpr {
                self.to_string().into()
            }
        }

    };

    r#gen.into()
}

/// Macro to generate a localized version of a struct and query functions for translation
///
/// This macro generates:
/// 1. A `Localized{StructName}` struct where `TextContentId` fields are replaced with `String`
/// 2. A `{StructName}WithTranslations` struct where `TextContentId` fields are replaced with `TextContent` and includes a `translations` field
/// 3. A `query_to_localisation` function that modifies queries to join with translation tables
///
/// Usage:
/// ```rust,ignore
/// use comhairle_macros::Translatable;
/// use uuid::Uuid;
/// use comhairle::models::translations::TextContentId;
///
/// #[derive(Translatable)]
/// struct MyStruct {
///     id: Uuid,
///     title: TextContentId,
///     description: TextContentId,
///     other_field: String,
/// }
/// ```
///
/// This will generate `LocalizedMyStruct`, `MyStructWithTranslations` and associated functions.
#[proc_macro_derive(Translatable)]
pub fn derive_translatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let localised_struct_name =
        syn::Ident::new(&format!("Localized{}", struct_name), struct_name.span());
    let with_translations_struct_name = syn::Ident::new(
        &format!("{}WithTranslations", struct_name),
        struct_name.span(),
    );

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Translatable only supports structs with named fields"),
        },
        _ => panic!("Translatable only supports structs"),
    };

    let mut localised_fields = Vec::new();
    let mut text_content_fields = Vec::new();
    let mut optional_text_content_fields = Vec::new();
    let mut non_text_content_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Check if this field is a TextContentId
        if is_text_content_id_type(field_type) {
            text_content_fields.push(field_name);
            // Replace TextContentId with String for both structs
            localised_fields.push(quote! {
                pub #field_name: String
            });
        } else if is_optional_text_content_id_type(field_type) {
            optional_text_content_fields.push(field_name);
            // Replace TextContentId with Option<String> for both structs
            localised_fields.push(quote! {
                pub #field_name: Option<String>
            });
        } else {
            non_text_content_fields.push(field_name);
            // Keep other fields as-is in both structs
            localised_fields.push(quote! {
                pub #field_name: #field_type
            });
        }
    }

    // Generate the table identifier enum name by convention
    let table_iden_name = syn::Ident::new(&format!("{}Iden", struct_name), struct_name.span());

    // Create field capitalized identifiers for the table enum (following PascalCase convention)
    let text_content_field_caps: Vec<_> = text_content_fields
        .iter()
        .map(|field| {
            let field_str = field.to_string();
            // Convert snake_case to PascalCase
            let pascal_case = snake_case_to_pascal(field_str);
            syn::Ident::new(&pascal_case, field.span())
        })
        .collect();
    let optional_text_content_field_caps: Vec<_> = optional_text_content_fields
        .iter()
        .map(|field| {
            let field_str = field.to_string();
            // Convert snake_case to PascalCase
            let pascal_case = snake_case_to_pascal(field_str);
            syn::Ident::new(&pascal_case, field.span())
        })
        .collect();

    // Generate the custom translations struct name
    let translations_struct_name =
        syn::Ident::new(&format!("{}Translations", struct_name), struct_name.span());

    // Generate fields for the translations struct
    let translation_fields = text_content_fields.iter().map(|field| {
        quote! {
            pub #field: Translation
        }
    });
    let optional_translation_fields = optional_text_content_fields.iter().map(|field| {
        quote! {
            pub #field: Option<Translation>
        }
    });

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, sqlx::FromRow, Debug, PartialEq, Clone)]
        pub struct #localised_struct_name {
            #(#localised_fields,)*
        }

        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct Translation {
            pub text_content: crate::routes::translations::dto::TextContentDto,
            pub text_translations: Vec<crate::routes::translations::dto::TextTranslationDto>,
        }

        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct #translations_struct_name {
            #(#translation_fields,)*
            #(#optional_translation_fields,)*
        }

        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, PartialEq, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct #with_translations_struct_name {
            #(#localised_fields,)*
            pub translations: #translations_struct_name,
        }

        impl #localised_struct_name {
            /// Modifies a query to join with translation tables and return localized text
            /// This function takes a partial query and a locale, and returns a modified query
            /// that joins with the translation tables to fetch the localized text content.
            /// Falls back to the primary locale if the requested locale is not available.
            pub fn query_to_localisation(
                mut query: sea_query::SelectStatement,
                locale: &str,
            ) -> sea_query::SelectStatement {
                use sea_query::{Expr, JoinType, Alias, Func};
                use crate::models::translations::{TextContentIden, TextTranslationIden};

                #(
                    {
                        // Create unique aliases for each text content field
                        let tc_alias = Alias::new(&format!("tc_{}", stringify!(#text_content_fields)));
                        let tt_alias = Alias::new(&format!("tt_{}", stringify!(#text_content_fields)));
                        let tt_primary_alias = Alias::new(&format!("tt_primary_{}", stringify!(#text_content_fields)));

                        // Join with text_content table using alias
                        query = query
                            .join_as(
                                JoinType::LeftJoin,
                                TextContentIden::Table,
                                tc_alias.clone(),
                                Expr::col((#table_iden_name::Table, #table_iden_name::#text_content_field_caps))
                                    .equals((tc_alias.clone(), TextContentIden::Id))
                            )
                            // Join with text_translation table for the specific locale using alias
                            .join_as(
                                JoinType::LeftJoin,
                                TextTranslationIden::Table,
                                tt_alias.clone(),
                                Expr::col((tc_alias.clone(), TextContentIden::Id))
                                    .equals((tt_alias.clone(), TextTranslationIden::ContentId))
                                    .and(Expr::col((tt_alias.clone(), TextTranslationIden::Locale)).eq(locale))
                            )
                            // Join with text_translation table for the primary locale as fallback
                            .join_as(
                                JoinType::LeftJoin,
                                TextTranslationIden::Table,
                                tt_primary_alias.clone(),
                                Expr::col((tc_alias.clone(), TextContentIden::Id))
                                    .equals((tt_primary_alias.clone(), TextTranslationIden::ContentId))
                                    .and(
                                        Expr::col((tt_primary_alias.clone(), TextTranslationIden::Locale))
                                            .equals((tc_alias.clone(), TextContentIden::PrimaryLocale))
                                    )
                            )
                            .to_owned();

                        // Select the translated content with COALESCE fallback to primary locale
                        query = query.expr_as(
                            Func::coalesce([
                                Expr::col((tt_alias, TextTranslationIden::Content)).into(),
                                Expr::col((tt_primary_alias, TextTranslationIden::Content)).into(),
                            ]),
                            Alias::new(stringify!(#text_content_fields))
                        ).to_owned();
                    }
                )*

                #(
                    {
                        // Create unique aliases for each text content field
                        let tc_alias = Alias::new(&format!("tc_{}", stringify!(#optional_text_content_fields)));
                        let tt_alias = Alias::new(&format!("tt_{}", stringify!(#optional_text_content_fields)));
                        let tt_primary_alias = Alias::new(&format!("tt_primary_{}", stringify!(#optional_text_content_fields)));

                        // Join with text_content table using alias
                        query = query
                            .join_as(
                                JoinType::LeftJoin,
                                TextContentIden::Table,
                                tc_alias.clone(),
                                Expr::col((#table_iden_name::Table, #table_iden_name::#optional_text_content_field_caps))
                                    .equals((tc_alias.clone(), TextContentIden::Id))
                            )
                            // Join with text_translation table for the specific locale using alias
                            .join_as(
                                JoinType::LeftJoin,
                                TextTranslationIden::Table,
                                tt_alias.clone(),
                                Expr::col((tc_alias.clone(), TextContentIden::Id))
                                    .equals((tt_alias.clone(), TextTranslationIden::ContentId))
                                    .and(Expr::col((tt_alias.clone(), TextTranslationIden::Locale)).eq(locale))
                            )
                            // Join with text_translation table for the primary locale as fallback
                            .join_as(
                                JoinType::LeftJoin,
                                TextTranslationIden::Table,
                                tt_primary_alias.clone(),
                                Expr::col((tc_alias.clone(), TextContentIden::Id))
                                    .equals((tt_primary_alias.clone(), TextTranslationIden::ContentId))
                                    .and(
                                        Expr::col((tt_primary_alias.clone(), TextTranslationIden::Locale))
                                            .equals((tc_alias.clone(), TextContentIden::PrimaryLocale))
                                    )
                            )
                            .to_owned();

                        // Select the translated content with COALESCE fallback to primary locale
                        query = query.expr_as(
                            Func::coalesce([
                                Expr::col((tt_alias, TextTranslationIden::Content)).into(),
                                Expr::col((tt_primary_alias, TextTranslationIden::Content)).into(),
                            ]),
                            Alias::new(stringify!(#optional_text_content_fields))
                        ).to_owned();
                    }
                )*

                query
            }
        }

        impl #with_translations_struct_name {
            /// Creates a new instance from the original struct and loads all translations
            pub async fn from_original(
                db: &sqlx::PgPool,
                original: #struct_name,
                locale: &str,
            ) -> Result<Self, crate::error::ComhairleError> {
                use crate::models::translations::{get_text_content_by_id, get_text_translations_by_content_id, get_text_translation_by_content_and_locale};

                Ok(Self {
                    #(
                        #text_content_fields: {
                            // Get the translated text for this locale, fallback to primary locale if needed
                            match get_text_translation_by_content_and_locale(db, &original.#text_content_fields, locale).await {
                                Ok(translation) => translation.content,
                                Err(_) => {
                                    // Try to get the text content to access primary locale
                                    let text_content = get_text_content_by_id(db, &original.#text_content_fields).await?;
                                    match get_text_translation_by_content_and_locale(db, &original.#text_content_fields, &text_content.primary_locale).await {
                                        Ok(translation) => translation.content,
                                        Err(_) => String::new(), // Fallback to empty string if no translation found
                                    }
                                }
                            }
                        },
                    )*
                    #(
                        #optional_text_content_fields: {
                            if let Some(field) = &original.#optional_text_content_fields {
                                // Get the translated text for this locale, fallback to primary locale if needed
                                match get_text_translation_by_content_and_locale(db, &field, locale).await {
                                    Ok(translation) => Some(translation.content),
                                    Err(_) => {
                                        // Try to get the text content to access primary locale
                                        let text_content = get_text_content_by_id(db, &field).await?;
                                        match get_text_translation_by_content_and_locale(db, &field, &text_content.primary_locale).await {
                                            Ok(translation) => Some(translation.content),
                                            Err(_) => Some(String::new()), // Fallback to empty string if no translation found
                                        }
                                    }
                                }
                            } else {
                                None
                            }
                        },
                    )*
                    // Copy non-TextContentId fields as-is
                    #(
                        #non_text_content_fields: original.#non_text_content_fields,
                    )*
                    translations: #translations_struct_name {
                        #(
                            #text_content_fields: {
                                // Get the TextContent for this field
                                let text_content = get_text_content_by_id(db, &original.#text_content_fields).await?.into();

                                // Get all translations for this content
                                let text_translations =
                                    (get_text_translations_by_content_id(db, &original.#text_content_fields).await?).into_iter().map(Into::into).collect();

                                // Create Translation struct for this field
                                Translation {
                                    text_content,
                                    text_translations,
                                }
                            },
                        )*
                        #(
                            #optional_text_content_fields: {
                                if let Some(field) = &original.#optional_text_content_fields {
                                    // Get the TextContent for this field
                                    let text_content = get_text_content_by_id(db, &field).await?.into();

                                    // Get all translations for this content
                                    let text_translations =
                                        (get_text_translations_by_content_id(db, &field).await?).into_iter().map(Into::into).collect();

                                    // Create Translation struct for this field
                                    Some(Translation {
                                        text_content,
                                        text_translations,
                                    })
                                } else {
                                    None
                                }
                            },
                        )*
                    },
                })
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derives translation-resolution support for a type whose fields (or, for
/// enums, whose variants' fields) may reference translatable text via
/// [`TextContentId`].
///
/// Allows translation resolution for nested JSON structures with translatable
/// text fields at different levels of nesting.
///
/// For a type `Foo`, `#[derive(TranslatableJson)]` generates:
///
/// - A mirror type `LocalizedFoo`, identical to `Foo` except every
///   `#[translatable]` field's type is replaced by that type's
///   [`LocalizeTranslations::Localized`] associated type (e.g. a
///   `TextContentId` field becomes `String`; a `Vec<Question>` field becomes
///   `Vec<LocalizedQuestion>`). Fields without `#[translatable]` are copied
///   through unchanged.
/// - `impl CollectTextContentIds for Foo`, which walks every
///   `#[translatable]` field (recursing into nested translatable types,
///   `Option`, and `Vec`) and collects every [`TextContentId`] found.
/// - `impl LocalizeTranslations for Foo`, with `type Localized = LocalizedFoo`,
///   which consumes `self` and a `&HashMap<TextContentId, String>` and
///   produces a `LocalizedFoo` by resolving each `#[translatable]` field
///   through the map and passing other fields through unchanged.
/// - `impl ResolveWithTranslations for Foo`, with `type WithTranslations =
///   FooWithTranslations`, which consumes `self` and a
///   `&HashMap<TextContentId, TranslationDto>` and produces a
///   `FooWithTranslations` calling `resolve_with_translations` at each
///   `#[translatable]` leaf.
///
/// # Field attribute
///
/// Mark any field whose type is [`TextContentId`], or whose type itself
/// derives `TranslatableJson` (directly, or wrapped in `Option<_>`/`Vec<_>`),
/// with `#[translatable]`:
///
/// ```ignore
/// #[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone, TranslatableJson)]
/// pub struct Question {
///     pub id: Uuid,
///     #[translatable]
///     pub text: TextContentId,
///     #[translatable]
///     pub r#type: QuestionType,
/// }
/// ```
///
/// expands (informally) to a `LocalizedQuestion { id: Uuid, text: String,
/// r#type: LocalizedQuestionType }` plus the two trait impls described
/// above. Fields without the attribute (e.g. `id`) are assumed to contain no
/// translatable content and are copied through as-is into `LocalizedQuestion`.
///
/// # Requirements on `#[translatable]` field types
///
/// The type of a `#[translatable]` field must implement both
/// `CollectTextContentIds`, `LocalizeTranslations` and `ResolveWithTranslations`
/// (directly, or via the `Option`/`Vec` blanket impls). [`TextContentId`]
/// implements all three directly; any other type used in a `#[translatable]`
/// field should itself derive `TranslatableJson`. Marking a field
/// `#[translatable]` when its type doesn't satisfy this produces a trait-bound
/// compile error at the use site.
#[proc_macro_derive(TranslatableJson, attributes(translatable))]
pub fn derive_translatable_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let localized_name = format_ident!("Localized{}", name);
    let with_translations_name = format_ident!("{}WithTranslations", name);
    let serde_attributes = passthrough_serde_attrs(&input.attrs);
    let derives: Vec<syn::Path> = vec![
        syn::parse_quote!(Debug),
        syn::parse_quote!(Clone),
        syn::parse_quote!(serde::Serialize),
        syn::parse_quote!(serde::Deserialize),
        syn::parse_quote!(JsonSchema),
        syn::parse_quote!(PartialEq),
    ];

    let expanded = match &input.data {
        Data::Struct(data) => derive_translatable_json_struct(
            name,
            &localized_name,
            &with_translations_name,
            data,
            &derives,
            &serde_attributes,
        ),
        Data::Enum(data) => derive_translatable_enum_struct(
            name,
            &localized_name,
            &with_translations_name,
            data,
            &derives,
            &serde_attributes,
        ),
        _ => panic!("TranslatableJson can only be derived for structs and enums"),
    };

    expanded.into()
}
