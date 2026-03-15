#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=.cnb/scripts/release-common.sh
source "${SCRIPT_DIR}/release-common.sh"

require_cmd git

git fetch --tags --force origin main

current_version="$(workspace_version)"
tag_name="$(version_tag "${current_version}")"
previous_version="$(workspace_version_at_ref HEAD^ 2>/dev/null || true)"

if git rev-parse -q --verify "refs/tags/${tag_name}" >/dev/null 2>&1; then
  cnb_log "标签 ${tag_name} 已存在，跳过自动打 tag"
  exit 0
fi

changed_files="$(git diff-tree --no-commit-id --name-only -r HEAD)"

if ! grep -qx 'Cargo.toml' <<<"${changed_files}"; then
  cnb_log "当前提交未修改 Cargo.toml，跳过自动打 tag"
  exit 0
fi

if ! grep -qx 'CHANGELOG.md' <<<"${changed_files}"; then
  cnb_log "当前提交未修改 CHANGELOG.md，跳过自动打 tag"
  exit 0
fi

if [[ -n "${previous_version}" && "${previous_version}" == "${current_version}" ]]; then
  cnb_log "workspace.package.version 未变化，跳过自动打 tag"
  exit 0
fi

create_tag "${tag_name}" "$(current_head_sha)" "Release ${tag_name}"

cnb_log "已创建标签 ${tag_name}"
