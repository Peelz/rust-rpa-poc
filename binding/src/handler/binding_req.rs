use actix_web::{HttpResponse, Responder, Scope, post, web};
use protocol::gcp::pubsub_push_message::PubSubPushMessage;

#[post("/binding")]
async fn binding_request_handle(body: web::Json<PubSubPushMessage>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn register() -> Scope {
    web::scope("").service(binding_request_handle)
}
