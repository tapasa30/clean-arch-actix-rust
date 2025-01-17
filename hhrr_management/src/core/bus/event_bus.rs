use std::sync::Arc;
use events_core::event::Event;
use events_core::event_bus::EventBus;
use events_core::event_subscriber::EventSubscriberBase;
use crate::core::container::repository_container::RepositoryContainer;

pub struct DomainEventBus {
    event_subscribers: Vec<Box<dyn EventSubscriberBase>>
}

impl DomainEventBus {
    pub fn new(repository_container: Arc<RepositoryContainer>) -> Self {
        let mut event_subscribers: Vec<Box<dyn EventSubscriberBase>> = Vec::new();

        return Self {
            event_subscribers
        }
    }
}

impl EventBus for DomainEventBus {
    // TODO - see if this can be inside the EventBus trait or if it needs to be implemented in each struct
    fn emit(&self, event: &dyn Event) {
        println!("{}", event.get_name());

        todo!("Implement emit method - look for subscribers and emit event");

        // return self.emit_event(event_imp.unwrap());
    }
}

