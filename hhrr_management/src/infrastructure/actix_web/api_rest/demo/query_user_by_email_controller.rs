use actix_web::{Responder, HttpResponse, web};
use crate::application::demo::query::find_user_by_email::find_user_by_email_query::FindUserByEmailQuery;
use crate::application::demo::query::find_user_by_email::find_user_by_email_query_response::FindUserByEmailQueryResponse;
use crate::core::bus::query_bus::QueryBus;

pub async fn query_user_by_email_action(request_body: String, query_bus: web::Data<QueryBus>) -> impl Responder {
    let find_user_by_email_query = &FindUserByEmailQuery {
        user_email: "user@mail.fake".to_string()
    };

    let find_user_by_email_response = query_bus.dispatch_query(find_user_by_email_query).unwrap();
    let find_user_by_email_response = find_user_by_email_response.as_any().downcast_ref::<FindUserByEmailQueryResponse>();
    
    return HttpResponse::Ok().json(find_user_by_email_response.unwrap().get_user_name());
}
