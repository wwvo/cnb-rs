# cnb-rs label issue-set

```
cnb-rs label issue-set <number> -l <labels>
```

替换指定 Issue 的所有标签（现有标签将被完全替换）。

## 选项

- `<number>`: Issue 编号（必填）
- `-l, --labels <LABELS>`: 新标签列表，逗号分隔或多次指定（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs label issue-set 42 -l "wontfix"
✓ 已替换 Issue #42 的标签为: wontfix
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label issue-list](/label/issue-list)
- [cnb-rs label issue-clear](/label/issue-clear)
