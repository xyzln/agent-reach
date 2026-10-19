use clap::{Parser, Subcommand};

use crate::channels;
use crate::config;
use crate::doctor;
use crate::install;
use crate::transcribe;

#[derive(Parser)]
#[command(
    name = "agent-reach",
    version,
    about = "Cross-platform content reach router"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List supported platforms / channels
    Channels,
    /// Dependency and channel check (read-only by default)
    Install {
        /// Environment: auto | local
        #[arg(long, default_value = "auto")]
        env: String,
        /// Install/configure core external tools (requires explicit approval)
        #[arg(long)]
        system: bool,
        /// Preview what would be done without changing the host
        #[arg(long)]
        dry_run: bool,
        /// Comma-separated channel ids to scope the install
        #[arg(long)]
        channels: Option<String>,
    },
    /// Show channel health status
    Doctor {
        /// Emit JSON (active_backend aware)
        #[arg(long)]
        json: bool,
    },
    /// Quick health + update check for scheduled tasks
    Watch,
    /// Configure credentials / proxy (hidden input)
    Configure {
        #[arg(long)]
        twitter_cookies: bool,
        #[arg(long)]
        proxy: bool,
        #[arg(long)]
        groq_key: bool,
        #[arg(long)]
        xhs_cookies: bool,
    },
    /// Transcribe a podcast / episode URL to text
    Transcribe {
        /// Episode or page URL
        url: String,
    },
}

impl Cli {
    pub fn execute(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Channels => {
                println!("{:<14} {:<16} {:<16} {:<8} region", "ID", "DISPLAY", "BACKEND", "AUTH");
                for c in channels::all() {
                    println!(
                        "{:<14} {:<16} {:<16} {:<8} {}",
                        c.id,
                        c.display,
                        c.preferred_backend,
                        format!("{:?}", c.auth),
                        c.region
                    );
                }
                Ok(())
            }
            Commands::Install {
                env,
                system,
                dry_run,
                channels,
            } => install::run(&env, system, dry_run, channels.as_deref()),
            Commands::Doctor { json } => doctor::run(json),
            Commands::Watch => {
                doctor::run(false)?;
                println!("(watch) update check not wired in this build; run `agent-reach doctor`.");
                Ok(())
            }
            Commands::Configure {
                twitter_cookies,
                proxy,
                groq_key,
                xhs_cookies,
            } => config::configure(twitter_cookies, proxy, groq_key, xhs_cookies),
            Commands::Transcribe { url } => transcribe::run(&url),
        }
    }
}
