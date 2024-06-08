use client::create_client;
use commands::*;
use std::collections::HashMap;

use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, SecureTCPTransport, TwitchIRCClient,
};

mod client;
mod commands;

#[tokio::main]
pub async fn main() {
    let client = create_client();

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client);

    let mut initial_channels = HashMap::new();

    initial_channels.insert("notnotoh", ());
    initial_channels.insert("notohh", ());
    initial_channels.insert("daph", ());
    initial_channels.insert("ryanpotat", ());

    for (channels, _) in initial_channels.iter() {
        match client.join(channels.to_owned().to_string()) {
            Ok(_) => println!("Joined channel {}", channels),
            Err(e) => eprintln!("Failed to join channels! {}", e),
        }
    }

    let message_handler = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "(#{}) {}: {}",
                        msg.channel_login, msg.sender.name, msg.message_text
                    );
                    if msg.sender.name == "notohh" {
                        match msg.message_text.as_str() {
                            "*ping" => ping(&msg).await,
                            "*test" => test(&msg).await,
                            _ => {}
                        }
                    }
                }
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                }
                _ => {}
            }
        }
    });

    message_handler.await.unwrap();
}
