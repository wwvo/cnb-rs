# cnb-rs workspace delete

```
cnb-rs workspace delete [options]
```

删除指定的云原生开发环境。

## 选项

- `-p, --pipeline-id <ID>`: 流水线 ID（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除指定工作区
$ cnb-rs workspace delete -p abc123
```

## 另请参阅

- [cnb-rs workspace](/workspace/)
- [cnb-rs workspace closed-clean](/workspace/closed-clean)
