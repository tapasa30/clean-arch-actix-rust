use std::sync::{Arc, Mutex};
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;
use crate::infrastructure::database::diesel::repository::demo::demo_repository::PostgreSqlDemoRepository;

pub struct RepositoryContainer {
    pub demo_repository: Arc<Mutex<Box<dyn DemoRepositoryTrait>>>
}

impl RepositoryContainer {
    pub fn new() -> Self {
        let demo_repository = PostgreSqlDemoRepository {};

        return RepositoryContainer {
            demo_repository: Arc::new(Mutex::new(Box::new(demo_repository)))
        }
    }
}
