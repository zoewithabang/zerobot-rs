mod commands;
mod config;
mod cytube;
mod tasks;

use crate::config::Config;
use backoff::{future::FutureOperation, ExponentialBackoff};
use lazy_static::lazy_static;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

lazy_static! {
    static ref CONFIG: Config = Config::new(
        env::var("BOT_COLOUR").expect("BOT_COLOUR missing!"),
        env::var("BOT_PREFIX").expect("BOT_PREFIX missing!"),
        env::var("CYTUBE_LOG").expect("CYTUBE_LOG missing!"),
        env::var("CYTUBE_URL").expect("CYTUBE_URL missing!"),
        env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing!"),
    );
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if !message.content.starts_with(&CONFIG.bot_prefix.as_str()) {
            return;
        }

        let command = message
            .content
            .strip_prefix(&CONFIG.bot_prefix.as_str())
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();

        if let Err(e) = match command {
            "commands" => commands::commands(context, message, &CONFIG).await,
            "help" => commands::help(context, message, &CONFIG).await,
            "np" => commands::now_playing(context, message, &CONFIG).await,
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
            tasks::cytube_now_playing_presence(&context, &CONFIG.cytube_log)
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

    let mut client = Client::builder(&CONFIG.discord_token.as_str())
        .event_handler(Handler)
        .await
        .expect("Unable to create client");

    if let Err(e) = client.start().await {
        log::error!("Client error: {:?}", e);
    }
}
