---
title: cnb-rs workspace detail
---

# cnb-rs workspace detail

```
cnb-rs workspace detail --sn <SN> [options]
```

查看工作区详情，获取各种 IDE 的访问地址。

输出包含 WebIDE、VS Code、Cursor、CodeBuddy、SSH 等接入方式的链接。

## 选项

- `--sn <SN>`: 流水线构建号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看工作区详情
$ cnb-rs workspace detail --sn 20250115-001

# JSON 格式输出
$ cnb-rs workspace detail --sn 20250115-001 --json
```

## 另请参阅

- [cnb-rs workspace](/workspace/)
- [cnb-rs workspace start](/workspace/start)
