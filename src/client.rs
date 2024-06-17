use std::env;

use dotenv::dotenv;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

pub struct TwitchClient {
    pub incoming_messages: tokio::sync::mpsc::UnboundedReceiver<ServerMessage>,
    pub twitch_client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
}

pub fn client_config() -> ClientConfig<StaticLoginCredentials> {
    dotenv().ok();
    let twitch_id = env::var("TWITCH_ID").expect("Failed to load twitch id");

    let twitch_oauth = env::var("TWITCH_OAUTH").expect("Failed to load oauth");

    let login_creds = StaticLoginCredentials::new(twitch_id, Some(twitch_oauth));

    ClientConfig::new_simple(login_creds)
}

pub fn client() -> TwitchClient {
    let config = client_config();

    let (incoming_messages, twitch_client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    TwitchClient {
        twitch_client,
        incoming_messages,
    }
}
