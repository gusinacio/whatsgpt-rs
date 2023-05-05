use std::env;

use crate::webhook::{MessageSender, WebhookBody};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Hub {
    #[serde(rename = "hub.mode")]
    mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    token: Option<String>,
    #[serde(rename = "hub.challenge")]
    challenge: String,
}

pub async fn get_webhook(Query(hub): Query<Hub>) -> impl IntoResponse {
    // Your verify token. Should be a random string.
    let verify_token = env::var("WEBHOOK_VERIFY_TOKEN").unwrap();

    // Checks if a token and mode is in the query string of the request
    match (&hub.mode, &hub.token) {
        (Some(mode), Some(token)) => {
            // Checks the mode and token sent is correct
            if mode == "subscribe" && token == &verify_token {
                // Responds with the challenge token from the request
                println!("WEBHOOK_VERIFIED");
                (StatusCode::OK, hub.challenge.clone())
            } else {
                // Responds with '403 Forbidden' if verify tokens do not match
                (StatusCode::FORBIDDEN, "Forbidden".to_string())
            }
        }
        // Responds with '403 Forbidden' if verify tokens do not match
        _ => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
    }
}

pub async fn post_webhook(
    State(tx): State<MessageSender>,
    // body: String,
    Json(body): Json<WebhookBody>,
) -> impl IntoResponse {
    if body.object == "whatsapp_business_account" {
        println!("Received webhook request from whatspp");
        tx.lock().unwrap().send(body).unwrap();
        (StatusCode::OK, "EVENT_RECEIVED")
    } else {
        // Returns a '404 Not Found' if event is not from a page subscription
        (StatusCode::NOT_FOUND, "")
    }
}
