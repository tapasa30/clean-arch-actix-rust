use std::env;
use std::sync::Arc;
use actix_web::{web, App, HttpServer, middleware};
use diesel::{PgConnection, r2d2};
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
    dotenvy::dotenv().ok();
    env_logger::init();

    let connection_manager = r2d2::ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").unwrap());
    let database_pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("database URL should be valid path to SQLite DB file");

    let repository_container_reference = Arc::new(RepositoryContainer::new(database_pool.clone()));
    let service_container_reference = Arc::new(ServiceContainer::new());

    let command_bus_reference = Arc::new(CommandBus::new(repository_container_reference.clone()));
    let query_bus_reference = Arc::new(QueryBus::new(repository_container_reference.clone()));

    HttpServer::new(move || {

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(service_container_reference.clone()))
            .app_data(web::Data::new(command_bus_reference.clone()))
            .app_data(web::Data::new(query_bus_reference.clone()))
            .service(web::scope("/demo").configure(demo_service_config))
        
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
