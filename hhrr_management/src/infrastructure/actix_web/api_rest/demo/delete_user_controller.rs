use actix_web::{HttpResponse, Responder, web};
use crate::application::demo::command::delete_user::delete_user_command::DeleteUserCommand;
use crate::core::bus::command_bus::CommandBus;

pub async fn delete_user_action(request_body: String, command_bus: web::Data<CommandBus>) -> impl Responder {

    let delete_user_command = &DeleteUserCommand {
        user_id: "Pop".to_string()
    };

    command_bus.dispatch_command(delete_user_command);

    return HttpResponse::Ok().json("");
}