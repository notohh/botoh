use commands::lastfm::lastfm_command;
use commands::ping::ping_command;
use std::collections::HashMap;

use client::client;
use twitch_irc::message::ServerMessage;

mod client;
mod commands;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::try_init().expect("Failed to load logger");
    let mut initial_channels = HashMap::new();

    let mut client = client();

    initial_channels.insert("notnotoh", ());
    initial_channels.insert("notohh", ());

    for (channels, _) in initial_channels.iter() {
        match client.twitch_client.join(channels.to_owned().to_string()) {
            Ok(_) => info!("Joined channel {}", channels),
            Err(e) => error!("Failed to join channels! {}", e),
        }
    }

    let message_handler = tokio::spawn(async move {
        while let Some(message) = client.incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "(#{}) {}: {}",
                        msg.channel_login, msg.sender.name, msg.message_text
                    );
                    if msg.sender.name == "notohh" {
                        match msg.message_text.as_str() {
                            "*ping" => ping_command(&msg, &client).await,
                            "*song" => lastfm_command(&msg, &client).await,
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
