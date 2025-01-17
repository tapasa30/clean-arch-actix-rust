use events_core_derive_macro::DomainEvent;

#[derive(DomainEvent)]
pub struct CoreUserPublishedEvent {
    pub data: String
}

impl CoreUserPublishedEvent {
    pub fn new() -> CoreUserPublishedEvent {
        CoreUserPublishedEvent { data: "".to_string() }
    }
}
