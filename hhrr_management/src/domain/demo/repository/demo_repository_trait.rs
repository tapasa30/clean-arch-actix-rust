use uuid::Uuid;
use crate::domain::demo::model::demo_model::DemoModel;

// TODO - derive for repositories

pub trait DemoRepositoryTrait: Send + Sync {
    fn find_something_by_id(&self, user_id: Uuid) -> Result<Option<DemoModel>, sqlx::Error>;
}