# cnb-rs label issue-add

```
cnb-rs label issue-add <number> -l <labels>
```

为指定 Issue 添加标签。

## 选项

- `<number>`: Issue 编号（必填）
- `-l, --labels <LABELS>`: 标签名称，逗号分隔或多次指定（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 多次指定标签
$ cnb-rs label issue-add 42 -l "bug" -l "urgent"
✓ 已为 Issue #42 添加标签: bug, urgent

# 逗号分隔
$ cnb-rs label issue-add 42 -l "bug,urgent"
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label issue-list](/label/issue-list)
- [cnb-rs label issue-remove](/label/issue-remove)
