# cnb-rs workspace stop

```
cnb-rs workspace stop [options]
```

停止/关闭正在运行的云原生开发环境。

`--pipeline-id` 和 `--sn` 至少指定一个。

## 选项

- `-p, --pipeline-id <ID>`: 流水线 ID
- `--sn <SN>`: 流水线构建号

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 按 pipeline ID 停止
$ cnb-rs workspace stop -p abc123

# 按 SN 停止
$ cnb-rs workspace stop --sn 20250115-001

# JSON 格式输出
$ cnb-rs workspace stop -p abc123 --json
```

## 另请参阅

- [cnb-rs workspace](/workspace/)
- [cnb-rs workspace start](/workspace/start)
