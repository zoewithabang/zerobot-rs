mod commands;
mod cytube;
mod tasks;

use dotenv;
use lazy_static::lazy_static;
use log;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

lazy_static! {
    static ref BOT_PREFIX: String = env::var("BOT_PREFIX").expect("BOT_PREFIX missing!");
    static ref CYTUBE_LOG: String = env::var("CYTUBE_LOG").expect("CYTUBE_LOG missing!");
    static ref CYTUBE_URL: String = env::var("CYTUBE_URL").expect("CYTUBE_URL missing!");
    static ref DISCORD_TOKEN: String = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing!");
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if !message.content.starts_with(&BOT_PREFIX.as_str()) {
            return;
        }

        let command = message.content.strip_prefix(&BOT_PREFIX.as_str()).unwrap();

        if let Err(e) = match command {
            "ping" => commands::ping(context, message).await,
            _ => Ok(()),
        } {
            log::error!("Error sending message: {:?}", e);
        };
    }

    async fn ready(&self, context: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        if let Err(e) = tasks::cytube_now_playing_presence(&context, &CYTUBE_LOG).await {
            log::error!("CyTube now playing presence error: {:?}", e);
        };
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = Client::builder(&DISCORD_TOKEN.as_str())
        .event_handler(Handler)
        .await
        .expect("Unable to create client");

    if let Err(e) = client.start().await {
        log::error!("Client error: {:?}", e);
    }
}
