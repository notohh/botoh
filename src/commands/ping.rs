use crate::client::TwitchClient;
use sysinfo::System;

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
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
        let uptime_minutes = uptime_seconds % (60 * 60) / 60;
        let uptime_hours = uptime_seconds / (60 * 60);
        let uptime_days = uptime_hours / 24;

        let remaining_seconds = uptime_seconds % 60;
        let remaining_minutes = uptime_minutes % 60;
        let remaining_hours = uptime_hours % 24;

        let host = System::name().unwrap();

        let s = format!(
            "🚀Pong! | ↑: {}d {}h {}m | Host: {} | Mem: {:.2} MB",
            uptime_days, remaining_hours, remaining_minutes, host, mem
        );

        c.twitch_client.say(channel, s).await?;
    }
    Ok(())
}
