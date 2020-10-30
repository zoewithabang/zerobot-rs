use crate::config::Config;
use serenity::builder::CreateMessage;

pub fn commands<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    config: &Config,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}commands", &config.bot_prefix),
            "Use this to find out what commands I'm listening for!",
            false,
        );

        create_embed.colour(config.bot_colour)
    })
}

pub fn help<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    config: &Config,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}help [command]", &config.bot_prefix),
            "Let me tell you how a command works!",
            false,
        );

        create_embed.footer(|create_embed_footer| {
            create_embed_footer.text(format!(
                "Use {}commands to find out what commands I'm listening for!",
                &config.bot_prefix
            ))
        });

        create_embed.colour(config.bot_colour)
    })
}

pub fn now_playing<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    config: &Config,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}np", &config.bot_prefix),
            "I'll let you know what's playing on ZeroTube!",
            false,
        );

        create_embed.colour(config.bot_colour)
    })
}

pub fn ping<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    config: &Config,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}ping", &config.bot_prefix),
            "Pong! I'll tell you how long it took me to see your message and successfully respond.",
            false,
        );

        create_embed.footer(|create_embed_footer| {
            create_embed_footer.text("Maybe good for checking if Discord is dying...?")
        });

        create_embed.colour(config.bot_colour)
    })
}

pub fn unknown<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    config: &Config,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field("???", "That's not a command I'm listening for...", false);

        create_embed.footer(|create_embed_footer| {
            create_embed_footer.text(format!("Maybe try {}commands instead?", &config.bot_prefix))
        });

        create_embed.colour(config.bot_colour)
    })
}
