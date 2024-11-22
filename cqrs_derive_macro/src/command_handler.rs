use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

pub fn implement_derive_command_handler_trait(derive_input: &DeriveInput) -> TokenStream {
    let command_handler_struct_name = &derive_input.ident;

    let expanded = quote! {
        impl cqrs_domain::command_handler::CommandHandlerBase for #command_handler_struct_name
        {
            fn handle_command(&self, command: &dyn cqrs_domain::command::Command) {
                return self.handle_command_as_any(command);
            }

            fn get_name(&self) -> &'static str {
                return std::any::type_name::<Self>();
            }

            fn get_command_name(&self) -> &'static str {
                return self.get_command_imp_name();
            }
        }
    };

    TokenStream::from(expanded)
}
