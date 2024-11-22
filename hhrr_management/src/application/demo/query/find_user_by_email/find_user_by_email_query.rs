use cqrs_derive_macro::Query;

#[derive(Query)]
pub struct FindUserByEmailQuery {
    pub user_email: String
}

impl FindUserByEmailQuery {
    pub fn get_user_email(&self) -> &String
    {
        return &self.user_email;
    }
}