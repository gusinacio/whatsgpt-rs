use std::sync::{mpsc::Sender, Arc, Mutex};

use serde::{Deserialize, Serialize};

pub type MessageSender = Arc<Mutex<Sender<WebhookBody>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookBody {
    pub object: String,
    pub entry: Vec<WebhookEntry>,
    id: Option<String>,
    changed_fields: Option<Vec<String>>,
    // changes: Vec<WebhookChanges>,
    time: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEntry {
    id: String,
    pub changes: Vec<WebhookChange>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookChange {
    field: WebhookField,
    pub value: WebhookValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum WebhookField {
    Messages,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookValue {
    Messages(WebhookWhatsappMessage),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookWhatsappMessage {
    messaging_product: String,
    metadata: WebhookMetadata,
    contacts: Vec<WebhookContact>,
    pub messages: Vec<WebhookMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookMessage {
    pub from: String,
    id: String,
    timestamp: String,
    pub text: WebhookText,

    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookText {
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WebhookContact {
    profile: WebhookProfile,
    wa_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WebhookProfile {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WebhookMetadata {
    display_phone_number: String,
    phone_number_id: String,
}
