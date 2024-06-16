use crate::client::TwitchClient;

use twitch_irc::message::PrivmsgMessage;

pub async fn test_command(m: &PrivmsgMessage, c: &TwitchClient) {
    let _message = c
        .twitch_client
        .say(m.channel_login.to_owned(), "test".to_owned())
        .await;
}
