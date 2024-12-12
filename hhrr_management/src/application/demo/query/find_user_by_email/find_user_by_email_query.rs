use cqrs_derive_macro::Query;

#[derive(Query)]
pub struct FindUserByEmailQuery {
    pub user_email: String
}
