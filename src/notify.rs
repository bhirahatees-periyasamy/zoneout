use chrono::{DateTime, Local, Utc};
use std::process::Command;

pub fn notify_enabled(ends_at: &DateTime<Utc>) {
    let local = ends_at.with_timezone(&Local);
    let msg = format!("Focus active until {} — AI assistants blocked.", local.format("%H:%M:%S"));
    osascript(&msg, "Focus Enabled");
}

pub fn notify_disabled() {
    osascript("All AI assistants are now unblocked.", "Focus Disabled");
}

pub fn notify_reminder(remaining_secs: i64) {
    let h = remaining_secs / 3600;
    let m = (remaining_secs % 3600) / 60;
    let msg = if h > 0 {
        format!("{}h {}m remaining in focus session.", h, m)
    } else {
        format!("{}m remaining in focus session.", m)
    };
    osascript(&msg, "Focus");
}

pub fn notify_expired() {
    osascript("Focus session complete. AI assistants are now unblocked.", "Focus Ended");
}

fn osascript(message: &str, title: &str) {
    let script = format!(
        "display notification {:?} with title {:?} sound name \"Glass\"",
        message, title
    );
    let _ = Command::new("osascript").arg("-e").arg(&script).status();
}
