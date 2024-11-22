pub trait UserRepositoryTrait {
    fn find_user_by_id(&self, user_id: &str);
}