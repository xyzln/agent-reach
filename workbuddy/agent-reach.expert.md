# agent-reach 跨平台内容触达专家团（WorkBuddy 配置说明）

本文件与 `agent-reach.team.json` 配套，描述如何在 WorkBuddy 中落地该专家团。
所有产物统一以 **`agent-reach`** 命名，能力相互对应。

## 1. 角色定位

agent-reach 用于让 WorkBuddy 跨平台检索、读取与交叉验证内容，再把结果交给你判断。

- 内核：router / selector / installer / doctor（**非 wrapper**）
- 直接调用上游工具：`twitter-cli`、`yt-dlp`、`gh`、`opencli`、`mcporter`、`bili`、`boss`
- 配置隔离在 `~/.agent-reach/config.json`，绝不读浏览器 Cookie、绝不代登录

## 2. 触发词（与 team.json 中 `triggers` 对应）

`agent-reach`、`跨平台搜索`、`全网触达`、`知乎`、`淘宝`、`京东`、
`小红书`、`B站`、`雪球`、`Boss直聘`

## 3. 渠道清单（共 18 个，与原 Rust crate `src/channels.rs` 完全一致）

原平台：web, youtube, github, rss, exa, twitter, reddit, bilibili, xiaohongshu,
xueqiu, xiaoyuzhou, linkedin, facebook, instagram, boss

**新增国内平台**：zhihu（知乎）、taobao（淘宝）、jd（京东）

| channel | 平台 | 区域 | 首选后端 | 认证 | 零配置 | 备注 |
|---|---|---|---|---|---|---|
| zhihu | 知乎 | cn | jina | Cookie | 否 | Q&A + 专栏；登录提升配额 |
| taobao | 淘宝 | cn | openapi | ApiKey | 否 | 淘宝开放平台 或 headless+cookie |
| jd | 京东 | cn | jingdong | ApiKey | 否 | 京东联盟 / 京东 API 或 headless+cookie |

## 4. 标准工作流程（供 WorkBuddy 调用）

1. `agent-reach channels` —— 确认可达平台
2. `agent-reach doctor --json` —— 获取各渠道 `active_backend` 与状态
3. 按用户需求扇出检索，聚焦真实用户讨论而非媒体报道
4. 交叉验证后输出：反复出现的好评 / 槽点 + 代表性用户观点 + 趋势判断依据

## 5. 安全边界（必须遵循）

- 绝不代登录；登录/扫码动作留给用户肉眼确认（小红书、Boss、知乎等）
- `--system` 需用户显式授权；默认只读体检，绝不私自 sudo、不动防火墙
- CDP 调试端口仅绑 `127.0.0.1`，不得暴露到局域网/公网
- 仅保存用户显式提供的同域 Cookie / Key

## 6. 与其他产物的对应

| 产物 | 路径 | 关系 |
|---|---|---|
| Rust crate | `agent-reach/`（包名 `agent-reach`） | 核心实现 |
| Codex skill | `skills/codex/SKILL.md` | 同一套 channel / CLI / 安全边界 |
| Claude Code skill | `.claude/skills/agent-reach/SKILL.md` | 同上，allowed-tools 限定 `agent-reach`/`cargo` |
| WorkBuddy 专家团 | `workbuddy/agent-reach.team.json` | 本文件配套 JSON |

> 落地建议：将本目录作为 WorkBuddy 专家包加载（经 expert-manager 校验 schema），
> 或把 `skills/codex/SKILL.md` 与 `.claude/skills/agent-reach/SKILL.md` 分别拷到
> 对应平台的 skill 目录。三者命名一致、功能相互对应。
