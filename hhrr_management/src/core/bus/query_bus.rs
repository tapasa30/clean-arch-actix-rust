use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use cqrs_domain::query::Query;
use cqrs_domain::query_handler::QueryHandlerBase;
use cqrs_domain::query_response::QueryResponse;
use crate::application::demo::query::find_user_by_email::find_user_by_email_query_handler::FindUserByEmailQueryHandler;
use crate::application::demo::query::find_user_by_id::find_user_by_id_query_handler::FindUserByIdQueryHandler;
use crate::core::container::repository_container::RepositoryContainer;

pub struct QueryBus {
    query_handlers: Mutex<HashMap<String, Box<dyn QueryHandlerBase>>>
}

impl QueryBus {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        let find_user_by_id_query_handler = FindUserByIdQueryHandler::new(
            repository_container.demo_repository
        );
        
        let find_user_by_email_query_handler = FindUserByEmailQueryHandler {};

        let mut query_handlers: HashMap<String, Box<dyn QueryHandlerBase>> = HashMap::new();

        query_handlers.insert(
            find_user_by_id_query_handler.get_query_name().to_string(),
            Box::new(find_user_by_id_query_handler)
        );

        query_handlers.insert(
            find_user_by_email_query_handler.get_query_name().to_string(),
            Box::new(find_user_by_email_query_handler)
        );

        return QueryBus {
            query_handlers: Mutex::new(query_handlers)
        }
    }

    pub fn dispatch_query(&self, query: &dyn Query) -> Result<Box<dyn QueryResponse>, Box<dyn Error>> {
        let query_handlers = self.query_handlers.lock().unwrap();
        let query_handler = query_handlers.get(query.get_name());

        if !query_handler.is_some() {
            panic!("QueryHandler not found");
        }

        let response = query_handler.unwrap().handle_query(query);

        drop(query_handlers);

        return response;
    }
}