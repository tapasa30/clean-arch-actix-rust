use crate::domain::demo::model::demo_model::DemoModel;
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;

pub struct PostgreSqlDemoRepository;

impl DemoRepositoryTrait for PostgreSqlDemoRepository {
    fn find_something_by_id(&self, user_id: &str) -> DemoModel {
        return DemoModel::new(
            format!("User ID is: {}", user_id),
            "This is a demo test".to_string(),
        );
        todo!();
    }

    fn find_something_by_email(&self, user_id: &str) -> DemoModel {
        todo!()
    }
}