/// Podcast / episode transcription via Groq Whisper, with an OpenAI fallback
/// path documented for the production build (ported from `transcribe.py`).
pub fn run(url: &str) -> anyhow::Result<()> {
    let cfg = crate::config::Config::load()?;
    match cfg.groq_key {
        Some(_) => println!("transcribing {url} via Groq Whisper (stub)"),
        None => anyhow::bail!("groq_key not configured; run `agent-reach configure --groq-key`"),
    }
    Ok(())
}
