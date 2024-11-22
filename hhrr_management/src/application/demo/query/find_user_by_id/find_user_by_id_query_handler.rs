use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard};
use cqrs_derive_macro::QueryHandler;
use cqrs_domain::query_handler::QueryHandler;
use crate::application::demo::query::find_user_by_id::find_user_by_id_query::FindUserByIdQuery;
use crate::application::demo::query::find_user_by_id::find_user_by_id_query_response::FindUserByIdQueryResponse;
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;

#[derive(QueryHandler)]
pub struct FindUserByIdQueryHandler {
    demo_repository: Arc<Mutex<Box<dyn DemoRepositoryTrait>>>
}

impl FindUserByIdQueryHandler {
    pub fn new (demo_repository: Arc<Mutex<Box<dyn DemoRepositoryTrait>>>) -> Self {
        return Self {
            demo_repository
        };
    }
}

impl QueryHandler<FindUserByIdQuery, FindUserByIdQueryResponse> for FindUserByIdQueryHandler {
    fn handle(&self, query: &FindUserByIdQuery) -> Result<Box<FindUserByIdQueryResponse>, Box<dyn Error>> {
        let demo_repository_guard = self.demo_repository.lock().unwrap();

        let demo_model = demo_repository_guard.find_something_by_id(query.get_user_id());

        drop(demo_repository_guard);

        return Ok(Box::new(FindUserByIdQueryResponse { user_name: demo_model.get_title().to_string() }));
    }
}
