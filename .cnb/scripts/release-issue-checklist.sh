#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=.cnb/scripts/release-common.sh
source "${SCRIPT_DIR}/release-common.sh"

release_tag="${RELEASE_TAG:-}"
next_target_tag="${NEXT_TARGET_TAG:-}"

require_cmd cargo
require_cmd git
require_cmd curl
require_cnb_token

[[ -n "${release_tag}" ]] || cnb_err "缺少 RELEASE_TAG 输入"
[[ "${release_tag}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || \
  cnb_err "RELEASE_TAG 必须是稳定版 tag，例如 v0.4.6"

if [[ -n "${next_target_tag}" ]] && [[ ! "${next_target_tag}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  cnb_err "NEXT_TARGET_TAG 必须是稳定版 tag，例如 v0.4.7"
fi

repo="$(repo_slug)"
target_label="target:${release_tag}"
released_label="released:${release_tag}"
cnb_web_endpoint="${CNB_WEB_ENDPOINT:-https://cnb.cool}"
cnb_release_url="${cnb_web_endpoint}/${repo}/-/releases/tag/${release_tag}"
github_release_url="https://github.com/wwvo/cnb-cli-rs/releases/tag/${release_tag}"

cnb_log "检查 CNB Release 是否存在"
curl -fsS \
  -H "Accept: application/vnd.cnb.api+json" \
  -H "Authorization: Bearer ${CNB_TOKEN}" \
  "$(api_endpoint)/${repo}/-/releases/tags/${release_tag}" >/dev/null

cnb_log "检查 GitHub Release 是否存在"
curl -fsSIL "${github_release_url}" >/dev/null

cnb_log "release issue 收口清单"
cat <<EOF
版本标签规范：
- 目标版本标签: ${target_label}
- 已发布标签: ${released_label}
- 辅助保留标签: epic / tracking / keep-open

本次检查目标：
- 版本: ${release_tag}
- CNB Release: ${cnb_release_url}
- GitHub Release: ${github_release_url}

EOF

cnb_log "开放中的 ${target_label} issue："
cargo run --quiet -- --repo "${repo}" issue list -l "${target_label}" --sort -updated_at -L 100 || true

echo
cnb_log "已关闭但仍保留 ${target_label} 的 issue："
cargo run --quiet -- --repo "${repo}" issue list -s closed -l "${target_label}" --sort -updated_at -L 100 || true

echo
cat <<EOF
手工收口 checklist：
1. 确认 ${release_tag} 的 CNB / GitHub Release 页面和附件都已完成
2. 对开放中的 ${target_label} issue 逐个确认：
   - 带 epic / tracking / keep-open 的 issue 默认保持打开
   - 已随 ${release_tag} 交付的 issue：评论、添加 ${released_label}、移除 ${target_label}、再关闭
   - 未赶上的 issue：移除 ${target_label}，改挂到下一个目标版本

关闭评论模板：
已随 \`${release_tag}\` 正式发布，关闭。

- CNB Release: ${cnb_release_url}
- GitHub Release: ${github_release_url}

仅记录不关闭时的评论模板：
该 issue 关联的改动已包含在 \`${release_tag}\` 中，当前保持打开。

推荐命令：
- 首次使用前创建版本标签：
  cnb-rs --repo ${repo} label create -n "${target_label}" -c "1d76db" -d "计划在 ${release_tag} 交付"
  cnb-rs --repo ${repo} label create -n "${released_label}" -c "2da44e" -d "已随 ${release_tag} 发布"
- 添加已发布标签：
  cnb-rs --repo ${repo} label issue-add <NUMBER> -l "${released_label}"
- 移除当前目标标签：
  cnb-rs --repo ${repo} label issue-remove <NUMBER> "${target_label}"
- 添加关闭评论：
  cnb-rs --repo ${repo} issue comment <NUMBER> -c '已随 `${release_tag}` 正式发布，关闭。'
- 关闭 issue：
  cnb-rs --repo ${repo} issue close <NUMBER> -r completed

EOF

if [[ -n "${next_target_tag}" ]]; then
  cat <<EOF
未赶上当前版本时，推荐迁移命令：
- 添加下一个目标标签：
  cnb-rs --repo ${repo} label issue-add <NUMBER> -l "target:${next_target_tag}"

EOF
else
  cat <<EOF
未赶上当前版本时，请先移除 ${target_label}，再人工补上下一个目标版本标签，例如：
- cnb-rs --repo ${repo} label issue-add <NUMBER> -l "target:v0.4.7"

EOF
fi
