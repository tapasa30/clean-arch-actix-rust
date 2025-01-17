use cqrs_core_derive_macro::Command;

#[derive(Command)]
pub struct CreateUserCommand {
    pub user_name: String,
    pub body_extra: String
}
