use dotenv::dotenv;
use std::error::Error;
use twitch_irc::message::PrivmsgMessage;

use serde::{Deserialize, Serialize};

use crate::api::HelixClient;
use crate::client::TwitchClient;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    data: Vec<UserData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    user_name: String,
}

pub async fn massping_command(m: &PrivmsgMessage, c: &TwitchClient) -> Result<(), Box<dyn Error>> {
    let base_url =
        "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=69768189&moderator_id=69768189";

    let helix_client = HelixClient::new(base_url).client;

    let twitch_client = c.twitch_client.clone();

    match helix_client.get(base_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await?;
                match serde_json::from_str::<Data>(&body) {
                    Ok(payload) => {
                        for items in payload.data {
                            let s = items.user_name.to_string();
                            twitch_client
                                .say(m.channel_login.to_owned(), s)
                                .await
                                .expect("Failed to send message");
                        }
                    }
                    Err(e) => error!("Failed: {}", e),
                }
            } else {
                let error = format!("Error with response: {}", response.status());
                twitch_client.say(m.channel_login.to_owned(), error).await?
            }
        }
        Err(e) => error!("Error sending request: {}", e),
    }
    Ok(())
}
