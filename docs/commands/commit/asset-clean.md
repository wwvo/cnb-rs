# cnb commit asset-clean

```
cnb commit asset-clean [flags]
```

按策略批量清理 Commit 附件。

必须指定至少一种删除策略。执行前会列出所有待删除的附件，需确认后才会执行删除（使用 `-y` 跳过确认）。

## 选项

- `--keep-days <N>`: 删除 N 天前提交的 Commit 的附件
- `--keep-num <N>`: 保留最近 N 个有附件的 Commit，删除更早的附件
- `-y, --yes`: 跳过确认提示，直接执行删除

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除 90 天前的 Commit 附件
$ cnb commit asset-clean --keep-days 90

# 保留最近 10 个有附件的 Commit
$ cnb commit asset-clean --keep-num 10

# 跳过确认
$ cnb commit asset-clean --keep-days 30 -y
```

## 另请参阅

- [cnb commit](/commit/)
