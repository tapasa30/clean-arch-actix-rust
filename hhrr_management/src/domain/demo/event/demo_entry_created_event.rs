use events_core_derive_macro::DomainEvent;

#[derive(DomainEvent)]
pub struct CoreUserCreatedEvent {
    pub data: String
}

impl CoreUserCreatedEvent {
    pub fn new() -> CoreUserCreatedEvent {
        CoreUserCreatedEvent { data: "".to_string() }
    }
}

