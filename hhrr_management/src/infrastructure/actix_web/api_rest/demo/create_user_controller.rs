use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use crate::application::demo::command::create_user::create_user_command::CreateUserCommand;
use crate::core::bus::command_bus::CommandBus;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub user_name: String
}

pub async fn create_user_action(create_user_request: web::Json<CreateUserRequest>, command_bus: web::Data<Arc<CommandBus>>) -> impl Responder {

    let create_user_command = &CreateUserCommand {
        user_name: create_user_request.user_name.clone(),
        body_extra: "extra_body".to_string(),
    };

    command_bus.dispatch_command(create_user_command);

    return HttpResponse::Ok().json("");
}