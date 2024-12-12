use uuid::Uuid;

// TODO - Create some trait for this: eg: DomainModelTrait
pub trait DemoModelTrait {
    fn to_domain_model(&self) -> DemoModel;
}

#[derive(Debug)]
pub struct DemoModel {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub is_published: bool,
}

impl DemoModel {
    pub fn new(title: String, body: String) -> DemoModel {
        DemoModel {
            id: Uuid::new_v4(),
            title,
            body,
            is_published: false,
        }
    }
}