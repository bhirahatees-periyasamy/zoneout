use chrono::{DateTime, TimeZone, Utc};
use std::io;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::{hosts, notify, state};

pub fn spawn_timer_daemon(end_epoch: i64) -> io::Result<u32> {
    let exe = std::env::current_exe()?;
    let child = Command::new(exe)
        .arg("daemon-run")
        .arg(end_epoch.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()?;
    Ok(child.id())
}

pub fn run_daemon(end_epoch: i64) {
    let end_time: DateTime<Utc> = match Utc.timestamp_opt(end_epoch, 0) {
        chrono::LocalResult::Single(t) => t,
        _ => return,
    };

    let mut last_reminder = Utc::now();

    loop {
        let now = Utc::now();
        if now >= end_time {
            break;
        }

        let remaining_secs = (end_time - now).num_seconds();

        // Fire reminder notification every 60 seconds.
        if (now - last_reminder).num_seconds() >= 60 {
            notify::notify_reminder(remaining_secs);
            last_reminder = now;
        }

        // Sleep in short intervals to handle macOS sleep/wake correctly.
        let sleep_secs = remaining_secs.min(30).max(1) as u64;
        std::thread::sleep(Duration::from_secs(sleep_secs));
    }

    let _ = hosts::remove_block();
    let _ = state::clear();
    notify::notify_expired();
}
