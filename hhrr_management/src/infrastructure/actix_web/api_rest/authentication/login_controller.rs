use actix_web::{HttpResponse, Responder, web};
use crate::application::authentication::services::authenticate_user_service::AuthenticateUserService;
use crate::core::container::service_container::ServiceContainer;

pub async fn login_action(request_body: String, service_container: web::Data<ServiceContainer>) -> impl Responder {
    let authenticate_user_service: &AuthenticateUserService = &service_container.authenticate_user_service;

    print!("{}", request_body.to_string());

    return HttpResponse::Ok().json(authenticate_user_service.authenticate_user());
}