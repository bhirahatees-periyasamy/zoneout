use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zoneout", about = "Block AI coding assistants on macOS for a set duration")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Enable focus mode and block AI assistants
    #[command(disable_help_flag = true)]
    Enable(EnableArgs),
    /// Disable focus mode immediately
    Disable,
    /// Show a live countdown of the current focus session
    Status,
    #[command(hide = true)]
    DaemonRun { end_epoch: i64 },
}

#[derive(Args)]
#[group(required = true, multiple = true)]
pub struct EnableArgs {
    /// Duration as HH:MM:SS (e.g. 01:30:00)
    #[arg(long, value_name = "HH:MM:SS", conflicts_with_all = &["hours", "minutes"])]
    pub time: Option<String>,

    /// Hours to block
    #[arg(short = 'h', long, value_name = "N")]
    pub hours: Option<u64>,

    /// Minutes to block
    #[arg(short = 'm', long, value_name = "N")]
    pub minutes: Option<u64>,
}
