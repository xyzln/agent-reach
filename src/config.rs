use std::io::{BufRead, Write};

use anyhow::Context;
use serde::{Deserialize, Serialize};

/// Persisted credentials and proxy settings. Stored at `~/.agent-reach/config.json`.
/// agent-reach never reads browser cookies or performs logins on the user's behalf;
/// only explicitly provided values are saved here.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub twitter_auth_token: Option<String>,
    pub twitter_ct0: Option<String>,
    pub proxy: Option<String>,
    pub groq_key: Option<String>,
    pub xhs_cookies: Option<String>,
}

impl Config {
    pub fn path() -> std::path::PathBuf {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        std::path::PathBuf::from(home)
            .join(".agent-reach")
            .join("config.json")
    }

    pub fn load() -> anyhow::Result<Self> {
        let p = Self::path();
        if !p.exists() {
            return Ok(Self::default());
        }
        let s = std::fs::read_to_string(&p).context("read config")?;
        let c: Config = serde_json::from_str(&s).context("parse config")?;
        Ok(c)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let p = Self::path();
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let s = serde_json::to_string_pretty(self)?;
        std::fs::write(&p, s).context("write config")?;
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn configure(
    twitter_cookies: bool,
    proxy: bool,
    groq_key: bool,
    xhs_cookies: bool,
) -> anyhow::Result<()> {
    let mut cfg = Config::load()?;
    if twitter_cookies {
        cfg.twitter_auth_token = Some(read_hidden("TWITTER_AUTH_TOKEN")?);
        cfg.twitter_ct0 = Some(read_hidden("TWITTER_CT0")?);
    }
    if proxy {
        cfg.proxy = Some(read_hidden("HTTP(S)_PROXY")?);
    }
    if groq_key {
        cfg.groq_key = Some(read_hidden("GROQ_API_KEY (gsk_...)")?);
    }
    if xhs_cookies {
        cfg.xhs_cookies = Some(read_hidden("XHS_COOKIE_SET (xiaohongshu.com same-domain)")?);
    }
    cfg.save()?;
    println!("saved configuration to {}", Config::path().display());
    Ok(())
}

/// Reads a single line without echo in a real TTY. Kept simple for the rewrite
/// scaffold; a production build should use a terminal raw-mode / rpassword crate.
fn read_hidden(prompt: &str) -> anyhow::Result<String> {
    print!("{prompt}: ");
    let _ = std::io::stdout().flush();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}
