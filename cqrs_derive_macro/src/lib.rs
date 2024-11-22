use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use crate::command::implement_derive_command_trait;
use crate::command_handler::implement_derive_command_handler_trait;
use crate::query::implement_derive_query_trait;
use crate::query_handler::implement_derive_query_handler_trait;
use crate::query_response::implement_derive_query_response_trait;

mod command;
mod command_handler;
mod query;
mod query_handler;
mod query_response;

#[proc_macro_derive(Command)]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_command_trait(&derive_input);
}

#[proc_macro_derive(CommandHandler)]
pub fn derive_command_handler(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_command_handler_trait(&derive_input);
}

#[proc_macro_derive(Query)]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_query_trait(&derive_input);
}

#[proc_macro_derive(QueryHandler)]
pub fn derive_query_handler(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_query_handler_trait(&derive_input);
}

#[proc_macro_derive(QueryResponse)]
pub fn derive_query_response(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    return implement_derive_query_response_trait(&derive_input);
}