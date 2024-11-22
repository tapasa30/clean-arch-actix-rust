use std::error::Error;
use cqrs_derive_macro::QueryHandler;
use cqrs_domain::query_handler::QueryHandler;
use crate::application::demo::query::find_user_by_email::find_user_by_email_query::FindUserByEmailQuery;
use crate::application::demo::query::find_user_by_email::find_user_by_email_query_response::FindUserByEmailQueryResponse;

#[derive(QueryHandler)]
pub struct FindUserByEmailQueryHandler {}

impl QueryHandler<FindUserByEmailQuery, FindUserByEmailQueryResponse> for FindUserByEmailQueryHandler {
    fn handle(&self, query: &FindUserByEmailQuery) -> Result<Box<FindUserByEmailQueryResponse>, Box<dyn Error>> {
        println!("Searching user by email: {}", query.get_user_email());

        Ok(Box::new(FindUserByEmailQueryResponse { user_name: "asd".to_string() }))
    }
}