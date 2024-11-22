use crate::domain::demo::model::demo_model::DemoModel;

pub trait DemoRepositoryTrait: Send {
    fn find_something_by_id(&self, user_id: &str) -> DemoModel;
    fn find_something_by_email(&self, user_id: &str) -> DemoModel;
}