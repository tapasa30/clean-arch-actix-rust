use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn implement_derive_query_handler_trait(derive_input: &DeriveInput) -> TokenStream {
    let query_handler_struct_name = &derive_input.ident;

    let expanded = quote! {
        impl cqrs_domain::query_handler::QueryHandlerBase for #query_handler_struct_name
        {
            fn handle_query(&self, query_imp: &dyn cqrs_domain::query::Query) -> Result<Box<dyn cqrs_domain::query_response::QueryResponse>, Box<dyn std::error::Error>> {
                return self.get_query_response_for_imp(query_imp);
            }

            fn get_name(&self) -> &'static str {
                return std::any::type_name::<Self>();
            }

            fn get_query_name(&self) -> &'static str {
                return self.get_query_imp_name();
            }
        }
    };

    TokenStream::from(expanded)
}