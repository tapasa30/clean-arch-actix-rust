use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer, middleware};
use infrastructure::actix_web::api_rest::demo::demo_service_config;
use crate::core::bus::command_bus::CommandBus;
use crate::core::bus::query_bus::QueryBus;
use crate::core::container::repository_container::RepositoryContainer;
use crate::core::container::service_container::ServiceContainer;

mod core;
mod infrastructure;
mod application;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();


    let service_container_reference = Arc::new(ServiceContainer::new());
    let command_bus_reference = Arc::new(CommandBus::new());
    let query_bus_reference = Arc::new(QueryBus::new(RepositoryContainer::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/demo")
                    .configure(demo_service_config)
            )
            .app_data(web::Data::from(service_container_reference.clone()))
            .app_data(web::Data::from(command_bus_reference.clone()))
            .app_data(web::Data::from(query_bus_reference.clone()))
        
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
