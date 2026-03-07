# cnb workspace list

```
cnb workspace list [options]
```

列出当前用户的云原生工作区。

## 选项

- `-s, --status <STATUS>`: 状态过滤：`running`（运行中）、`closed`（已关闭）、`all`（全部）
- `-r, --repo <REPO>`: 按仓库路径过滤
- `-b, --branch <BRANCH>`: 按分支过滤

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出所有工作区
$ cnb workspace list

# 仅列出运行中的工作区
$ cnb workspace list -s running

# 按仓库过滤
$ cnb workspace list -r my-org/repo1

# JSON 格式输出
$ cnb workspace list --json
```

## 另请参阅

- [cnb workspace](/workspace/)
- [cnb workspace start](/workspace/start)
