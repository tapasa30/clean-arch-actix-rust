use actix_web::{HttpResponse, Responder, web};
use crate::application::demo::command::create_user::create_user_command::CreateUserCommand;
use crate::core::bus::command_bus::CommandBus;

pub async fn create_user_action(request_body: String, command_bus: web::Data<CommandBus>) -> impl Responder {

    let create_user_command = &CreateUserCommand {
        user_name: "Pep".to_string()
    };

    command_bus.dispatch_command(create_user_command);

    return HttpResponse::Ok().json("");
}