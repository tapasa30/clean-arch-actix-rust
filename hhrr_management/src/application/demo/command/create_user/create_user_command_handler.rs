use std::sync::Arc;
use cqrs_derive_macro::CommandHandler;
use cqrs_domain::command_handler::CommandHandler;
use crate::application::demo::command::create_user::create_user_command::CreateUserCommand;
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;
use crate::infrastructure::sqlx::model::demo_models::demo_user::CreateDemoUser;

#[derive(CommandHandler)]
pub struct CreateUserCommandHandler {
    demo_repository: Arc<Box<dyn DemoRepositoryTrait>>,
}

impl CreateUserCommandHandler {
    pub fn new(demo_repository: Arc<Box<dyn DemoRepositoryTrait>>) -> Self {
        return Self {
            demo_repository
        };
    }
}

impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    fn handle(&self, command: &CreateUserCommand) {
        let user_crated = self.demo_repository
            .create_something_by_create_demo_model(CreateDemoUser {
                id: uuid::Uuid::new_v4(),
                title: command.user_name.to_string(),
                body: "body".to_string(),
                is_published: true
            });

        if !user_crated {
            println!("Error creating user: {}", command.user_name);
            
            return;
        }
        
        println!("User created: {}", command.user_name);
    }
}
