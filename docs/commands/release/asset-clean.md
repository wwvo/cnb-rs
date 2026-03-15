# cnb-rs release asset-clean

```
cnb-rs release asset-clean [flags]
```

按策略批量清理 Release 附件。

必须指定至少一种删除策略。执行前会列出所有待删除的附件，需确认后才会执行删除（使用 `-y` 跳过确认）。

## 选项

- `--tag-name-prefix <PREFIX>`: 删除 Tag 名称以指定前缀开头的 Release 的所有附件
- `--release-name-prefix <PREFIX>`: 删除 Release 名称以指定前缀开头的 Release 的所有附件
- `--keep-days <N>`: 删除 N 天前发布的 Release 的附件
- `--keep-num <N>`: 保留最近 N 个有附件的 Release，删除更早的附件
- `-y, --yes`: 跳过确认提示，直接执行删除

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除 Tag 以 "nightly-" 开头的附件
$ cnb-rs release asset-clean --tag-name-prefix "nightly-"

# 保留最近 5 个有附件的 Release
$ cnb-rs release asset-clean --keep-num 5

# 删除 30 天前的附件，跳过确认
$ cnb-rs release asset-clean --keep-days 30 -y
```

## 另请参阅

- [cnb-rs release](/release/)
