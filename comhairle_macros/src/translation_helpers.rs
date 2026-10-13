use quote::quote;
use syn::{Attribute, Fields, GenericArgument, PathArguments, Type};

pub fn derive_translatable_json_struct(
    name: &syn::Ident,
    localized_name: &syn::Ident,
    with_translations_name: &syn::Ident,
    data: &syn::DataStruct,
    derives: &[syn::Path],
    serde_attrs: &Vec<Attribute>,
) -> proc_macro2::TokenStream {
    let fields = match &data.fields {
        Fields::Named(f) => &f.named,
        _ => panic!("TranslatableJson only supports structs with named fields"),
    };

    let mut localized_fields = Vec::new();
    let mut localize_inits = Vec::new();
    let mut collect_text_content_id_statements = Vec::new();
    let mut with_translations_fields = Vec::new();
    let mut with_translations_inits = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let visibility = &field.vis;

        if has_translatable_attr(&field.attrs) {
            localized_fields.push(quote! {
                #visibility #ident: <#ty as crate::models::translations::LocalizeTranslations>::Localized
            });
            localize_inits.push(quote! {
                #ident: self.#ident.localize(map)
            });
            with_translations_fields.push(quote! {
                #visibility #ident: <#ty as crate::models::translations::ResolveWithTranslations>::WithTranslations
            });
            with_translations_inits.push(quote! {
                #ident: self.#ident.resolve_with_translations(translations, locale)
            });

            collect_text_content_id_statements.push(quote! {
                self.#ident.collect_text_content_ids(out);
            });
        } else {
            localized_fields.push(quote! { #visibility #ident: #ty });
            localize_inits.push(quote! { #ident: self.#ident });
            with_translations_fields.push(quote! { #visibility #ident: #ty });
            with_translations_inits.push(quote! { #ident: self.#ident });
        }
    }

    quote! {
        #[derive(#(#derives),*)]
        #(#serde_attrs)*
        pub struct #localized_name {
            #(#localized_fields),*
        }

        #[derive(#(#derives),*)]
        #(#serde_attrs)*
        pub struct #with_translations_name {
            #(#with_translations_fields),*
        }

        impl crate::models::translations::CollectTextContentIds for #name {
            fn collect_text_content_ids(&self, out: &mut std::collections::HashSet<TextContentId>) {
                #(#collect_text_content_id_statements)*
            }
        }

        impl crate::models::translations::LocalizeTranslations for #name {
            type Localized = #localized_name;

            fn localize(self, map: &std::collections::HashMap<TextContentId, String>) -> Self::Localized {
                #localized_name {
                    #(#localize_inits),*
                }
            }
        }

        impl crate::models::translations::ResolveWithTranslations for #name {
            type WithTranslations = #with_translations_name;

            fn resolve_with_translations(
                self,
                translations: &std::collections::HashMap<TextContentId, crate::models::translations::TranslationDto>,
                locale: &str,
            ) -> Self::WithTranslations {
                #with_translations_name {
                    #(#with_translations_inits),*
                }
            }

        }
    }
}

pub fn derive_translatable_enum_struct(
    name: &syn::Ident,
    localized_name: &syn::Ident,
    with_translations_name: &syn::Ident,
    data: &syn::DataEnum,
    derives: &[syn::Path],
    serde_attrs: &Vec<Attribute>,
) -> proc_macro2::TokenStream {
    let mut localized_variants = Vec::new();
    let mut localize_arms = Vec::new();
    let mut with_translations_variants = Vec::new();
    let mut with_translations_arms = Vec::new();

    let mut collect_arms = Vec::new();

    for variant in &data.variants {
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                // Nothing to localize so push verbatim
                localized_variants.push(quote! { #variant_ident });
                // Nothing to localize, `Self::Text => LocalizedType::Text`
                localize_arms
                    .push(quote! { Self::#variant_ident => #localized_name::#variant_ident });
                with_translations_variants.push(quote! { #variant_ident });
                with_translations_arms.push(
                    quote! { Self::#variant_ident => #with_translations_name::#variant_ident },
                );

                collect_arms.push(quote! { Self::#variant_ident => {} });
            }
            Fields::Named(fields) => {
                let mut field_idents = Vec::new();
                let mut localized_fields = Vec::new();
                let mut localize_inits = Vec::new();
                let mut with_translations_fields = Vec::new();
                let mut with_translations_inits = Vec::new();

                let mut collect_body = Vec::new();

                for field in &fields.named {
                    let ident = field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    field_idents.push(quote! { #ident });

                    if has_translatable_attr(&field.attrs) {
                        localized_fields.push(quote! {
                            #ident: <#ty as crate::models::translations::LocalizeTranslations>::Localized
                        });
                        localize_inits.push(quote! { #ident: #ident.localize(map) });
                        with_translations_fields.push(quote! {
                            #ident: <#ty as crate::models::translations::ResolveWithTranslations>::WithTranslations
                        });
                        with_translations_inits.push(quote! { #ident: #ident.resolve_with_translations(translations, locale) });

                        collect_body.push(quote! {
                            #ident.collect_text_content_ids(out);
                        });
                    } else {
                        localized_fields.push(quote! { #ident: #ty });
                        localize_inits.push(quote! { #ident: #ident });
                        with_translations_fields.push(quote! { #ident: #ty });
                        with_translations_inits.push(quote! { #ident: #ident });
                    }
                }

                localized_variants.push(quote! { #variant_ident { #(#localized_fields),* }});
                localize_arms.push(quote! {
                    Self::#variant_ident { #(#field_idents),* } => #localized_name::#variant_ident { #(#localize_inits),* }
                });
                with_translations_variants
                    .push(quote! { #variant_ident { #(#with_translations_fields),* }});
                with_translations_arms.push(quote! {
                    Self::#variant_ident { #(#field_idents),* } => #with_translations_name::#variant_ident { #(#with_translations_inits),* }
                });

                collect_arms.push(quote! {
                    Self::#variant_ident { #(#field_idents),* } => { #(#collect_body)* }
                });
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    panic!(
                        "TranslatableJson only supports tuple variants with exactly one field \
             (newtype variants), e.g. `Foo(Bar)`"
                    );
                }

                let inner_ty = &fields.unnamed[0].ty;

                if has_translatable_attr(&variant.attrs) {
                    localized_variants.push(quote! {
                        #variant_ident(<#inner_ty as crate::models::translations::LocalizeTranslations>::Localized)
                    });
                    localize_arms.push(quote! {
                        Self::#variant_ident(inner) => #localized_name::#variant_ident(inner.localize(map))
                    });
                    with_translations_variants.push(quote! {
                        #variant_ident(<#inner_ty as crate::models::translations::ResolveWithTranslations>::WithTranslations)
                    });
                    with_translations_arms.push(quote! {
                        Self::#variant_ident(inner) => #with_translations_name::#variant_ident(inner.resolve_with_translations(translations, locale))
                    });

                    collect_arms.push(quote! {
                        Self::#variant_ident(inner) => { inner.collect_text_content_ids(out); }
                    });
                } else {
                    localized_variants.push(quote! { #variant_ident(#inner_ty) });
                    localize_arms.push(quote! {
                        Self::#variant_ident(inner) => #localized_name::#variant_ident(inner)
                    });
                    with_translations_variants.push(quote! { #variant_ident(#inner_ty) });
                    with_translations_arms.push(quote! {
                        Self::#variant_ident(inner) => #with_translations_name::#variant_ident(inner)
                    });

                    collect_arms.push(quote! { Self::#variant_ident(_inner ) => {} });
                }
            }
        }
    }

    quote! {
        #[derive(#(#derives),*)]
        #(#serde_attrs)*
        pub enum #localized_name {
            #(#localized_variants),*
        }

        #[derive(#(#derives),*)]
        #(#serde_attrs)*
        pub enum #with_translations_name {
            #(#with_translations_variants),*
        }

        impl crate::models::translations::CollectTextContentIds for #name {
            fn collect_text_content_ids(&self, out: &mut std::collections::HashSet<TextContentId>) {
                match self {
                    #(#collect_arms),*
                }
            }
        }

        impl crate::models::translations::LocalizeTranslations for #name {
            type Localized = #localized_name;

            fn localize(self, map: &std::collections::HashMap<TextContentId, String>) -> Self::Localized {
                match self {
                    #(#localize_arms),*
                }
            }
        }

        impl crate::models::translations::ResolveWithTranslations for #name {
            type WithTranslations = #with_translations_name;

            fn resolve_with_translations(
                self,
                translations: &std::collections::HashMap<TextContentId, crate::models::translations::TranslationDto>,
                locale: &str,
            ) -> Self::WithTranslations {
                match self {
                    #(#with_translations_arms),*
                }
            }
        }
    }
}

pub fn has_translatable_attr(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("translatable"))
}

pub fn passthrough_serde_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .cloned()
        .collect()
}

/// Helper function to check if a type is TextContentId
pub fn is_text_content_id_type(ty: &Type) -> bool {
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

/// Helper function to check if a type is <TextContentId>
pub fn is_optional_text_content_id_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident != "Option" {
                    return false;
                }

                if let PathArguments::AngleBracketed(args) = &segment.arguments
                    && let Some(GenericArgument::Type(inner_type)) = args.args.first()
                {
                    return is_text_content_id_type(inner_type);
                }
            }
            false
        }
        _ => false,
    }
}

pub fn snake_case_to_pascal(snake_word: String) -> String {
    snake_word
        .split('_')
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap();
            }
            chars.into_iter().collect::<String>()
        })
        .collect::<String>()
}
