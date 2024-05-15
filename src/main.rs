use dotenv::dotenv;
use std::env;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let twitch_id = match env::var("TWITCH_ID").to_owned() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("TWITCH_ID not found in environment variables.");
            return;
        }
    };

    let twitch_oauth = match env::var("TWITCH_OAUTH").to_owned() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("TWITCH_OAUTH not found in environment variables.");
            return;
        }
    };

    let config =
        ClientConfig::new_simple(StaticLoginCredentials::new(twitch_id, Some(twitch_oauth)));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle =
        tokio::spawn(async move { while let Some(message) = incoming_messages.recv().await {} });

    client.join("notnotoh".to_owned()).unwrap();

    client
        .say("notnotoh".to_owned(), "test".to_owned())
        .await
        .unwrap();

    join_handle.await.unwrap();
}
