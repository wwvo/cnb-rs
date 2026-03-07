# cnb badge

```
cnb badge <subcommand>
```

管理仓库的 CI/CD 徽章。

徽章是 CI/CD 流水线生成的状态标记，通常以 SVG 格式嵌入 README 中，显示构建状态、代码质量等信息。
CNB 平台支持内置徽章（如 `ci/status/push`）和用户自定义徽章。

## 可用命令

- [cnb badge get](/badge/get) — 获取指定徽章数据
- [cnb badge list](/badge/list) — 列出仓库所有徽章
- [cnb badge upload](/badge/upload) — 上传自定义徽章

## 示例

```bash
# 列出仓库徽章
$ cnb badge list

# 获取最新 CI 状态徽章
$ cnb badge get latest ci/status/push --json

# 上传自定义徽章
$ cnb badge upload --key security/tca --sha abc12345 --message "A+"
```

## 另请参阅

- [cnb build](/build/)
- [cnb repo](/repo/)
