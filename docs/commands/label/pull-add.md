# cnb-rs label pull-add

```
cnb-rs label pull-add <number> -l <labels>
```

为指定 Pull Request 添加标签。

## 选项

- `<number>`: Pull 编号（必填）
- `-l, --labels <LABELS>`: 标签名称，逗号分隔或多次指定（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs label pull-add 10 -l "approved" -l "ready-to-merge"
✓ 已为 Pull #10 添加标签: approved, ready-to-merge
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label pull-list](/label/pull-list)
- [cnb-rs label pull-remove](/label/pull-remove)
