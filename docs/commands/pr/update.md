# cnb-rs pr update

```
cnb-rs pr update <NUMBER> [flags]
```

更新指定 Pull Request 的标题、描述或状态。

至少需要指定 `--title`、`--body` 或 `--state` 中的一个。

## 选项

- `<NUMBER>`: PR 编号（必填）
- `-t, --title <TITLE>`: 修改标题
- `-b, --body <BODY>`: 修改描述
- `-s, --state <STATE>`: 修改状态：`open`、`closed`

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 修改标题
$ cnb-rs pr update 42 --title "新标题"

# 关闭 PR
$ cnb-rs pr update 42 --state closed

# 同时修改标题和描述
$ cnb-rs pr update 42 -t "新标题" -b "新描述"
```

## 另请参阅

- [cnb-rs pr](/pr/)
