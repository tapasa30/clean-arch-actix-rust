use std::collections::HashMap;
use std::sync::Arc;
use cqrs_domain::command::Command;
use cqrs_domain::command_handler::CommandHandlerBase;
use crate::application::demo::command::create_user::create_user_command_handler::CreateUserCommandHandler;
use crate::application::demo::command::delete_user::delete_user_command_handler::DeleteUserCommandHandler;
use crate::core::container::repository_container::RepositoryContainer;

pub struct CommandBus {
    command_handlers: HashMap<String, Box<dyn CommandHandlerBase>>
}

impl CommandBus {
    pub fn new(repository_container: Arc<RepositoryContainer>) -> Self {
        let create_user_command_handler = CreateUserCommandHandler::new(
            repository_container.demo_repository.clone()
        );

        let delete_user_command_handler = DeleteUserCommandHandler {};

        let mut command_handlers: HashMap<String, Box<dyn CommandHandlerBase>> = HashMap::new();

        command_handlers.insert(
            create_user_command_handler.get_command_name().to_string(),
            Box::new(create_user_command_handler)
        );

        command_handlers.insert(
            delete_user_command_handler.get_command_name().to_string(),
            Box::new(delete_user_command_handler)
        );

        return CommandBus {
            command_handlers
        }
    }

    pub fn dispatch_command(&self, command: &dyn Command) {
        let command_handler = self.command_handlers.get(command.get_name());

        if !command_handler.is_some() {
            panic!("CommandHandler not found");
        }
        
        // TODO - retrieve handler returned events and send them to event_bus

        command_handler.unwrap().handle_command(command);
    }
}