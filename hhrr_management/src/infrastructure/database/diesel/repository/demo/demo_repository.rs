use diesel::{PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use uuid::Uuid;
use crate::domain::demo::model::demo_model::{DemoModel};
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;

pub struct PostgreSqlDemoRepository {
    database_pool: Pool<ConnectionManager<PgConnection>>
}

impl DemoRepositoryTrait for PostgreSqlDemoRepository {
    fn find_something_by_id(&self, user_id: Uuid) -> Result<Option<DemoModel>, Error> {
        return Ok(Some(DemoModel {
            id: user_id,
            title: "".to_string(),
            body: "".to_string(),
            is_published: false,
        }));
    }
}

impl PostgreSqlDemoRepository {
    pub fn new (database_pool: Pool<ConnectionManager<PgConnection>>) -> Self
    {
        return Self {
            database_pool
        };
    }
}