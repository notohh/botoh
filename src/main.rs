use commands::lastfm::lastfm_command;
use commands::logs::logs_command;
use commands::massping::massping_command;
use commands::ping::ping_command;
use commands::user::get_user_command;
use std::collections::HashMap;

use client::client;
use twitch_irc::message::ServerMessage;

mod api;
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
    initial_channels.insert("daph", ());

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
                    let channel = msg.channel_login.clone();
                    let sender = msg.sender.name.clone();
                    let contents = msg.message_text.clone();
                    let prefix = "*";

                    println!("(#{}) {}: {}", &channel, &sender, &contents);

                    if sender == "notohh" && contents.starts_with(prefix) {
                        let mut parts = contents.split_whitespace();
                        let command = parts.next().unwrap_or("").trim_start_matches(prefix);
                        let arguments: Vec<&str> = parts.collect();

                        match command {
                            "ping" => ping_command(&msg, &client).await.unwrap_or_default(),
                            "song" => lastfm_command(&msg, &client).await.unwrap_or_default(),
                            "user" => get_user_command(&msg, &client, &arguments)
                                .await
                                .unwrap_or_default(),
                            "logs" => logs_command(&msg, &client, &arguments).await,
                            "massping" => massping_command(&msg, &client).await.unwrap_or_default(),
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
