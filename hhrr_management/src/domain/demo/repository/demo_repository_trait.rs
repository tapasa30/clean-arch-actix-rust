use diesel::result::Error;
use uuid::Uuid;
use crate::domain::demo::model::demo_model::DemoModel;

pub trait DemoRepositoryTrait: Send {
    fn find_something_by_id(&self, user_id: Uuid) -> Result<Option<DemoModel>, Error>;
    fn find_something_by_email(&self, user_email: &str) -> Result<Option<DemoModel>, Error>;
    // fn find_all_users(&self) -> Result<Option<Vec<DemoModel>>, Error>;
}