use cqrs_derive_macro::CommandHandler;
use cqrs_domain::command_handler::CommandHandler;
use crate::application::demo::command::create_user::create_user_command::CreateUserCommand;

#[derive(CommandHandler)]
pub struct CreateUserCommandHandler {}

impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    fn handle(&self, command: &CreateUserCommand) {
        println!("Creating user: {}", command.get_user_name());
    }
}
