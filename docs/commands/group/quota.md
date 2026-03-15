# cnb-rs group quota

```
cnb-rs group quota <GROUP>
```

查看组织的特权额度信息，包括算力、存储等资源配额。

## 选项

- `<GROUP>`: 组织路径（必填）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看额度
$ cnb-rs group quota my-org

# JSON 格式输出
$ cnb-rs group quota my-org --json
```

## 另请参阅

- [cnb-rs group](/group/)
- [cnb-rs group view](/group/view)
