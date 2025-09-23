use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

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
