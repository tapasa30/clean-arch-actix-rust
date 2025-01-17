use std::any::Any;

pub trait Event: Any {
    fn get_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}