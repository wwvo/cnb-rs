---
title: cnb-rs label issue-list
---

# cnb-rs label issue-list

```
cnb-rs label issue-list <number>
```

列出指定 Issue 的所有标签。

## 选项

- `<number>`: Issue 编号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs label issue-list 42
bug
enhancement

# JSON 格式输出
$ cnb-rs label issue-list 42 --json
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label issue-add](/label/issue-add)
- [cnb-rs label issue-remove](/label/issue-remove)
