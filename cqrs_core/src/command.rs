use std::any::Any;

pub trait Command: Any {
    fn get_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}