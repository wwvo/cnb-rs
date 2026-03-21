---
title: cnb-rs mission create
---

# cnb-rs mission create

```
cnb-rs mission create --group <group> --name <name> [options]
```

创建任务集。

在指定组织下创建新的任务集，可同时关联多个仓库并设置可见性。

## 选项

- `-g, --group <GROUP>`: 组织路径（必填）
- `-n, --name <NAME>`: 任务集名称（必填）
- `-d, --description <DESC>`: 描述
- `--repos <REPOS>`: 关联仓库列表（逗号分隔）
- `--visibility <LEVEL>`: 可见性：`public`、`private`、`secret`

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 创建基本任务集
$ cnb-rs mission create --group my-org --name "Sprint 2025-Q2"
✓ 任务集已创建

# 创建并关联仓库
$ cnb-rs mission create --group my-org --name "Bug Tracking" \
    --description "缺陷跟踪" \
    --repos "my-org/repo-a,my-org/repo-b" \
    --visibility public
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission list](/mission/list)
- [cnb-rs mission delete](/mission/delete)
