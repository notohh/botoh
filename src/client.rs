use std::env;

use dotenv::dotenv;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;

pub fn create_client() -> ClientConfig<StaticLoginCredentials> {
    dotenv().ok();
    let twitch_id = env::var("TWITCH_ID").expect("Failed to load twitch id");

    let twitch_oauth = env::var("TWITCH_OAUTH").expect("Failed to load oauth");

    let login_creds = StaticLoginCredentials::new(twitch_id, Some(twitch_oauth));

    ClientConfig::new_simple(login_creds)
}
