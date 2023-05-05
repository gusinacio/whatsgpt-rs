# whatsgpt-rs
Whatsapp bot that answers questions using OpenAI's GPT

This is an opensource version of [ZapGPT](https://www.zapgpt.com.br), which is a ChatGPT for WhatsApp written in Rust.
Instead of paying for ZapGPT, you can use your own OpenAI keys and just pay for what you use, without limitations.

It uses Rust so it's way more efficient if you use it on serverless architecture.


## Getting the keys

This application uses two main components: WhatsApp and GPT.

To get the keys for WhatsApp, you need to create a [Meta app](https://developers.facebook.com/docs/whatsapp/cloud-api/get-started#set-up-developer-assets). Here, you can get the WhatsApp phone and WhatsApp token. Also, don't forget to setup the test number and allow your account.

To get the keys for ChatGPT, you need to create an [API key](https://platform.openai.com/account/api-keys) in OpenAI's platform.

Finally, Facebook's api will send you messages through Webhooks and if you want it to work in localhost, the fastest way is using [ngrok](https://ngrok.com/). Get the keys there.


## Environment variables

```
# whatsapp tokens
WHATSAPP_TOKEN=<token from facebook developer>
WHATSAPP_PHONE=<phone from facebook developer>

# arbitrary token for webhook verification.
WEBHOOK_VERIFY_TOKEN=<create a webhook for whatsapp>

# ngrok token
NGROK_AUTHTOKEN=<token>

# openai token
OPENAI_API_KEY=<token from openai>

```

## How to run

Download the project, using Rust and run:

```shell
cargo run
```

After running the first time, you need to enable the webhook in [Facebook Developer](https://developers.facebook.com/docs/whatsapp/cloud-api/get-started#configure-webhooks).

Use `https://<ngrok hostname>/webhook` as the URL and the value in `WEBHOOK_VERIFY_TOKEN` as token verification.

If you received a message with `WEBHOOK_VERIFIED` in your console, you are ready to send messages to your own personal chatbot.


## Current limitations

Currently, it only uses local storage for context saving. It also doesn't have a way to clean context and get a fresh conversation.
