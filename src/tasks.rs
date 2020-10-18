use crate::cytube;
use serenity::{model::gateway::Activity, prelude::*};
use std::io::Error as IoError;
use tokio::time::{self, Duration};

pub async fn cytube_now_playing_presence(
    context: &Context,
    cytube_log: &str,
) -> Result<(), IoError> {
    let mut presence_timer = time::interval(Duration::from_secs(5));

    loop {
        presence_timer.tick().await;
        let cytube_log = cytube_log.to_string();

        if let Some(media) =
            tokio::task::spawn_blocking(move || cytube::get_now_playing(&cytube_log)).await??
        {
            context
                .set_activity(Activity::listening(&media.title))
                .await;
        }
    }
}
