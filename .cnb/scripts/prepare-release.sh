#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=.cnb/scripts/release-common.sh
source "${SCRIPT_DIR}/release-common.sh"

base_branch="main"
dry_run=0
release_branch=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-branch)
      shift
      base_branch="${1:-}"
      ;;
    --release-branch)
      shift
      release_branch="${1:-}"
      ;;
    --dry-run)
      dry_run=1
      ;;
    *)
      cnb_err "未知参数: $1"
      ;;
  esac
  shift
done

require_cmd git
require_cmd cargo

git fetch origin "${base_branch}" --tags --force

current_version="$(workspace_version)"
[[ -n "${current_version}" ]] || cnb_err "无法读取当前 workspace version"

commit_range="$(version_range "${current_version}")"
commit_count="$(commit_count_in_range "${commit_range}")"

if [[ "${commit_count}" -eq 0 ]]; then
  cnb_err "当前版本 ${current_version} 自上次发布以来没有新提交"
fi

bump_level="$(detect_bump_level "${commit_range}")"
next_version="$(bump_version "${current_version}" "${bump_level}")"
release_branch="${release_branch:-release/v${next_version}}"
commit_title="🔧 chore(release): 准备 v${next_version} 发版"
pr_body=$'## 自动发版准备\n\n- 自动推导下一个版本号并更新 `Cargo.toml`\n- 自动同步 `Cargo.lock` 中的 workspace crate 版本\n- 自动重建项目级 `CHANGELOG.md`\n- 合并到 `main` 后将由流水线自动创建对应 tag\n'

if git ls-remote --exit-code --heads origin "${release_branch}" >/dev/null 2>&1; then
  cnb_err "远端分支 ${release_branch} 已存在，请先处理现有 release PR"
fi

cnb_log "当前版本: ${current_version}"
cnb_log "升级级别: ${bump_level}"
cnb_log "目标版本: ${next_version}"
cnb_log "目标分支: ${release_branch}"

if [[ "${dry_run}" -eq 1 ]]; then
  exit 0
fi

git checkout -B "${release_branch}" "origin/${base_branch}"

set_workspace_version "${next_version}"
cargo metadata --format-version 1 --no-deps >/dev/null
git cliff --unreleased --tag "v${next_version}" --prepend CHANGELOG.md

git add Cargo.toml Cargo.lock CHANGELOG.md
git diff --cached --quiet && cnb_err "没有检测到需要提交的 release 变更"

git config user.name "CNB Bot"
git config user.email "bot@cnb.cool"

git commit \
  -m "${commit_title}" \
  -m "- 根据 ${current_version} 以来的提交自动推导出下一个版本号" \
  -m "- 更新 workspace version、同步 Cargo.lock 并重建项目级 CHANGELOG.md" \
  -m "- 为 main 合并后的自动打 tag 流程准备 release PR" \
  -m "Refs: #48"

git push origin "HEAD:${release_branch}"

create_pull_request \
  "${base_branch}" \
  "${release_branch}" \
  "${commit_title}" \
  "${pr_body}"
