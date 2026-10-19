//! agent-reach
//!
//! A cross-platform content reach router. It acts as a **router / selector /
//! installer / doctor** for upstream platform tools. After install you call the
//! upstream tools (`twitter-cli`, `yt-dlp`, `gh`, `opencli`, `mcporter`, `bili`,
//! `boss`, ...); this binary only chooses the backend, provisions it,
//! health-checks it, and routes the request.
//!
//! Rust's standard strings are UTF-8, so the Windows-console emoji crash that
//! the original `cli.py` worked around with an encoding wrapper does not occur
//! here.

mod channels;
mod cli;
mod config;
mod doctor;
mod install;
mod router;
mod transcribe;

use clap::Parser;
use cli::Cli;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
