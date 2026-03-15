# cnb-rs build stage

```
cnb-rs build stage <sn> <pipeline_id> <stage_id>
```

查看指定构建的 Stage 详情，包括状态、耗时和日志内容。

## 选项

- `<sn>`: 构建号（必填）
- `<pipeline_id>`: 流水线 ID（必填）
- `<stage_id>`: Stage ID（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs build stage cnb-1qa-1i3f5ecau pipeline-1 stage-build
Stage:    stage-build
状态:     success
耗时:     45s
日志:
  Step 1/5: Pulling image...
  Step 2/5: Building...
```

## 另请参阅

- [cnb-rs build](/build/)
- [cnb-rs build status](/build/status)
- [cnb-rs build download-log](/build/download-log)
