use cqrs_core_derive_macro::QueryResponse;

#[derive(QueryResponse)]
pub struct FindUserByEmailQueryResponse {
    pub user_name: String
}

impl FindUserByEmailQueryResponse {
    pub fn get_user_name(&self) -> &String {
        return &self.user_name;
    }
}