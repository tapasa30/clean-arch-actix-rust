use std::error::Error;
use std::sync::Arc;
use cqrs_core_derive_macro::QueryHandler;
use cqrs_core::query_handler::QueryHandler;
use crate::application::demo::query::find_user_by_id::find_user_by_id_query::FindUserByIdQuery;
use crate::application::demo::query::find_user_by_id::find_user_by_id_query_response::FindUserByIdQueryResponse;
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;

#[derive(QueryHandler)]
pub struct FindUserByIdQueryHandler {
    demo_repository: Arc<Box<dyn DemoRepositoryTrait>>,
}

impl FindUserByIdQueryHandler {
    pub fn new(demo_repository: Arc<Box<dyn DemoRepositoryTrait>>) -> Self {
        return Self {
            demo_repository
        };
    }
}

impl QueryHandler<FindUserByIdQuery, FindUserByIdQueryResponse> for FindUserByIdQueryHandler {
    fn handle(&self, query: &FindUserByIdQuery) -> Result<Box<FindUserByIdQueryResponse>, Box<dyn Error>> {
        return match self.demo_repository.find_something_by_id(query.user_id) {
            Ok(Some(demo_model)) => Ok(
                Box::new(FindUserByIdQueryResponse { user_name: demo_model.title.to_string() })
            ),
            Ok(None) => panic!("TODO - none found"),
            Err(_) => panic!("TODO - Repository query error"),
        };
    }
}
