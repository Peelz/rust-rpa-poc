use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PubSubPushMessage {
    pub message: PubSubMessage,
    pub subscription: String,
}

#[derive(Debug, Deserialize)]
pub struct PubSubMessage {
    pub data: String,
    pub message_id: String,
    pub attributes: Option<HashMap<String, String>>,
}
