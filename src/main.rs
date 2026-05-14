mod cli;
mod daemon;
mod domains;
mod hosts;
mod notify;
mod state;
mod timer;

use chrono::{Local, Utc};
use clap::Parser;
use cli::{Cli, Command};
use std::io::Write;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Enable(args) => cmd_enable(args),
        Command::Disable => cmd_disable(),
        Command::Status => cmd_status(),
        Command::Add { domain } => cmd_add(&domain),
        Command::Remove(args) => cmd_remove(args),
        Command::List => cmd_list(),
        Command::DaemonRun { end_epoch } => daemon::run_daemon(end_epoch),
    }
}

fn cmd_enable(args: cli::EnableArgs) {
    let duration = timer::duration_from_args(
        args.time.as_deref(),
        args.hours,
        args.minutes,
    )
    .unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    if hosts::is_blocking() {
        if let Some(s) = state::load() {
            let remaining = (s.ends_at - Utc::now()).num_seconds().max(0);
            eprintln!(
                "Focus is already active. {} remaining.",
                timer::fmt_duration_secs(remaining as u64)
            );
        } else {
            eprintln!("Focus is already active. Run 'sudo zoneout disable' first.");
        }
        std::process::exit(0);
    }

    hosts::add_block(&args.extra_domains).unwrap_or_else(|e| {
        eprintln!("Failed to modify /etc/hosts: {}", e);
        eprintln!("Try: sudo zoneout enable ...");
        std::process::exit(1);
    });

    let ends_at = Utc::now()
        + chrono::Duration::from_std(duration).expect("duration in range");
    let end_epoch = ends_at.timestamp();

    let pid = daemon::spawn_timer_daemon(end_epoch).unwrap_or_else(|e| {
        eprintln!("Daemon spawn failed: {}. Rolling back...", e);
        let _ = hosts::remove_block();
        std::process::exit(1);
    });

    let st = state::FocusState {
        enabled: true,
        started_at: Utc::now(),
        ends_at,
        daemon_pid: pid,
    };
    state::save(&st).ok();
    notify::notify_enabled(&ends_at);

    let local_end = ends_at.with_timezone(&Local);
    let total_secs = duration.as_secs();
    println!("Focus enabled for {}", timer::fmt_duration_secs(total_secs));
    println!("  Ends at:  {}", local_end.format("%H:%M:%S"));
    println!("  You'll get a reminder notification every minute.");
    println!("\nRun 'zoneout status' to see a live countdown.");
    println!("Run 'zoneout list' to see all blocked domains.");
}

fn cmd_disable() {
    hosts::remove_block().unwrap_or_else(|e| {
        eprintln!("Failed to modify /etc/hosts: {}", e);
        eprintln!("Try: sudo zoneout disable");
        std::process::exit(1);
    });

    if let Some(s) = state::load() {
        let _ = std::process::Command::new("kill")
            .arg(s.daemon_pid.to_string())
            .status();
    }

    state::clear().ok();
    notify::notify_disabled();
    println!("Focus disabled. All AI assistants unblocked.");
}

fn cmd_status() {
    if !hosts::is_blocking() {
        println!("Focus is not active.");
        return;
    }

    let state = state::load();

    match &state {
        Some(s) => {
            let local_end = s.ends_at.with_timezone(&Local);
            println!("Focus active — ends at {}", local_end.format("%H:%M:%S"));
        }
        None => {
            println!("Focus active (no state file — duration unknown).");
        }
    }

    println!("Press Ctrl+C to exit status view.\n");

    loop {
        let remaining = match &state {
            Some(s) => (s.ends_at - Utc::now()).num_seconds().max(0),
            None => {
                print!("\r  Focus active (duration unknown)     ");
                std::io::stdout().flush().ok();
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        };

        let h = remaining / 3600;
        let m = (remaining % 3600) / 60;
        let s = remaining % 60;
        print!("\r  {:02}h {:02}m {:02}s remaining    ", h, m, s);
        std::io::stdout().flush().ok();

        if remaining == 0 {
            println!("\n\nFocus session ended.");
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn cmd_add(domain: &str) {
    match domains::add(domain) {
        Ok(true) => println!("Added '{}' to the block list.", domain.trim_start_matches("www.").to_lowercase()),
        Ok(false) => println!("'{}' is already in the block list.", domain),
        Err(e) => {
            eprintln!("Failed to save domain: {}", e);
            std::process::exit(1);
        }
    }
    println!("It will be blocked the next time you run 'sudo zoneout enable'.");
}

fn cmd_remove(args: cli::RemoveArgs) {
    if args.all {
        domains::remove_all().unwrap_or_else(|e| {
            eprintln!("Failed to remove domains: {}", e);
            std::process::exit(1);
        });
        println!("All custom domains removed.");
        return;
    }

    match args.domain {
        Some(domain) => match domains::remove(&domain) {
            Ok(true) => println!("Removed '{}' from the block list.", domain),
            Ok(false) => println!("'{}' was not in the custom block list.", domain),
            Err(e) => {
                eprintln!("Failed to update domains: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Specify a domain to remove, or use --all to remove all custom domains.");
            std::process::exit(1);
        }
    }
}

fn cmd_list() {
    let defaults = hosts::BLOCKED_DOMAINS;
    let custom = domains::load();

    println!("Default domains:");
    for d in defaults {
        println!("  {}", d);
    }

    if custom.is_empty() {
        println!("\nCustom domains: (none)");
        println!("\nTip: run 'zoneout add <domain>' to add your own.");
    } else {
        println!("\nCustom domains:");
        for d in &custom {
            println!("  {}", d);
        }
    }
}
