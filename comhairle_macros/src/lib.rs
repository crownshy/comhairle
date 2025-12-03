use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

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

    let gen = quote! {
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

    gen.into()
}

/// Macro to generate a localized version of a struct and query functions for translation
/// 
/// This macro generates:
/// 1. A `Localised{StructName}` struct where `TextContentId` fields are replaced with `String`
/// 2. A `query_to_localisation` function that modifies queries to join with translation tables
/// 
/// Usage:
/// ```rust
/// #[derive(Translatable)]
/// struct MyStruct {
///     id: Uuid,
///     title: TextContentId,
///     description: TextContentId,
///     other_field: String,
/// }
/// ```
/// 
/// This will generate `LocalisedMyStruct` and associated functions.
#[proc_macro_derive(Translatable)]
pub fn derive_translatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let struct_name = &input.ident;
    let localised_struct_name = syn::Ident::new(&format!("Localised{}", struct_name), struct_name.span());
    
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Translatable only supports structs with named fields"),
        },
        _ => panic!("Translatable only supports structs"),
    };

    let mut localised_fields = Vec::new();
    let mut text_content_fields = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        
        // Check if this field is a TextContentId
        if is_text_content_id_type(field_type) {
            text_content_fields.push(field_name);
            // Replace TextContentId with String
            localised_fields.push(quote! {
                pub #field_name: String
            });
        } else {
            // Keep other fields as-is
            localised_fields.push(quote! {
                pub #field_name: #field_type
            });
        }
    }
    
    // Generate the table identifier enum name by convention
    let table_iden_name = syn::Ident::new(&format!("{}Iden", struct_name), struct_name.span());
    
    // Create field capitalized identifiers for the table enum (following PascalCase convention)
    let text_content_field_caps: Vec<_> = text_content_fields.iter().map(|field| {
        let field_str = field.to_string();
        // Convert snake_case to PascalCase
        let pascal_case = field_str
            .split('_')
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                if !chars.is_empty() {
                    chars[0] = chars[0].to_uppercase().next().unwrap();
                }
                chars.into_iter().collect::<String>()
            })
            .collect::<String>();
        syn::Ident::new(&pascal_case, field.span())
    }).collect();

    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, sqlx::FromRow, Debug, PartialEq, Clone)]
        pub struct #localised_struct_name {
            #(#localised_fields,)*
        }
        
        impl #localised_struct_name {
            /// Modifies a query to join with translation tables and return localized text
            /// This function takes a partial query and a locale, and returns a modified query
            /// that joins with the translation tables to fetch the localized text content.
            pub fn query_to_localisation(
                mut query: sea_query::SelectStatement,
                locale: &str,
            ) -> sea_query::SelectStatement {
                use sea_query::{Expr, JoinType, Alias};
                use crate::models::translations::{TextContentIden, TextTranslationIden};
                
                #(
                    {
                        // Create unique aliases for each text content field
                        let tc_alias = Alias::new(&format!("tc_{}", stringify!(#text_content_fields)));
                        let tt_alias = Alias::new(&format!("tt_{}", stringify!(#text_content_fields)));
                        
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
                            .to_owned();
                        
                        // Select the translated content with the original field name as alias
                        query = query.expr_as(
                            Expr::col((tt_alias, TextTranslationIden::Content)),
                            Alias::new(stringify!(#text_content_fields))
                        ).to_owned();
                    }
                )*
                
                query
            }
        }
    };

    TokenStream::from(expanded)
}

/// Helper function to check if a type is TextContentId
fn is_text_content_id_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == "TextContentId"
            } else {
                false
            }
        }
        _ => false,
    }
}
