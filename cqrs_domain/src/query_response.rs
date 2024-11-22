use std::any::Any;

pub trait QueryResponse {
    fn as_any(&self) -> &dyn Any;

    fn get_response_name(&self) -> String;
}