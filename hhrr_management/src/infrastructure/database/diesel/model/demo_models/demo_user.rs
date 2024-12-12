use diesel::{Queryable, Selectable};
use uuid::Uuid;
use crate::domain::demo::model::demo_model::{DemoModel, DemoModelTrait};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::infrastructure::database::diesel::schema::schema::demo_models)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateDemoUser {
    id: Uuid,
    title: String,
    body: String,
    is_published: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::infrastructure::database::diesel::schema::schema::demo_models)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ViewDemoUser {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub is_published: bool,
}

impl DemoModelTrait for CreateDemoUser {
    fn to_domain_model(&self) -> DemoModel {
        return DemoModel {
            id: self.id.clone(),
            title: self.title.clone(),
            body: self.body.clone(),
            is_published: self.is_published.clone()
        }
    }
}

impl DemoModelTrait for ViewDemoUser {
    fn to_domain_model(&self) -> DemoModel {
        return DemoModel {
            id: self.id.clone(),
            title: self.title.clone(),
            body: self.body.clone(),
            is_published: self.is_published.clone()
        }
    }
}
