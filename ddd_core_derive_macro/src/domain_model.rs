use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn implement_derive_domain_model_trait(derive_input: &DeriveInput) -> TokenStream {
    let name = &derive_input.ident;

    let expanded = quote! {
        impl ddd_core::domain_model::DomainModel for #name {}
    };

    TokenStream::from(expanded)
}