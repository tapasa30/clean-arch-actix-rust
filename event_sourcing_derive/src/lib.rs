use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Procedural macro for deriving the Event trait.
#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl Event for #name {
            fn event_type(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Procedural macro for deriving the EventHandler trait.
#[proc_macro_derive(EventHandler)]
pub fn derive_event_handler(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl EventHandler for #name {
            type EventType = Self;

            fn apply(&mut self, event: &Self::EventType) {
                println!("Applying event: {}", event.event_type());
                // Implement your event handling logic here
            }
        }
    };

    TokenStream::from(expanded)
}

/// Procedural macro for deriving the Aggregate trait.
#[proc_macro_derive(Aggregate)]
pub fn derive_aggregate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl Aggregate for #name {
            fn apply_event(&mut self, event: &dyn Event) {
                println!("Applying event to aggregate: {}", event.event_type());
                // Implement your aggregate logic here
            }
        }
    };

    TokenStream::from(expanded)
}
