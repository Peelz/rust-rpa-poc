use actix_web::{HttpResponse, Responder, Result, Scope, post, web};
use base64::{Engine, prelude::BASE64_STANDARD};
use protocol::{biz_priv::binding::BindingRequest, gcp::pubsub_push_message::PubSubPushMessage};

#[post("/binding")]
async fn binding_request_handle(body: web::Json<PubSubPushMessage>) -> Result<impl Responder> {
    let decode = match BASE64_STANDARD.decode(&body.message.data) {
        Ok(data) => data,
        Err(_) => return Ok(HttpResponse::Ok()),
    };

    let bindingreq: BindingRequest = serde_json::from_slice(&decode)?;

    Ok(HttpResponse::Ok())
}

pub fn register() -> Scope {
    web::scope("").service(binding_request_handle)
}
