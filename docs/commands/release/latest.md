# cnb-rs release latest

```
cnb-rs release latest
```

查看仓库最新的 Release 版本，显示基本信息和附件摘要。

## 选项

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 输出示例

```
v1.2.0 — v1.2.0  [Latest]
发布时间: 2025-01-15 10:30
作者: zhangsan
附件: 3 个文件, 共 30.3 MB
```

## 示例

```bash
# 查看最新 Release
$ cnb-rs release latest

# JSON 输出
$ cnb-rs release latest --json
```

## API

| 方法 | 端点                        |
| ---- | --------------------------- |
| GET  | `/{repo}/-/releases/latest` |

## 另请参阅

- [cnb-rs release](/release/)
- [cnb-rs release view](/release/view)
