use dotenvy::dotenv;
use std::{
    env,
    net::SocketAddr,
    sync::{mpsc, Arc, Mutex},
};
use whatsapp_cloud_api::{
    models::{Message, Text},
    WhatasppClient,
};

use anyhow::Result;
use axum::{routing::get, Router, Server};
use ngrok::prelude::*;

use crate::{
    routes::{get_webhook, post_webhook},
    webhook::WebhookBody,
    whatsapp::start_whatsapp,
};

mod chatgpt;
mod routes;
mod webhook;
mod whatsapp;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    dotenv().ok();

    let (tx, rx) = mpsc::channel::<WebhookBody>();
    let rx = Arc::new(Mutex::new(rx));
    let tx = Arc::new(Mutex::new(tx));

    let token = env::var("WHATSAPP_TOKEN").unwrap();
    let phone = env::var("WHATSAPP_PHONE").unwrap();
    let whatsapp_client = WhatasppClient::new(&token, &phone);

    start_whatsapp(rx, whatsapp_client);

    let app = Router::new()
        .route("/webhook", get(get_webhook).post(post_webhook))
        .with_state(tx);

    // Listen on ngrok's global network (i.e. https://myapp.ngrok.dev)
    let listener = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?
        .http_endpoint()
        .listen()
        .await?;
    println!("Ingress URL: {:?}", listener.url());

    Server::builder(listener)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
    Ok(())
}
