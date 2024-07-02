use dotenv::dotenv;
use std::error::Error;
use twitch_irc::message::PrivmsgMessage;

use crate::client::TwitchClient;

pub async fn logs_command(
    m: &PrivmsgMessage,
    c: &TwitchClient,
    a: &[&str],
) -> Result<(), Box<dyn Error>> {
    let twitch_client = c.twitch_client.clone();

    if let Some(_args) = a.first() {
        let base_url = format!(
            "https://logs.flake.sh/?username={}&channel={}",
            a.first().unwrap(),
            a.get(1).unwrap()
        );
        twitch_client
            .say(m.channel_login.to_owned(), base_url.to_string())
            .await?
    } else {
        twitch_client
            .say(
                m.channel_login.to_owned(),
                "Please provide args: <username> <channel> ".to_string(),
            )
            .await?
    }

    Ok(())
}
