use crate::client::create_client;

use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

pub async fn ping(m: &PrivmsgMessage) {
    let client = create_client();

    let (mut _incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client);

    let s = format!("Pong!");

    let _message = client.say(m.channel_login.to_owned(), s.to_owned()).await;
}

pub async fn test(m: &PrivmsgMessage) {
    let client = create_client();

    let (mut _incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client);

    let _message = client
        .say(m.channel_login.to_owned(), "test".to_owned())
        .await;
}
