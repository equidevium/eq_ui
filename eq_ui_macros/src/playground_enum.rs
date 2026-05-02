//! `#[derive(PlaygroundEnum)]` — generates `PlaygroundEnumInfo` impl.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Result};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(e) => &e.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "PlaygroundEnum can only be derived on enums",
            ));
        }
    };

    // Validate: all variants must be unit (no fields)
    for v in variants {
        if !matches!(v.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                v,
                "PlaygroundEnum only supports unit variants (no fields)",
            ));
        }
    }

    // Collect variant names
    let variant_names: Vec<String> = variants.iter().map(|v| v.ident.to_string()).collect();
    let variant_idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    // Find the #[default] variant
    let default_variant = variants
        .iter()
        .find(|v| v.attrs.iter().any(|a| a.path().is_ident("default")))
        .map(|v| v.ident.to_string())
        .unwrap_or_else(|| variant_names.first().cloned().unwrap_or_default());

    let variant_count = variant_names.len();

    // Build the match arms for from_name
    let match_arms = variant_names.iter().zip(variant_idents.iter()).map(|(name_str, ident)| {
        quote! { #name_str => #name::#ident }
    });

    // Default arm uses the default variant
    let default_ident = variants
        .iter()
        .find(|v| v.ident.to_string() == default_variant)
        .map(|v| &v.ident)
        .unwrap_or(&variant_idents[0]);

    Ok(quote! {
        impl crate::playground_enum_trait::PlaygroundEnumInfo for #name {
            fn variant_names() -> &'static [&'static str] {
                static NAMES: [&str; #variant_count] = [#(#variant_names),*];
                &NAMES
            }

            fn from_name(s: &str) -> Self {
                match s {
                    #(#match_arms,)*
                    _ => #name::#default_ident,
                }
            }

            fn default_name() -> &'static str {
                #default_variant
            }
        }
    })
}
