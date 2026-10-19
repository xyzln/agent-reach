# agent-reach（SkillHub 版）

跨平台内容触达路由工具，Rust 实现。一个 CLI 即可检索与分析 X、Reddit、YouTube、B站、小红书、知乎、淘宝、京东 等 18 个平台的内容，零 API 费用。

本目录是发布到 SkillHub 的技能包，仅含技能描述与文档；Rust 源码与构建请见 GitHub 仓库：<https://github.com/xyzln/agent-reach>。

## 快速开始

```bash
# 安装 Rust 命令行工具
cargo install --git https://github.com/xyzln/agent-reach

# 查看可触达平台
agent-reach channels

# 健康检查
agent-reach doctor --json
```

## 说明

- 工具定位为 router / selector / installer / doctor，不封装上游工具，只负责选后端、预置、体检、路由。
- 国内平台（知乎 / 淘宝 / 京东）已内置支持。
- 凭据仅存于 `~/.agent-reach/config.json`，不读取浏览器 Cookie，默认只读。

许可证：MIT
