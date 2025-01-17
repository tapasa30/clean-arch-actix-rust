use uuid::Uuid;
use cqrs_core_derive_macro::Query;

#[derive(Query)]
pub struct FindUserByIdQuery {
    pub user_id: Uuid
}
