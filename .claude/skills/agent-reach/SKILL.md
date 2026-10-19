---
name: agent-reach
description: Cross-platform content reach for AI agents. Use when the user wants to
  search, read, or analyze content across X, Reddit, YouTube, Bilibili, 小红书,
  知乎, 淘宝, 京东, GitHub, RSS and more from a single CLI with zero API fees.
allowed-tools: Bash(agent-reach:*), Bash(cargo:*), Bash(which:*), Bash(where:*)
---

# agent-reach

agent-reach is a
**router / selector / installer / doctor**, NOT a wrapper: after install you call
upstream tools (`twitter-cli`, `yt-dlp`, `gh`, `opencli`, `mcporter`, `bili`,
`boss`, ...) directly. The `agent-reach` binary only picks the backend, provisions
it, health-checks it, and routes the request.

## Install (Rust)

```bash
cargo install --git https://github.com/xyzln/agent-reach
# or build locally:
cargo build --release
```

## Commands

| Command | What it does |
|---|---|
| `agent-reach channels` | List all supported platforms/channels |
| `agent-reach install --dry-run` | Read-only preview of install plan (default) |
| `agent-reach install --system` | Install/configure core external tools (explicit approval) |
| `agent-reach doctor [--json]` | Real-probe channel health; `--json` for machine output |
| `agent-reach watch` | Health + update check (for scheduled tasks) |
| `agent-reach configure --groq-key` | Save a credential via hidden input |
| `agent-reach transcribe <url>` | Transcribe a podcast/episode to text |

## Supported channels

平台：web, youtube, github, rss, exa, twitter, reddit, bilibili,
xiaohongshu, xueqiu, xiaoyuzhou, linkedin, facebook, instagram, boss.

China platforms added in this build: **zhihu (知乎)**, **taobao (淘宝)**,
**jd (京东)** — see `src/channels.rs` for auth/region/backend per channel.

## Workflow

1. Run `agent-reach channels` to see what is reachable.
2. Run `agent-reach doctor --json` to learn `active_backend` per channel.
3. For a research task, fan out across platforms and return real user discussion
   (not just media coverage). Example:

   > 使用 agent-reach 搜一下 [产品/话题] 最近一个月在 Reddit、X、YouTube、B站、小红书、知乎 上的讨论，挑反复出现的好评和槽点，别只给媒体报道。

## Safety

- Never auto-login; leave login to the user.
- `--system` requires explicit approval; default is read-only.
- Credentials live only in `~/.agent-reach/config.json`; browser cookies are never read.
