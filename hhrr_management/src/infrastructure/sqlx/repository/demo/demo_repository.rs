use uuid::Uuid;
use crate::domain::demo::model::demo_model::{DemoModel};
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;
use crate::infrastructure::sqlx::model::demo_models::demo_user::{CreateDemoUser, ViewDemoUser};
use futures::executor::block_on;
use sqlx::postgres::PgQueryResult;

pub struct PostgreSqlDemoRepository {
    database_pool: sqlx::PgPool
}

impl DemoRepositoryTrait for PostgreSqlDemoRepository {
    fn find_something_by_id(&self, user_id: Uuid) -> Result<Option<DemoModel>, sqlx::Error> {
        let result: Result<ViewDemoUser, sqlx::Error> = block_on(
            sqlx::query_as!(
                ViewDemoUser,
                "SELECT * FROM demo_models WHERE id = $1",
                user_id
            ).fetch_one(&self.database_pool)
        );

        if result.is_err() {
            return Ok(None);
        }

        let view_demo_user = result.unwrap();

        return Ok(Some(DemoModel {
            id: view_demo_user.id,
            title: view_demo_user.title,
            body: view_demo_user.body,
            is_published: view_demo_user.is_published,
        }));
    }

    fn create_something_by_create_demo_model(&self, create_demo_user: CreateDemoUser) -> bool {
        let result: Result<PgQueryResult, sqlx::Error> = block_on(
            sqlx::query_as!(
                ViewDemoUser,
                "INSERT INTO demo_models (id, title, body, is_published) VALUES ($1, $2, $3, $4)",
                create_demo_user.id,
                create_demo_user.title,
                create_demo_user.body,
                create_demo_user.is_published
            ).execute(&self.database_pool)
        );

        match result {
            Err(error) => {
                println!("Error inserting demo user: {}", create_demo_user.title);
                println!("Error message: [{}].\n", error.to_string());

                return false;
            }

            Ok(res) => {
                println!("Employee has been inserted.");
                println!("Number of employees inserted: {}", res.rows_affected());
            }
        }

        return true;
    }
}

impl PostgreSqlDemoRepository {
    pub fn new (database_pool: sqlx::PgPool) -> Self
    {
        return Self {
            database_pool
        };
    }
}