use actix_web::{Responder, HttpResponse};

pub async fn register_action(request_body: String) -> impl Responder {
    return HttpResponse::Ok().json(request_body);
}

