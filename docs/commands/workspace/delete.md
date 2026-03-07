# cnb workspace delete

```
cnb workspace delete [options]
```

删除指定的云原生开发环境。

## 选项

- `-p, --pipeline-id <ID>`: 流水线 ID（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除指定工作区
$ cnb workspace delete -p abc123
```

## 另请参阅

- [cnb workspace](/workspace/)
- [cnb workspace closed-clean](/workspace/closed-clean)
