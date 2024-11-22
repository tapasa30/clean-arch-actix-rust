use cqrs_derive_macro::Command;

#[derive(Command)]
pub struct CreateUserCommand {
    pub user_name: String
}

impl CreateUserCommand {
    pub fn get_user_name(&self) -> &String
    {
        return &self.user_name;
    }
}