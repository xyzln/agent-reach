---
name: agent-reach
description: 跨平台内容触达路由工具，让一个 CLI 即可检索与分析 X、Reddit、YouTube、B站、小红书、知乎、淘宝、京东 等 18 个平台的内容，零 API 费用。
slug: agent-reach-d85f7348
displayName: agent-reach 跨平台内容触达
version: 1.0.0
summary: 一个 Rust 实现的跨平台内容触达路由（router/selector/installer/doctor），支持 18 个国内外平台，零 API 费用。
tags: [content-reach, research, rust, cli, cross-platform]
license: MIT
---

# agent-reach

agent-reach 是一个 **router / selector / installer / doctor**（路由器 / 选择器 / 安装器 / 诊断器），而不是对上游工具的封装：安装后，它会直接调用上游工具（`twitter-cli`、`yt-dlp`、`gh`、`opencli`、`mcporter`、`bili`、`boss` 等）。`agent-reach` 本身只负责挑选后端、预置环境、健康检查并路由请求。

## 安装（Rust）

```bash
cargo install --git https://github.com/xyzln/agent-reach
# 或本地构建：
cargo build --release
```

## 命令

| 命令 | 作用 |
| --- | --- |
| `agent-reach channels` | 列出所有支持的平台 / 频道 |
| `agent-reach install --dry-run` | 只读预览安装计划（默认） |
| `agent-reach install --system` | 安装 / 配置核心外部工具（需显式授权） |
| `agent-reach doctor [--json]` | 真实探测各频道健康状态；`--json` 输出机器可读结果 |
| `agent-reach watch` | 健康检查 + 更新检查（用于定时任务） |
| `agent-reach configure --groq-key` | 通过隐藏输入保存凭据 |
| `agent-reach transcribe <url>` | 将播客 / 节目转写为文本 |

## 支持的平台

web、youtube、github、rss、exa、twitter、reddit、bilibili、xiaohongshu、xueqiu、xiaoyuzhou、linkedin、facebook、instagram、boss。

本版本新增国内平台支持：**知乎（zhihu）**、**淘宝（taobao）**、**京东（jd）**——各频道的鉴权 / 地区 / 后端实现见 `src/channels.rs`。

## 使用流程

1. 运行 `agent-reach channels` 查看可触达的平台。
2. 运行 `agent-reach doctor --json` 了解每个频道的 `active_backend`。
3. 做调研任务时，让 agent 在多个平台并行检索，返回真实用户讨论而非仅媒体报道。例如：

   > 使用 agent-reach 搜一下 [产品/话题] 最近一个月在 Reddit、X、YouTube、B站、小红书、知乎 上的讨论，挑反复出现的好评和槽点，别只给媒体报道。

## 安全

- 绝不自动登录，登录交由用户完成。
- `--system` 需显式授权；默认只读。
- 凭据仅保存在 `~/.agent-reach/config.json`，绝不读取浏览器 Cookie。
