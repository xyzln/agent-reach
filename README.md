# agent-reach

> 一个 CLI、尽量零 API 费用，跨 18 个平台检索与读取内容。本实现新增国内主流平台（知乎 / 淘宝 / 京东）适配。

## 定位与设计原则

agent-reach 是一个 **router / selector / installer / doctor** 工具（编排器，而非封装层）。
装完后你（Agent）**直接调用上游工具**（`twitter-cli`、`yt-dlp`、`gh`、`opencli`、`mcporter`、`bili`、`boss` …），
本二进制只负责：选哪个后端、把工具装上、给工具做体检、把请求路由过去。

重构严格保留原架构四层：

| 层 | 参考实现 | 本实现 |
|---|---|---|
| 入口 | 参考实现入口 | `main.rs` + `cli.rs` |
| 真实探测 | 参考实现探测 | `doctor.rs` |
| 转录 | `transcribe.py`（Groq→OpenAI） | `transcribe.rs` |
| 渠道 | `channels/*.py` | `channels.rs`（统一注册表） |

## 架构

```
用户/Agent
   │ 自然语言指令
   ▼
agent-reach CLI  (router · selector · installer · doctor)
   │ 按 channel 选 active_backend
   ▼
上游工具层 (直接调用，非封装)
   ▼
平台 channel  (含知乎 / 淘宝 / 京东)
```

## 构建与运行

```bash
cargo build --release
cargo run -- channels          # 列出所有 channel
cargo run -- install --dry-run # 只读预览安装计划
cargo run -- doctor            # 真实探测各渠道状态
cargo run -- doctor --json     # JSON 输出（active_backend 感知）
cargo run -- configure --groq-key
cargo run -- transcribe <url>
```

> 本环境未安装 cargo，源码未经编译验证；请在有 Rust 工具链的环境 `cargo build` 后使用。

## 支持的 channel（共 18 个）

平台：web, youtube, github, rss, exa, twitter, reddit, bilibili, xiaohongshu,
xueqiu, xiaoyuzhou, linkedin, facebook, instagram, boss

**新增国内平台**：`zhihu`（知乎）、`taobao`（淘宝）、`jd`（京东）

各 channel 字段见 `src/channels.rs`：`id / display / region / preferred_backend /
upstream_tool / auth / zero_config / requires_proxy_cn / notes`。

## 安全边界（与原版一致）

- 绝不代登录：不索取账号密码、不替用户登录；登录动作留给用户肉眼确认。
- 目录隔离：配置仅存 `~/.agent-reach/`，不污染 agent 工作区，不碰 `~` 外系统目录。
- `--system` 需显式授权：默认只读；改系统级依赖/配置必须用户明确批准，绝不私自 sudo、不动防火墙。
- 认证边界清晰：不读浏览器 Cookie；仅保存用户显式提供的同域 Cookie / Key。

## 配套资源（命名一致，功能对应）

| 产物 | 路径 | 用途 |
|---|---|---|
| Rust crate | `agent-reach/` | 核心实现，包名 `agent-reach` |
| Codex skill | `skills/codex/SKILL.md` | 面向 Codex 的 skill 定义 |
| Claude Code skill | `.claude/skills/agent-reach/SKILL.md` | 面向 Claude Code 的 skill 定义 |
| WorkBuddy 专家团 | `workbuddy/agent-reach.team.json` + `.md` | 适用于 WorkBuddy 的专家团配置 |

四个产物统一以 **`agent-reach`** 命名，能力相互对应：同一套 channel 注册表、同一套 CLI 子命令、同一套安全边界。
