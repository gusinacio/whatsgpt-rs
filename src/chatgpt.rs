use std::fs;

use anyhow::Result;
use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse, Role,
    },
    Client,
};

const MODEL: &str = "gpt-3.5-turbo";
const MAX_TOKENS: u16 = 512;

#[derive(Default)]
pub struct ChatContext {
    id: String,
    messages: Vec<ChatCompletionRequestMessage>,
}

impl ChatContext {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Self::default()
        }
    }

    pub fn load(id: &str) -> Result<Self> {
        let path = format!("data/{}.json", id);
        let content = fs::read_to_string(path)?;
        let messages: Vec<ChatCompletionRequestMessage> = serde_json::from_str(&content)?;

        Ok(Self {
            id: id.to_string(),
            messages,
        })
    }

    pub fn save(&self) -> Result<()> {
        let path = format!("data/{}.json", self.id);
        let content = serde_json::to_string(&self.messages)?;
        fs::write(path, content)?;

        Ok(())
    }

    pub fn add_message(&mut self, message: &str) -> Result<()> {
        let message = ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(message)
            .build()?;
        self.messages.push(message);
        Ok(())
    }

    pub async fn generate_message(&mut self) -> Result<CreateChatCompletionResponse> {
        let client = Client::new();
        let mut messages = vec![ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content(
                "You are WhatsGPT-rs, a helpful assistent that works with WhatsApp messages. \
                    You are written in Rust and answers their questions using Webhooks.",
            )
            .build()?];

        messages.extend(self.messages.clone());
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(MAX_TOKENS)
            .model(MODEL)
            .messages(messages)
            .build()?;
        let response = client.chat().create(request).await?;
        for r in &response.choices {
            let message = ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content(&r.message.content)
                .build()?;
            self.messages.push(message);
        }
        Ok(response)
    }
}
