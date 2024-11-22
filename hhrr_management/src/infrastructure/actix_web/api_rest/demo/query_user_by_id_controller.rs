use actix_web::{HttpResponse, Responder, web};
use crate::application::demo::query::find_user_by_id::find_user_by_id_query::FindUserByIdQuery;
use crate::application::demo::query::find_user_by_id::find_user_by_id_query_response::FindUserByIdQueryResponse;
use crate::core::bus::query_bus::QueryBus;

pub async fn query_user_by_id_action(request_body: String, query_bus: web::Data<QueryBus>) -> impl Responder {
    let find_user_by_id_query = &FindUserByIdQuery {
        user_id: String::from("1")
    };

    let find_user_by_id_response = query_bus.dispatch_query(find_user_by_id_query).unwrap();
    let find_user_by_id_response = find_user_by_id_response.as_any().downcast_ref::<FindUserByIdQueryResponse>();

    return HttpResponse::Ok().json(find_user_by_id_response.unwrap().get_user_name());
}