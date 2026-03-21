---
title: cnb-rs knowledge info
---

# cnb-rs knowledge info

```
cnb-rs knowledge info
```

获取当前仓库的知识库信息。

## 选项

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 输出字段

输出以下字段：

- `ID` — 知识库 ID
- `LastCommitSha` — 最后处理的 Commit SHA
- `Model` — 使用的 Embedding 模型名称
- `Include` — 包含的文件模式
- `Exclude` — 排除的文件模式

## 输出示例

```
ID                   kb-12345
LastCommitSha        abc123def456
Model                text-embedding-v2
Include              *.md,*.rs
Exclude              test/*
```

## 另请参阅

- [cnb-rs knowledge](/knowledge/)
