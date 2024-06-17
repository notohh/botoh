use dotenv::dotenv;
use twitch_irc::message::PrivmsgMessage;

use crate::client::TwitchClient;

pub async fn logs_command(m: &PrivmsgMessage, c: &TwitchClient, a: &[&str]) {
    dotenv().ok();

    let url = format!(
        "https://logs.flake.sh/?channel={}&username={}",
        a.first().unwrap(),
        a.get(1).unwrap()
    );

    let twitch_client = c.twitch_client.clone();

    let _response = twitch_client.say(m.channel_login.to_owned(), url).await;
}
