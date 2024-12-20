use std::sync::Arc;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;
use crate::infrastructure::database::diesel::repository::demo::demo_repository::PostgreSqlDemoRepository;

pub struct RepositoryContainer {
    pub demo_repository: Arc<Box<dyn DemoRepositoryTrait>>
}

impl RepositoryContainer {
    pub fn new(database_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        let demo_repository = PostgreSqlDemoRepository::new(
            database_pool.clone()
        );

        return Self {
            demo_repository: Arc::new(Box::new(demo_repository))
        }
    }
}
