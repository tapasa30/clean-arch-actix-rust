use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn implement_derive_query_trait(derive_input: &DeriveInput) -> TokenStream {
    let name = &derive_input.ident;

    let expanded = quote! {
        impl cqrs_domain::query::Query for #name {
            fn get_name(&self) -> &'static str {
                return std::any::type_name::<Self>();
            }

            fn as_any(&self) -> &dyn std::any::Any {
                return self
            }
        }
    };

    TokenStream::from(expanded)
}