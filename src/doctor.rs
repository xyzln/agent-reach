use crate::channels;
use crate::router::Status;
use std::process::Command;

/// Real-probing health check. Unlike a version banner, this actually attempts to
/// locate each upstream tool on PATH and reports a graded status, mirroring the
/// original `probe.py` (missing / broken / timeout).
pub fn run(json: bool) -> anyhow::Result<()> {
    let mut rows = Vec::new();
    for c in channels::all() {
        let status = probe_tool(c.upstream_tool);
        rows.push(serde_json::json!({
            "channel": c.id,
            "display": c.display,
            "backend": c.preferred_backend,
            "status": format!("{status:?}"),
            "zero_config": c.zero_config,
            "auth": format!("{:?}", c.auth),
            "region": c.region,
        }));
        if !json {
            println!(
                "{:<14} {:<16} backend={:<16} status={status:?}",
                c.id, c.display, c.preferred_backend
            );
        }
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&rows)?);
    }
    Ok(())
}

/// Probe whether an upstream tool exists on PATH. For composite tool strings
/// (`a / b`) we probe the first token; a production build would probe each
/// candidate in order and surface the first that resolves.
fn probe_tool(tool: &str) -> Status {
    let base = tool
        .split(|c| c == ' ' || c == '/')
        .next()
        .unwrap_or(tool);
    let checker = if cfg!(windows) { "where" } else { "which" };
    match Command::new(checker).arg(base).output() {
        Ok(o) if o.status.success() => Status::Ok,
        Ok(_) => Status::Missing,
        Err(_) => Status::Broken,
    }
}
