use crate::client::create_client;
use sysinfo::System;

use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

pub async fn ping(m: &PrivmsgMessage) {
    let client = create_client();

    let (mut _incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client);
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = sysinfo::get_current_pid().unwrap();

    if let Some(process) = sys.process(pid) {
        let process_memory = process.memory();
        let total_memory = sys.total_memory();
        let mem = (process_memory as f64 / total_memory as f64) * 100.0;
        let cpu = process.cpu_usage().round();
        let uptime = process.run_time();

        let host = System::name().unwrap();
        let s = format!(
            "Pong! | ↑: {}s | Host: {:?} | Mem: {:.2}% | CPU: {:?}%",
            uptime, host, mem, cpu
        );
        let _message = client.say(m.channel_login.to_owned(), s.to_owned()).await;
    }
}

pub async fn test(m: &PrivmsgMessage) {
    let client = create_client();

    let (mut _incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client);

    let _message = client
        .say(m.channel_login.to_owned(), "test".to_owned())
        .await;
}
