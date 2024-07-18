use std::thread::sleep;
use std::time::Duration;

use commands::lastfm::lastfm_command;
use commands::logs::logs_command;
use commands::massping::massping_command;
use commands::ping::ping_command;
use commands::user::get_user_command;
use dotenv::dotenv;

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
    dotenv().ok();
    pretty_env_logger::try_init().expect("Failed to load logger");
    let mut client = client();

    let initial_channels = vec!["notnotoh", "notohh", "daph", "fembotfriday", "miwo", "elis"];

    for &channel in &initial_channels {
        match client.twitch_client.join(channel.to_string()) {
            Ok(_) => info!("Joined channel {}", channel),
            Err(e) => error!("Failed to join channels! {}", e),
        }
    }

    let message_handler = tokio::spawn(async move {
        while let Some(message) = client.incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(m) => {
                    let is_moderator = m.badges.iter().any(|badge| badge.name == "moderator");
                    let is_broadcaster = m.badges.iter().any(|badge| badge.name == "broadcaster");
                    let channel = m.channel_login.clone();
                    let sender = m.sender.name.clone();
                    let contents = m.message_text.clone();
                    let prefix = "*";

                    println!("(#{}) {}: {}", &channel, &sender, &contents);

                    if sender == "notohh" && contents.starts_with(prefix) {
                        let mut parts = contents.split_whitespace();
                        let command = parts.next().unwrap_or("").trim_start_matches(prefix);
                        let arguments: Vec<&str> = parts.collect();

                        match command {
                            "ping" => {
                                if is_moderator || is_broadcaster {
                                    ping_command(&m, &client).await.unwrap_or_default();
                                } else {
                                    sleep(Duration::from_secs(1));
                                    ping_command(&m, &client).await.unwrap_or_default();
                                }
                            }
                            "song" => {
                                if is_moderator || is_broadcaster {
                                    lastfm_command(&m, &client).await.unwrap_or_default();
                                } else {
                                    sleep(Duration::from_secs(1));
                                    lastfm_command(&m, &client).await.unwrap_or_default();
                                }
                            }
                            "user" => {
                                if is_moderator || is_broadcaster {
                                    get_user_command(&m, &client, &arguments)
                                        .await
                                        .unwrap_or_default();
                                } else {
                                    sleep(Duration::from_secs(1));
                                    get_user_command(&m, &client, &arguments)
                                        .await
                                        .unwrap_or_default();
                                }
                            }
                            "logs" => {
                                if is_moderator || is_broadcaster {
                                    logs_command(&m, &client, &arguments)
                                        .await
                                        .unwrap_or_default();
                                } else {
                                    sleep(Duration::from_secs(1));
                                    logs_command(&m, &client, &arguments)
                                        .await
                                        .unwrap_or_default();
                                }
                            }
                            "massping" => {
                                if is_moderator || is_broadcaster {
                                    massping_command(&m, &client).await.unwrap_or_default();
                                } else {
                                    sleep(Duration::from_secs(1));
                                    massping_command(&m, &client).await.unwrap_or_default()
                                }
                            }
                            _ => {}
                        }
                    }
                }
                ServerMessage::Whisper(m) => {
                    println!("(w) {}: {}", m.sender.name, m.message_text);
                }
                _ => {}
            }
        }
    });

    message_handler.await.unwrap();
}
