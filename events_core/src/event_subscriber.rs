use crate::event::Event;

pub trait EventSubscriber<EventImp: Event> {
    fn emit(&self, event: &EventImp);

    fn handle_event_as_any(&self, event: &dyn Event) {
        let event_imp = event.as_any().downcast_ref::<EventImp>();

        if !event_imp.is_some() {
            panic!("Event not implemented");
        }

        return self.emit(event_imp.unwrap());
    }

    fn get_event_imp_name(&self) -> &'static str {
        return std::any::type_name::<EventImp>();
    }
}

pub trait EventSubscriberBase: Send + Sync {
    fn handle_event(&self, event: &dyn Event);

    fn get_name(&self) -> &'static str;

    fn get_event_name(&self) -> &'static str;
}