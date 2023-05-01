use whatsapp_cloud_api::WhatasppClient;

use crate::{
    chatgpt::ChatContext,
    webhook::{WebhookBody, WebhookValue},
    Message, Text,
};
use std::sync::{mpsc::Receiver, Arc, Mutex};

pub fn start_whatsapp(rx: Arc<Mutex<Receiver<WebhookBody>>>, client: WhatasppClient) {
    tokio::spawn(async move {
        loop {
            let body = rx.lock().unwrap().recv().unwrap();
            match &body.entry[0].changes[0].value {
                WebhookValue::Messages(value) => {
                    for message in value.messages.iter() {
                        let to = &message.from;
                        println!("Processing message from: {}", to);
                        let mut context =
                            ChatContext::load(to).unwrap_or_else(|_| ChatContext::new(to));
                        context.add_message(&message.text.body).unwrap();

                        let response = context.generate_message().await;

                        match response {
                            Ok(response) => {
                                for choice in response.choices {
                                    let message = choice.message.content;
                                    let text = Text::new(&message);
                                    let message = Message::from_text(to, text);
                                    client.send_message(&message).await.unwrap();
                                }
                            }
                            Err(_) => println!("error"),
                        }

                        context.save().unwrap();
                    }
                }
            }
        }
    });
}
