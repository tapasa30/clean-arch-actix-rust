use std::collections::HashMap;
use std::sync::Mutex;
use cqrs_domain::command::Command;
use cqrs_domain::command_handler::CommandHandlerBase;
use crate::application::demo::command::create_user::create_user_command_handler::CreateUserCommandHandler;
use crate::application::demo::command::delete_user::delete_user_command_handler::DeleteUserCommandHandler;

pub struct CommandBus {
    command_handlers: Mutex<HashMap<String, Box<dyn CommandHandlerBase>>>
}

impl CommandBus {
    pub fn new() -> Self {
        let create_user_command_handler = CreateUserCommandHandler {};
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
            command_handlers: Mutex::new(command_handlers)
        }
    }

    pub fn dispatch_command(&self, command: &dyn Command) {
        let command_handlers = self.command_handlers.lock().unwrap();
        let command_handler = command_handlers.get(command.get_name());

        if !command_handler.is_some() {
            panic!("CommandHandler not found");
        }
        
        // TODO - retrieve handler returned events and send them to event_bus

        command_handler.unwrap().handle_command(command);
        
        drop(command_handlers);
    }
}