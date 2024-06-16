use commands::ping::ping_command;
use commands::test::test_command;
use std::collections::HashMap;

use client::client;
use twitch_irc::message::ServerMessage;

mod client;
mod commands;

#[tokio::main]
pub async fn main() {
    let mut initial_channels = HashMap::new();

    let mut client = client();

    initial_channels.insert("notnotoh", ());
    initial_channels.insert("notohh", ());
    initial_channels.insert("daph", ());

    for (channels, _) in initial_channels.iter() {
        match client.twitch_client.join(channels.to_owned().to_string()) {
            Ok(_) => println!("Joined channel {}", channels),
            Err(e) => eprintln!("Failed to join channels! {}", e),
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
                            "*test" => test_command(&msg, &client).await,
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
