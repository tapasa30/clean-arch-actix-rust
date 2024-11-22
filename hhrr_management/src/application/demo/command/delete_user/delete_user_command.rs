use cqrs_derive_macro::Command;

#[derive(Command)]
pub struct DeleteUserCommand {
    pub user_id: String
}

impl DeleteUserCommand {
    pub fn get_user_id(&self) -> &String
    {
        return &self.user_id;
    }
}