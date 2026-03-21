---
title: cnb-rs release view
---

# cnb-rs release view

```
cnb-rs release view <TAG> [options]
```

查看指定 Release 的详细信息，包括基本信息、描述内容和附件列表。

## 选项

- `<TAG>`: Tag 名称（必填）
- `-w, --web`: 在浏览器中打开 Release 页面

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 输出示例

```
v1.2.0 — v1.2.0
状态: Latest
作者: zhangsan
发布时间: 2025-01-15 10:30

## 更新内容
- 新增仓库管理功能
- 修复若干 Bug

附件:
NAME                          SIZE        DOWNLOADS
app-linux-amd64.tar.gz        15.2 MB     128
app-darwin-arm64.tar.gz       14.8 MB     96
checksums.txt                 256 B       45
```

## 示例

```bash
# 查看 Release 详情
$ cnb-rs release view v1.2.0

# 在浏览器中打开
$ cnb-rs release view v1.2.0 --web

# JSON 输出
$ cnb-rs release view v1.2.0 --json
```

## 另请参阅

- [cnb-rs release](/release/)
- [cnb-rs release latest](/release/latest)
