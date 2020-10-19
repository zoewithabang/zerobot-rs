use crate::cytube;
use serenity::{model::channel::Message, prelude::*};
use std::error::Error;

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
