# cnb-rs build status

```
cnb-rs build status <sn>
```

查询指定构建的状态和流水线状态。

## 选项

- `<sn>`: 构建号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs build status cnb-1qa-1i3f5ecau
构建号:   cnb-1qa-1i3f5ecau
状态:     success
流水线状态:
  pipeline-1: success
  pipeline-2: success

# JSON 格式输出
$ cnb-rs build status cnb-1qa-1i3f5ecau --json
```

## 另请参阅

- [cnb-rs build](/build/)
- [cnb-rs build list](/build/list)
- [cnb-rs build stage](/build/stage)
