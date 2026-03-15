# cnb-rs badge get

```
cnb-rs badge get <sha> <badge> [options]
```

获取指定徽章数据。

默认输出 SVG 格式，可通过 `--json` 全局选项输出 JSON 数据。
`<sha>` 参数支持传入 `latest` 来获取最新的徽章。

## 选项

- `<sha>`: Commit SHA 或 `latest`（必填）
- `<badge>`: 徽章名称，如 `ci/status/push`（必填）
- `-b, --branch <BRANCH>`: 指定分支
- `--svg`: 输出 SVG 格式（默认）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 获取最新 CI 状态徽章 JSON 数据
$ cnb-rs badge get latest ci/status/push --json

# 获取指定 commit 的徽章 SVG
$ cnb-rs badge get 89d48c07 ci/status/push --svg > badge.svg

# 指定分支
$ cnb-rs badge get latest ci/status/push --branch main --json
```

## 另请参阅

- [cnb-rs badge](/badge/)
- [cnb-rs badge list](/badge/list)
- [cnb-rs badge upload](/badge/upload)
