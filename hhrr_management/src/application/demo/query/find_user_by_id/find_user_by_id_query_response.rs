use cqrs_core_derive_macro::QueryResponse;

#[derive(QueryResponse)]
pub struct FindUserByIdQueryResponse {
    pub user_name: String
}

impl FindUserByIdQueryResponse {
    pub fn get_user_name(&self) -> &String {
        return &self.user_name;
    }
}