use actix_web::web;

mod login_controller;
mod register_controller;

pub fn authentication_service_config(service_config: &mut web::ServiceConfig) -> () {
    service_config
        .service(web::resource("/login").route(web::post().to(login_controller::login_action)))
        .service(web::resource("/register").route(web::post().to(register_controller::register_action)));
}
