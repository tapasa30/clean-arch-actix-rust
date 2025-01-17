use std::sync::Arc;
use cqrs_core_derive_macro::CommandHandler;
use cqrs_core::command_handler::CommandHandler;
use events_core::domain_event::DomainEvent;
use events_core::event::Event;
use events_core::event_bus::EventBus;
use crate::application::demo::command::create_user::create_user_command::CreateUserCommand;
use crate::core::bus::event_bus::DomainEventBus;
use crate::domain::demo::model::demo_model::DemoModel;
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;

#[derive(CommandHandler)]
pub struct CreateUserCommandHandler {
    demo_repository: Arc<Box<dyn DemoRepositoryTrait>>,
    domain_event_bus: Arc<DomainEventBus>
}

impl CreateUserCommandHandler {
    pub fn new(
        demo_repository: Arc<Box<dyn DemoRepositoryTrait>>,
        domain_event_bus: Arc<DomainEventBus>
    ) -> Self {
        return Self {
            demo_repository,
            domain_event_bus
        };
    }
}

impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    fn handle(&self, command: &CreateUserCommand) {
        let mut demo_model = DemoModel::new(
            command.user_name.to_string(),
            command.body_extra.to_string(),
        );

        let user_crated = self.demo_repository
            .create_something_by_create_demo_model(&demo_model);

        if !user_crated {
            println!("Error creating user: {}", command.user_name);
            
            return;
        }

        demo_model.domain_events.iter().for_each(|domain_event| {
            println!("Emitting event: {}", domain_event.get_name());
            // self.domain_event_bus.emit(); TODO - make emit_event wrapper
        });
    }
}
