use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use uuid::Uuid;
use crate::domain::demo::model::demo_model::{DemoModel, DemoModelTrait};
use crate::domain::demo::repository::demo_repository_trait::DemoRepositoryTrait;
use crate::infrastructure::database::diesel::model::demo_models::demo_user::{ViewDemoUser};
use crate::infrastructure::database::diesel::schema::schema::demo_models;

pub struct PostgreSqlDemoRepository {
    database_pool: Pool<ConnectionManager<PgConnection>>
}

impl DemoRepositoryTrait for PostgreSqlDemoRepository {
    fn find_something_by_id(&self, user_id: Uuid) -> Result<Option<DemoModel>, Error> {
        let mut db_connection = self.database_pool.get().unwrap();

        let found_demo_user = demo_models::table
            .find(user_id)
            .limit(1)
            .select(ViewDemoUser::as_select())
            .first(&mut db_connection)
            .expect("Error loading demol");
        
        return Ok(Some(found_demo_user.to_domain_model()));
    }

    fn find_something_by_email(&self, user_email: &str) -> Result<Option<DemoModel>, Error> {
        todo!()
    }

    // fn find_all_users(&self) -> Result<Option<Vec<DemoModel>>, Error> {
    //     let mut db_connection = self.database_pool.get().unwrap();
    // 
    //     let results = demo_models::table
    //         .select(ViewDemoUser::as_select())
    //         .load(&mut db_connection)
    //         .expect("Error loading users");
    // 
    //     println!("Displaying {} posts", results.len());
    //     for demo_model in results {
    //         println!("{}", demo_model.title);
    //         println!("-----------\n");
    //         println!("{}", demo_model.body);
    //     }
    // 
    //     return Ok(Some([vec![""]]));
    // 
    //     // return Ok(results.iter()
    //     //     .map(|create_demo_ser| create_demo_ser.to_domain_model())
    //     //     .collect()
    //     // );
    // }
}

impl PostgreSqlDemoRepository {
    pub fn new (database_pool: Pool<ConnectionManager<PgConnection>>) -> Self
    {
        return Self {
            database_pool
        };
    }
}