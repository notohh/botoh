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

        let uptime_seconds = process.run_time();
        let uptime_minute = uptime_seconds / 60;
        let remaining_seconds = uptime_seconds % 60;

        let host = System::name().unwrap();

        let s = format!(
            "🚀Pong! | ↑: {}m {}s | Host: {} | Mem: {:.2} MB",
            uptime_minute, remaining_seconds, host, mem
        );
        let _message = c.twitch_client.say(m.channel_login.to_owned(), s).await;
    }
}
