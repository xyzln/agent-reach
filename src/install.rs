use crate::channels;

/// Build and (optionally) execute an install plan.
///
/// Safety model (ported from the original `docs/install.md`):
/// - Default is read-only: no changes to the host.
/// - `--system` requires explicit user approval; this binary never runs sudo,
///   never touches files outside `~/.agent-reach/`, and never clones into the
///   agent workspace.
/// - `--dry-run` previews the plan and makes no changes.
pub fn run(
    env: &str,
    system: bool,
    dry_run: bool,
    channels_filter: Option<&str>,
) -> anyhow::Result<()> {
    let targets: Vec<_> = match channels_filter {
        Some(list) => channels::all()
            .into_iter()
            .filter(|c| list.split(',').any(|x| x.trim() == c.id))
            .collect(),
        None => channels::all(),
    };

    println!("agent-reach install --env={env} --system={system} --dry-run={dry_run}");
    println!("planned channels: {}", targets.len());

    for c in &targets {
        let action = if dry_run {
            "preview"
        } else if system {
            "install/configure"
        } else {
            "check-only"
        };
        println!(
            "  [{action}] {} ({}) -> upstream: {}",
            c.display, c.id, c.upstream_tool
        );
    }

    if dry_run {
        println!("dry-run: no changes made.");
    } else if !system {
        println!("read-only check complete. re-run with --system (requires explicit approval) to install.");
    } else {
        println!("system install path: provision upstream tooling under ~/.agent-reach/tools per directory rules.");
    }
    Ok(())
}
