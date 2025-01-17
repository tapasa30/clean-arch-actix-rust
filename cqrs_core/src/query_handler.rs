use std::error::Error;
use crate::query::Query;
use crate::query_response::QueryResponse;

pub trait QueryHandler<QueryImp: Query, QueryResponseImp: QueryResponse + 'static> {
    fn handle(&self, query: &QueryImp) -> Result<Box<QueryResponseImp>, Box<dyn Error>>;
    
    fn get_query_response_for_imp(&self, query: &dyn Query) -> Result<Box<dyn QueryResponse>, Box<dyn Error>> {
        let query_imp = query.as_any().downcast_ref::<QueryImp>();

        if !query_imp.is_some() {
            panic!("Query not implemented");
        }

        let query_result = self.handle(query_imp.unwrap())?;
        
        return Ok(query_result);
    }

    fn get_query_imp_name(&self) -> &'static str {
        return std::any::type_name::<QueryImp>();
    }
}

pub trait QueryHandlerBase: Send + Sync {
    fn handle_query(&self, query: &dyn Query) -> Result<Box<dyn QueryResponse>, Box<dyn Error>>;

    fn get_name(&self) -> &'static str;

    fn get_query_name(&self) -> &'static str;
}