use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

pub fn helix_client() -> Client {
    let twitch_client_id = env::var("TWITCH_CLIENT_ID").expect("a");
    let twitch_bearer = env::var("TWITCH_BEARER").expect("a");

    let mut headers = HeaderMap::new();

    headers.insert(
        "Authorization",
        HeaderValue::from_str(&twitch_bearer).expect("a"),
    );
    headers.insert(
        "Client-Id",
        HeaderValue::from_str(&twitch_client_id).expect("a"),
    );

    let client = Client::builder().default_headers(headers).build().unwrap();

    return client;
}
