---
title: cnb-rs repo security
---

# cnb-rs repo security

```
cnb-rs repo security [<repo>] [flags]
```

查看仓库的安全模块概览数据，包括代码敏感信息扫描、代码漏洞扫描和源码问题扫描的风险统计。

不指定仓库路径时，使用当前 Git 目录对应的仓库。

## 选项

- `[<repo>]`: 仓库路径（如 `org/repo`），可选
- `-t, --types <TYPES>`: 扫描类型过滤（逗号分隔：`code_sensitive`、`code_vulnerability`、`code_issue`）
- `--tab <TAB>`: 查询类型：`open`、`ignore`、`all`（默认：`all`）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当前仓库安全概览
$ cnb-rs repo security

# 查看指定仓库
$ cnb-rs repo security org/repo

# 仅查看漏洞扫描的开启中问题
$ cnb-rs repo security --types code_vulnerability --tab open

# JSON 输出
$ cnb-rs repo security --json
```

## API

| 步骤         | API                                              | 方法 | 说明     |
| ------------ | ------------------------------------------------ | ---- | -------- |
| 获取安全概览 | `${CNB_API_ENDPOINT}/{repo}/-/security/overview` | GET  | 安全概览 |

**API 详情**（OpenAPI：[`GetRepoSecurityOverview`](https://api.cnb.cool/#/operations/GetRepoSecurityOverview)）：

- **权限要求：** `repo-security:r`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo view](/repo/view)
