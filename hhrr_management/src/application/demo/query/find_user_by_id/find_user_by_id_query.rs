use cqrs_derive_macro::Query;

#[derive(Query)]
pub struct FindUserByIdQuery {
    pub user_id: String
}

impl FindUserByIdQuery {
    pub fn get_user_id(&self) -> &str
    {
        return self.user_id.as_str();
    }
}