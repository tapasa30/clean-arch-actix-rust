use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use crate::domain_model::implement_derive_domain_model_trait;

mod domain_model;

#[proc_macro_derive(DomainModel)]
pub fn derive_domain_model(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_domain_model_trait(&derive_input);
}
