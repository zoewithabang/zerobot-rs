use serenity::utils::Colour;

pub struct Config {
    pub bot_colour: Colour,
    pub bot_prefix: String,
    pub cytube_log: String,
    pub cytube_url: String,
    pub discord_token: String,
}

impl Config {
    pub fn new(
        bot_colour: String,
        bot_prefix: String,
        cytube_log: String,
        cytube_url: String,
        discord_token: String,
    ) -> Self {
        let bot_colour = u32::from_str_radix(bot_colour.trim_start_matches("0x"), 16)
            .expect("BOT_COLOUR unable to be parsed, expected hexadecimal format, e.g. 0xFFFFFF")
            .into();

        Config {
            bot_colour,
            bot_prefix,
            cytube_log,
            cytube_url,
            discord_token,
        }
    }
}
