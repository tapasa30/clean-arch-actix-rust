use actix_web::web;

mod query_user_by_id_controller;
mod query_user_by_email_controller;
mod create_user_controller;
mod delete_user_controller;

pub fn demo_service_config(service_config: &mut web::ServiceConfig) -> () {
    service_config
        .service(web::resource("/query-user-by-id").route(web::get().to(query_user_by_id_controller::query_user_by_id_action)))
        .service(web::resource("/query-user-by-email").route(web::get().to(query_user_by_email_controller::query_user_by_email_action)))
        .service(web::resource("/create-user").route(web::post().to(create_user_controller::create_user_action)))
        .service(web::resource("/delete-user").route(web::delete().to(delete_user_controller::delete_user_action)))
    ;
}
