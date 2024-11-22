use cqrs_derive_macro::CommandHandler;
use cqrs_domain::command_handler::CommandHandler;
use crate::application::demo::command::delete_user::delete_user_command::DeleteUserCommand;

#[derive(CommandHandler)]
pub struct DeleteUserCommandHandler {}

impl CommandHandler<DeleteUserCommand> for DeleteUserCommandHandler {
    fn handle(&self, command: &DeleteUserCommand) {
        println!("Deleting user with ID: {}", command.get_user_id());
    }
}