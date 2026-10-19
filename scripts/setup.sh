#!/usr/bin/env bash
# agent-reach — 环境引导脚本。
# 安全默认：仅安装 Rust 工具链并构建本 crate；不触碰系统文件、不执行 sudo。
# 各平台上游工具由运行时的 `agent-reach install` 负责供给。
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_DIR"

echo "[agent-reach] 检查 Rust 工具链..."
if ! command -v cargo >/dev/null 2>&1; then
  if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
  else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
    source "$HOME/.cargo/env"
  fi
fi

echo "[agent-reach] 构建中..."
cargo build --release

echo "[agent-reach] 完成。试一试： ./target/release/agent-reach channels"
