use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

pub struct HelixClient {
    pub base_url: String,
    pub headers: HeaderMap<HeaderValue>,
    pub client: Client,
}

// pub struct GqlClient {
//     pub base_url: String,
//     pub headers: HeaderMap<HeaderValue>,
//     pub client: Client,
// }

impl HelixClient {
    pub fn new(base_url: &str) -> HelixClient {
        let twitch_client_id =
            env::var("TWITCH_CLIENT_ID").expect("Couldnt load twitch client id.");
        let twitch_auth_token = env::var("TWITCH_AUTH").expect("Couldnt load twitch auth token.");

        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&twitch_auth_token).expect("Failed to insert twitch auto token."),
        );
        headers.insert(
            "Client-Id",
            HeaderValue::from_str(&twitch_client_id).expect("Failed to insert twitch client id."),
        );

        let client = Client::builder()
            .default_headers(headers.clone())
            .build()
            .unwrap();

        HelixClient {
            base_url: base_url.to_string(),
            headers,
            client,
        }
    }
}

// impl GqlClient {
//     pub fn new() {}
// }
