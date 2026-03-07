#!/bin/bash
# 通用多平台构建脚本（使用预配置的 rust-cross 镜像）
# 用法: build-target.sh <target1> [target2] ...
set -euo pipefail

VERSION="${CNB_BRANCH}"
TARGETS=("$@")

if [ ${#TARGETS[@]} -eq 0 ]; then
  echo "错误: 请指定至少一个构建 target"
  exit 1
fi

echo "=== 构建版本: ${VERSION} ==="
echo "=== Targets: ${TARGETS[*]} ==="

# ========== 构建 ==========
for target in "${TARGETS[@]}"; do
  echo "--- 构建 ${target} ---"
  case "$target" in
    x86_64-unknown-linux-gnu|x86_64-pc-windows-gnu)
      cargo build --release --target "$target"
      ;;
    *)
      cargo zigbuild --release --target "$target"
      ;;
  esac
done

# ========== 打包 ==========
echo "--- 打包构建产物 ---"
for target in "${TARGETS[@]}"; do
  dir="cnb-${VERSION}-${target}"
  mkdir -p "${dir}"

  if [[ "$target" == *-windows-* ]]; then
    cp "target/${target}/release/cnb.exe" "${dir}/"
    cp "target/${target}/release/git-cnb.exe" "${dir}/"
    zip -r "${dir}.zip" "${dir}"
    echo "  完成: ${dir}.zip"
  else
    cp "target/${target}/release/cnb" "${dir}/"
    cp "target/${target}/release/git-cnb" "${dir}/"
    tar czf "${dir}.tar.gz" "${dir}"
    echo "  完成: ${dir}.tar.gz"
  fi
done

echo "=== 构建完成 ==="
ls -lh cnb-*.tar.gz cnb-*.zip 2>/dev/null || true
