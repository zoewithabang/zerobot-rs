mod commands;
mod cytube;
mod tasks;

use backoff::{future::FutureOperation, ExponentialBackoff};
use dotenv;
use lazy_static::lazy_static;
use log;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

// TODO: single config object instead for ease of passing around?
lazy_static! {
    static ref BOT_PREFIX: String = env::var("BOT_PREFIX").expect("BOT_PREFIX missing!");
    static ref CYTUBE_LOG: String = env::var("CYTUBE_LOG").expect("CYTUBE_LOG missing!");
    static ref CYTUBE_URL: String = env::var("CYTUBE_URL").expect("CYTUBE_URL missing!");
    static ref DISCORD_TOKEN: String = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing!");
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if !message.content.starts_with(&BOT_PREFIX.as_str()) {
            return;
        }

        let command = message.content.strip_prefix(&BOT_PREFIX.as_str()).unwrap();

        if let Err(e) = match command {
            "np" => commands::now_playing(context, message, &CYTUBE_LOG, &CYTUBE_URL).await,
            "ping" => commands::ping(context, message).await,
            _ => Ok(()),
        } {
            log::error!("Error sending message: {:?}", e);
        };
    }

    // Note: `let _ =` is used to discard Results where we're not using backoff's permanent errors
    async fn ready(&self, context: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        let _ = (|| async {
            tasks::cytube_now_playing_presence(&context, &CYTUBE_LOG)
                .await
                .map_err(|e| {
                    log::error!("CyTube now playing presence error: {:?}", e);

                    e.into()
                })
        })
        .retry(ExponentialBackoff::default())
        .await;
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
