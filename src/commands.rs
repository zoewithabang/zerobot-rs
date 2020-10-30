mod command_info;
mod help;

use crate::cytube;
use serenity::{model::channel::Message, prelude::*};
use std::error::Error;

pub async fn commands(
    context: Context,
    message: Message,
    bot_prefix: &str,
) -> Result<(), Box<dyn Error>> {
    message
        .channel_id
        .send_message(&context.http, |create_message| {
            create_message.embed(|create_embed| {
                command_info::commands(create_embed, bot_prefix);
                command_info::help(create_embed, bot_prefix);
                command_info::now_playing(create_embed, bot_prefix);
                command_info::ping(create_embed, bot_prefix);

                // TODO: move the colour to config
                create_embed.colour((250, 207, 255))
            })
        })
        .await?;

    Ok(())
}

pub async fn help(
    context: Context,
    message: Message,
    bot_prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let text = message.content.split_whitespace().nth(2);

    message
        .channel_id
        .send_message(&context.http, |create_message| match text {
            Some("commands") => help::commands(create_message, bot_prefix),
            Some("help") | None => help::help(create_message, bot_prefix),
            Some("np") => help::now_playing(create_message, bot_prefix),
            Some("ping") => help::ping(create_message, bot_prefix),
            _ => help::unknown(create_message, bot_prefix),
        })
        .await?;

    Ok(())
}

pub async fn now_playing(
    context: Context,
    message: Message,
    cytube_log: &str,
    cytube_url: &str,
) -> Result<(), Box<dyn Error>> {
    let cytube_log = cytube_log.to_string();

    if let Some(media) =
        tokio::task::spawn_blocking(move || cytube::get_now_playing(&cytube_log)).await??
    {
        message
            .channel_id
            .send_message(&context.http, |create_message| {
                create_message.embed(|create_embed| {
                    create_embed.field(
                        &media.title,
                        format!(
                            "[{}]({}) || [Tune in~]({}?queue={})",
                            &media.service,
                            &media.get_url(),
                            &cytube_url,
                            &media.id,
                        ),
                        false,
                    );

                    // TODO: move the colour to config
                    create_embed.colour((250, 207, 255))
                })
            })
            .await?;
    }

    Ok(())
}

pub async fn ping(context: Context, ping: Message) -> Result<(), Box<dyn Error>> {
    let mut pong = ping.channel_id.say(&context.http, "Pong!").await?;
    let content = pong.content.clone();
    let time_ms = (pong.timestamp - ping.timestamp).num_milliseconds();

    let time = if time_ms < 1000 {
        format!("{}ms", time_ms)
    } else {
        format!("{}s", (time_ms as f64) / 1000.0)
    };

    pong.edit(&context, |message| {
        message.content(format!("{} I took {} to respond!", content, time))
    })
    .await?;

    Ok(())
}
