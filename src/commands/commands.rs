use serenity::builder::CreateEmbed;

pub fn commands<'a>(create_embed: &'a mut CreateEmbed, bot_prefix: &str) -> &'a mut CreateEmbed {
    create_embed.field(
        format!("{}commands", bot_prefix),
        "That's this command!",
        false,
    )
}

pub fn help<'a>(create_embed: &'a mut CreateEmbed, bot_prefix: &str) -> &'a mut CreateEmbed {
    create_embed.field(
        format!("{}help", bot_prefix),
        "Find out more about a certain command!",
        false,
    )
}

pub fn now_playing<'a>(create_embed: &'a mut CreateEmbed, bot_prefix: &str) -> &'a mut CreateEmbed {
    create_embed.field(
        format!("{}np", bot_prefix),
        "See what's playing in ZeroTube~",
        false,
    )
}

pub fn ping<'a>(create_embed: &'a mut CreateEmbed, bot_prefix: &str) -> &'a mut CreateEmbed {
    create_embed.field(format!("{}ping", bot_prefix), "Pong!", false)
}
