use cqrs_derive_macro::Command;

#[derive(Command)]
pub struct CreateUserCommand {
    pub user_name: String
}
