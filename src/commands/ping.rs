use crate::client::TwitchClient;
use sysinfo::System;

use std::{error::Error, thread::sleep, time::Duration};
use twitch_irc::message::PrivmsgMessage;

pub async fn ping_command(m: &PrivmsgMessage, c: &TwitchClient) -> Result<(), Box<dyn Error>> {
    let mut sys = System::new_all();

    sys.refresh_all();

    let pid = sysinfo::get_current_pid().unwrap();
    let channel = m.channel_login.to_owned();

    if let Some(process) = sys.process(pid) {
        let process_memory = process.memory();
        let mem = process_memory as f64 / (1024.0 * 1024.0);

        let uptime_seconds = process.run_time();
        let uptime_minute = uptime_seconds / 60;
        let remaining_seconds = uptime_seconds % 60;
        let remaining_hour = uptime_minute % 60;
        let is_moderator = m.badges.iter().any(|badge| badge.name == "moderator");

        let host = System::name().unwrap();

        let s = format!(
            "🚀Pong! | ↑: {}h {}m {}s | Host: {} | Mem: {:.2} MB",
            uptime_minute, remaining_hour, remaining_seconds, host, mem
        );

        // need to make this check global for all commands eventually

        if is_moderator {
            c.twitch_client.say(channel, s).await?;
        } else {
            sleep(Duration::from_secs(1));
            c.twitch_client.say(channel, s).await?;
        }
    }
    Ok(())
}
