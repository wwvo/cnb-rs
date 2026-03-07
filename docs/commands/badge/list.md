# cnb badge list

```
cnb badge list
```

列出仓库所有徽章。

输出为表格格式，包含徽章名称、类型、状态和 URL。

## 选项

无子命令特有选项。

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb badge list
NAME                       TYPE      STATUS    URL
ci/status/push             ci        passing   https://cnb.cool/.../badge/...
ci/status/pull_request     ci        passing   https://cnb.cool/.../badge/...

# JSON 格式输出
$ cnb badge list --json
```

## 另请参阅

- [cnb badge](/badge/)
- [cnb badge get](/badge/get)
- [cnb badge upload](/badge/upload)
