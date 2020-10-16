use serenity::{model::channel::Message, prelude::*, Error as SerenityError};

pub async fn ping(context: Context, ping: Message) -> Result<(), SerenityError> {
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
