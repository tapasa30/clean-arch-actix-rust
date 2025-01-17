use crate::event::Event;

pub trait EventBus {
    fn emit(&self, event: &dyn Event);
}
