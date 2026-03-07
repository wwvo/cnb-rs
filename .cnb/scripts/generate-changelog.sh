#!/bin/bash
# 生成本次版本的 CHANGELOG（供 Release 描述使用）
# git-cliff 已预装在 rust-cross 镜像中
set -euo pipefail

# 确保 tag 信息完整
git fetch --tags --force 2>/dev/null || true

echo "--- 生成 CHANGELOG ---"
git cliff --latest --strip header -o LATEST_CHANGELOG.md

echo "=== CHANGELOG 内容 ==="
cat LATEST_CHANGELOG.md
