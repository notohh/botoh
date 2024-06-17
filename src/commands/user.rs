use dotenv::dotenv;
use twitch_irc::message::PrivmsgMessage;

use serde::{Deserialize, Serialize};

use crate::api::helix_client;
use crate::client::TwitchClient;

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

pub async fn get_user_command(m: &PrivmsgMessage, c: &TwitchClient, a: &[&str]) {
    dotenv().ok();

    let url = format!("https://api.twitch.tv/helix/users?login={}", a.join(" "));

    let helix_client = helix_client();

    let twitch_client = c.twitch_client.clone();

    match helix_client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap_or_default();
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
                twitch_client
                    .say(m.channel_login.to_owned(), error)
                    .await
                    .expect("Error sending message to twitch.");
            }
        }
        Err(e) => error!("Error sending request: {}", e),
    }
}
