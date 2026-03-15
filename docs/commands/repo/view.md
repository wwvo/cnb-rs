# cnb-rs repo view

```
cnb-rs repo view [<repo>] [flags]
```

查看仓库详情。

不指定仓库路径时，使用当前 Git 目录对应的仓库（从 remote URL 推断）。

以表格形式展示仓库的名称、描述、Star/Fork/Issue/PR 数量、可见性、许可证、语言、URL、创建时间和更新时间。

## 选项

- `[<repo>]`: 仓库路径（如 `org/repo`），可选
- `-w, --web`: 在浏览器中打开仓库页面

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当前仓库
$ cnb-rs repo view

# 查看指定仓库
$ cnb-rs repo view org/repo

# 在浏览器中打开
$ cnb-rs repo view --web

# JSON 输出
$ cnb-rs repo view --json
```

## API

| 步骤     | API                          | 方法 | 说明         |
| -------- | ---------------------------- | ---- | ------------ |
| 获取仓库 | `${CNB_API_ENDPOINT}/{repo}` | GET  | 获取仓库详情 |

**API 详情**（OpenAPI：[`GetByID`](https://api.cnb.cool/#/operations/GetByID)）：

- **权限要求：** `repo-basic-info:r`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo list](/repo/list)
- [cnb-rs info](/info)
