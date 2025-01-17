use uuid::Uuid;
use ddd_core_derive_macro::DomainModel;
use events_core::domain_event::DomainEvent;
use crate::domain::demo::event::demo_entry_created_event::CoreUserCreatedEvent;
use crate::domain::demo::event::demo_entry_published_event::CoreUserPublishedEvent;

#[derive(DomainModel)]
pub struct DemoModel {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub is_published: bool,
    pub domain_events: Vec<Box<dyn DomainEvent>>,
}

impl DemoModel {
    pub fn new(title: String, body: String) -> DemoModel {
        let mut domain_events = Vec::new();

        let created_user_event: Box<dyn DomainEvent> = Box::new(CoreUserCreatedEvent::new());

        domain_events.push(created_user_event);

        DemoModel {
            id: Uuid::new_v4(),
            title,
            body,
            is_published: false,
            domain_events,
        }
    }

    pub fn recreate(
        id: Uuid,
        title: String,
        body: String,
        is_published: bool,
    ) -> Self {
        Self {
            id,
            title,
            body,
            is_published,
            domain_events: Vec::new(),
        }
    }

    pub fn publish(&mut self) {
        self.is_published = true;
        
        let published_user_event: Box<dyn DomainEvent> = Box::new(CoreUserPublishedEvent::new());

        self.domain_events.push(published_user_event);
    }
}