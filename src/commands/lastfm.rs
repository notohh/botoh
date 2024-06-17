use crate::client::TwitchClient;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

use twitch_irc::message::PrivmsgMessage;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    recenttracks: Recenttracks,
}

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    size: String,
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Registered {
    unixtime: String,
    #[serde(rename = "#text")]
    text: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recenttracks {
    track: Vec<Track>,
    #[serde(rename = "@attr")]
    attr: Attr,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attr {
    user: String,
    total_pages: String,
    page: String,
    per_page: String,
    total: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    artist: Album,
    streamable: String,
    image: Vec<Image>,
    mbid: String,
    album: Album,
    name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    mbid: String,
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    uts: String,
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Size {
    Extralarge,
    Large,
    Medium,
    Small,
}

pub async fn lastfm_command(m: &PrivmsgMessage, c: &TwitchClient) {
    dotenv().ok();

    let lastfm_api_key = env::var("LASTFM_API_KEY").expect("Failed to load lastfm api key.");
    let user = "notoh";

    let recent_tracks_url = format!(
        "http://ws.audioscrobbler.com/2.0/?method=user.getRecentTracks&user={}&api_key={}&format=json&nowplaying=true", user,
        lastfm_api_key
    );

    let client = reqwest::Client::new();

    match client.get(recent_tracks_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap_or_default();
                match serde_json::from_str::<Data>(&body) {
                    Ok(payload) => {
                        if let Some(tracks) = payload.recenttracks.track.first() {
                            let s = format!(
                                "Listening to: {} - {} {}",
                                tracks.name, tracks.artist.text, tracks.url
                            );
                            c.twitch_client
                                .say(m.channel_login.to_owned(), s.to_owned())
                                .await
                                .expect("Error sending message to twitch");
                        }
                    }
                    Err(e) => error!("{}", e),
                }
            } else {
                error!("Response error: {}", response.status());
            }
        }
        Err(e) => error!("Error sending request: {}", e),
    }
}
