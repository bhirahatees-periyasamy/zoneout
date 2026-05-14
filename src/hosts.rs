use crate::domains;
use std::fs;
use std::io;
use std::process::Command;

pub const HOSTS_PATH: &str = "/etc/hosts";
const MARKER_BEGIN: &str = "# BEGIN FOCUS BLOCK";
const MARKER_END: &str = "# END FOCUS BLOCK";

pub const BLOCKED_DOMAINS: &[&str] = &[
    "claude.ai",
    "api.anthropic.com",
    "anthropic.com",
    "chat.openai.com",
    "chatgpt.com",
    "api.openai.com",
    "copilot.microsoft.com",
];

pub fn is_blocking() -> bool {
    fs::read_to_string(HOSTS_PATH)
        .map(|content| content.contains(MARKER_BEGIN))
        .unwrap_or(false)
}

pub fn all_blocked_domains() -> Vec<String> {
    let mut all: Vec<String> = BLOCKED_DOMAINS.iter().map(|s| s.to_string()).collect();
    for d in domains::load() {
        if !all.contains(&d) {
            all.push(d);
        }
    }
    all
}

pub fn add_block(session_domains: &[String]) -> io::Result<()> {
    let mut content = fs::read_to_string(HOSTS_PATH)?;

    let mut all = all_blocked_domains();
    for d in session_domains {
        let d = d.trim_start_matches("www.").to_lowercase();
        if !all.contains(&d) {
            all.push(d);
        }
    }

    let block: String = std::iter::once(MARKER_BEGIN.to_string())
        .chain(all.iter().map(|d| format!("127.0.0.1 {}", d)))
        .chain(std::iter::once(MARKER_END.to_string()))
        .collect::<Vec<_>>()
        .join("\n");

    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push('\n');
    content.push_str(&block);
    content.push('\n');

    fs::write(HOSTS_PATH, content)?;
    flush_dns()?;
    Ok(())
}

pub fn remove_block() -> io::Result<()> {
    let content = fs::read_to_string(HOSTS_PATH)?;
    let mut inside = false;
    let mut lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.trim() == MARKER_BEGIN {
            inside = true;
            continue;
        }
        if line.trim() == MARKER_END {
            inside = false;
            continue;
        }
        if !inside {
            lines.push(line);
        }
    }

    // Remove trailing blank lines that were padding around the block.
    while lines.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        lines.pop();
    }

    let mut result = lines.join("\n");
    result.push('\n');
    fs::write(HOSTS_PATH, result)?;
    flush_dns()?;
    Ok(())
}

fn flush_dns() -> io::Result<()> {
    Command::new("dscacheutil").arg("-flushcache").status()?;
    Command::new("killall").args(["-HUP", "mDNSResponder"]).status()?;
    Ok(())
}
