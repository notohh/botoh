use crate::client::TwitchClient;
use sysinfo::System;

use twitch_irc::message::PrivmsgMessage;

pub async fn ping_command(m: &PrivmsgMessage, c: &TwitchClient) {
    let mut sys = System::new_all();

    sys.refresh_all();

    let pid = sysinfo::get_current_pid().unwrap();

    if let Some(process) = sys.process(pid) {
        let process_memory = process.memory();
        let mem = process_memory as f64 / (1024.0 * 1024.0);
        let cpu = process.cpu_usage().round();
        let uptime = process.run_time();

        let host = System::name().unwrap();
        let s = format!(
            "Pong! | ↑: {}s | Host: {:?} | Mem: {:.2} MB | CPU: {:?}%",
            uptime, host, mem, cpu
        );
        let _message = c
            .twitch_client
            .say(m.channel_login.to_owned(), s.to_owned())
            .await;
    }
}
