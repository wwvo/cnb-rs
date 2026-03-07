# cnb browse

```
cnb browse [path]
```

在浏览器中打开当前仓库的页面。

## 参数

| 参数     | 说明                                         |
|----------|----------------------------------------------|
| `[path]` | 子路径（如 `-/issues`、`-/pulls`、`-/settings`） |

## 示例

```bash
# 打开仓库首页
cnb browse

# 打开 Issues 页面
cnb browse -/issues

# 打开 Pull Requests 页面
cnb browse -/pulls

# 打开仓库设置页面
cnb browse -/settings
```
