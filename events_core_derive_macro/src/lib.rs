use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use crate::domain_event::implement_derive_domain_event_trait;
use crate::event_subscriber::implement_derive_event_subscriber_trait;

mod domain_event;
mod event_subscriber;

#[proc_macro_derive(DomainEvent)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_domain_event_trait(&derive_input);
}

#[proc_macro_derive(EventSubscriber)]
pub fn derive_event_subscriber(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_event_subscriber_trait(&derive_input);
}
