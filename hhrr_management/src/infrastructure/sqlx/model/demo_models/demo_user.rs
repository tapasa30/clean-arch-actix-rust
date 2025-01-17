use uuid::Uuid;
use crate::domain::demo::model::demo_model::{DemoModel};

pub struct CreateDemoUser {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub is_published: bool,
}

pub struct ViewDemoUser {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub is_published: bool,
}

impl CreateDemoUser {
    fn to_domain_model(&self) -> DemoModel {
        return DemoModel::recreate(
            self.id.clone(),
            self.title.clone(),
            self.body.clone(),
            self.is_published.clone()
        );
    }
}

impl ViewDemoUser {
    fn to_domain_model(&self) -> DemoModel {
        return DemoModel::recreate(
            self.id.clone(),
            self.title.clone(),
            self.body.clone(),
            self.is_published.clone()
        );
    }
}
