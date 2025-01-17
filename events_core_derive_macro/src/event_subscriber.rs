use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn implement_derive_event_subscriber_trait(derive_input: &DeriveInput) -> TokenStream {
    let event_subscriber_struct_name = &derive_input.ident;

    let expanded = quote! {
        impl events_core::event_subscriber::EventSubscriberBase for #event_subscriber_struct_name
        {
            fn handle_event(&self, event: &dyn events_core::event::Event) {
                return self.handle_event_as_any(event);
            }

            fn get_name(&self) -> &'static str {
                return std::any::type_name::<Self>();
            }

            fn get_event_name(&self) -> &'static str {
                return self.get_event_imp_name();
            }
        }
    };

    TokenStream::from(expanded)
}
