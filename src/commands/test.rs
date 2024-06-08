use crate::client::create_client;
use sysinfo::System;

use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

pub async fn test_command(m: &PrivmsgMessage) {
    let client = create_client();

    let (mut _incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client);
    let _message = client
        .say(m.channel_login.to_owned(), "test".to_owned())
        .await;
}
