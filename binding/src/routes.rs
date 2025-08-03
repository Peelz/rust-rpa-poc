use std::sync::Arc;

use actix_web::{
    HttpResponse, ResponseError, Scope, http::StatusCode, post, web,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use common::protocol::{
    biz_priv::binding::binding_request::BindingRequestEvent,
    gcp::pubsub_push_message::PubSubPushMessage,
};
use derive_more::{Display, Error};
use log::error;

use crate::{
    error::BindingError,
    service::AddPolicyServiceImp,
};

#[derive(Debug, Error, Display)]
enum ErrorHandle {
    #[display("retry error")]
    RetryAbleError,
    #[display("non retry error")]
    NonRetryAbleError,
}

impl ResponseError for ErrorHandle {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ErrorHandle::RetryAbleError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorHandle::NonRetryAbleError => StatusCode::OK,
        }
    }
}

#[post("/binding")]
async fn binding_request_handle(
    body: web::Json<PubSubPushMessage>,
    logic: web::Data<Arc<AddPolicyServiceImp>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let decode = match BASE64_STANDARD.decode(&body.message.data) {
        Ok(data) => data,
        Err(_) => {
            error!("Decode failed");
            return Ok(HttpResponse::Ok());
        }
    };

    let req: BindingRequestEvent = serde_json::from_slice(&decode)?;

    match logic.add_policy(req).await {
        Ok(()) => Ok(HttpResponse::Ok()),
        Err(err) => {
            if let Some(e) = err.downcast_ref::<BindingError>() {
                error!("Biz error: {err}"); // Debug format
                Ok(HttpResponse::Ok())
            } else {
                error!("On Error: {err}"); // Debug format
                Ok(HttpResponse::InternalServerError())
            }
        }
    }
}

pub fn register() -> Scope {
    web::scope("v1").service(binding_request_handle)
}
