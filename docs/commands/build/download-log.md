# cnb-rs build download-log

```
cnb-rs build download-log <pipeline_id> [-o <path>]
```

下载指定流水线的 Runner 日志。

不指定输出路径时默认输出到标准输出。

## 选项

- `<pipeline_id>`: 流水线 ID（必填）
- `-o, --output <PATH>`: 输出文件路径（不指定则输出到 stdout）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 输出到终端
$ cnb-rs build download-log pipeline-1

# 保存到文件
$ cnb-rs build download-log pipeline-1 -o ./build.log
✓ 日志已保存到 ./build.log
```

## 另请参阅

- [cnb-rs build](/build/)
- [cnb-rs build stage](/build/stage)
- [cnb-rs build delete-log](/build/delete-log)
