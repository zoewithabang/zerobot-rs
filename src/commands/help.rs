use serenity::builder::CreateMessage;

pub fn commands<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    bot_prefix: &str,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}commands", bot_prefix),
            "Use this to find out what commands I'm listening for!",
            false,
        );

        // TODO: move the colour to config
        create_embed.colour((250, 207, 255))
    })
}

pub fn help<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    bot_prefix: &str,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}help [command]", bot_prefix),
            "Let me tell you how a command works!",
            false,
        );

        create_embed.footer(|create_embed_footer| {
            create_embed_footer.text(format!(
                "Use {}commands to find out what commands I'm listening for!",
                bot_prefix
            ))
        });

        // TODO: move the colour to config
        create_embed.colour((250, 207, 255))
    })
}

pub fn now_playing<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    bot_prefix: &str,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}np", bot_prefix),
            "I'll let you know what's playing on ZeroTube!",
            false,
        );

        // TODO: move the colour to config
        create_embed.colour((250, 207, 255))
    })
}

pub fn ping<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    bot_prefix: &str,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field(
            format!("{}ping", bot_prefix),
            "Pong! I'll tell you how long it took me to see your message and successfully respond.",
            false,
        );

        create_embed.footer(|create_embed_footer| {
            create_embed_footer.text("Maybe good for checking if Discord is dying...?")
        });

        // TODO: move the colour to config
        create_embed.colour((250, 207, 255))
    })
}

pub fn unknown<'a, 'b>(
    create_message: &'b mut CreateMessage<'a>,
    bot_prefix: &str,
) -> &'b mut CreateMessage<'a> {
    create_message.embed(|create_embed| {
        create_embed.field("???", "That's not a command I'm listening for...", false);

        create_embed.footer(|create_embed_footer| {
            create_embed_footer.text(format!("Maybe try {}commands instead?", bot_prefix))
        });

        // TODO: move the colour to config
        create_embed.colour((250, 207, 255))
    })
}
