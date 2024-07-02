use dotenv::dotenv;
use twitch_irc::message::PrivmsgMessage;

use serde::{Deserialize, Serialize};

use crate::api::HelixClient;
use crate::client::TwitchClient;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    data: Vec<UserData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    broadcaster_type: String,
    created_at: String,
    description: String,
    display_name: String,
    id: String,
    login: String,
    offline_image_url: String,
    profile_image_url: String,
    #[serde(rename = "type")]
    datum_type: String,
    view_count: i64,
}

pub async fn get_user_command(
    m: &PrivmsgMessage,
    c: &TwitchClient,
    a: &[&str],
) -> Result<(), Box<dyn Error>> {
    let base_url = format!("https://api.twitch.tv/helix/users?login={}", a.join(" "));
    let helix_client = HelixClient::new(&base_url).client;
    let twitch_client = c.twitch_client.clone();

    match helix_client.get(base_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await?;
                match serde_json::from_str::<Data>(&body) {
                    Ok(payload) => {
                        for items in payload.data {
                            let s = format!(
                                "Name: {} | Created: {} | ID: {} | Broadcaster status: {} | PFP: {}",
                                items.display_name,
                                items.created_at,
                                items.id,
                                items.broadcaster_type.to_uppercase(),
                                items.profile_image_url
                            );
                            twitch_client.say(m.channel_login.to_owned(), s).await?
                        }
                    }
                    Err(e) => error!("Failed: {}", e),
                }
            } else {
                twitch_client
                    .say(m.channel_login.to_owned(), response.status().to_string())
                    .await?
            }
        }
        Err(e) => error!("Error sending request: {}", e),
    }

    Ok(())
}
