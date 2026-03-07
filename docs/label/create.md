# cnb label create

```
cnb label create -n <name> -c <color> [-d <description>]
```

创建仓库标签。

## 参数

| 参数                   | 缩写 | 必填 | 说明                     |
|------------------------|------|------|--------------------------|
| `--name <name>`        | `-n` | 是   | 标签名称                 |
| `--color <hex>`        | `-c` | 是   | 颜色（十六进制，不含 #） |
| `--description <desc>` | `-d` | 否   | 标签描述                 |

## 示例

```bash
cnb label create -n "bug" -c "d73a4a" -d "Bug 修复"
# ✓ 标签 bug 已创建

cnb label create -n "feature" -c "0075ca"
# ✓ 标签 feature 已创建
```

## 另请参阅

- [cnb label list](/label/list)
- [cnb label update](/label/update)
