use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;
use crate::infrastructure::sqlx::repository::demo::demo_repository::PostgreSqlDemoRepository;

pub struct RepositoryContainer {
    pub demo_repository: Arc<Box<dyn DemoRepositoryTrait>>
}

impl RepositoryContainer {
    pub fn new(database_pool: Pool<Postgres>) -> Self {
        let demo_repository = PostgreSqlDemoRepository::new(
            database_pool.clone()
        );

        return Self {
            demo_repository: Arc::new(Box::new(demo_repository))
        }
    }
}
