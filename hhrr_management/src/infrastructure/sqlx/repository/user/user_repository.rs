use crate::domain::core_user::repository::core_user_repository_trait::UserRepositoryTrait;

pub struct PostgreSqlUserRepository;

impl UserRepositoryTrait for PostgreSqlUserRepository {
    fn find_user_by_id(&self, user_id: &str) {
        todo!()
    }
}