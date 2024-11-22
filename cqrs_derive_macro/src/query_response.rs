use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn implement_derive_query_response_trait(derive_input: &DeriveInput) -> TokenStream {
    let query_response_struct_name = &derive_input.ident;

    let expanded = quote! {
        impl cqrs_domain::query_response::QueryResponse for #query_response_struct_name
        {
            fn as_any(&self) -> &dyn std::any::Any {
                return self
            }

            fn get_response_name(&self) -> String {
                stringify!(#query_response_struct_name).to_string()
            }
        }
    };

    TokenStream::from(expanded)
}