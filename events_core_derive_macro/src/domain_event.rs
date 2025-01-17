use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn implement_derive_domain_event_trait(derive_input: &DeriveInput) -> TokenStream {
    let event_derive = generate_derive_event_trait(&derive_input);
    let domain_event_derive = generate_derive_domain_event_trait(&derive_input);

    TokenStream::from(quote! {
        #event_derive
        #domain_event_derive
    })
}

fn generate_derive_event_trait(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    quote! {
        impl events_core::event::Event for #name {
            fn get_name(&self) -> &'static str {
                return std::any::type_name::<Self>();
            }

            fn as_any(&self) -> &dyn std::any::Any {
                return self
            }
        }
    }
}

fn generate_derive_domain_event_trait(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    quote! {
        impl events_core::domain_event::DomainEvent for #name {}
    }
}
