# cnb group view

```
cnb group view <GROUP> [options]
```

查看指定组织的详细信息。

## 选项

- `<GROUP>`: 组织路径（必填）
- `-w, --web`: 在浏览器中打开组织页面

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看组织详情
$ cnb group view my-org

# 在浏览器中打开
$ cnb group view my-org --web

# JSON 格式输出
$ cnb group view my-org --json
```

## 另请参阅

- [cnb group](/group/)
- [cnb group list](/group/list)
