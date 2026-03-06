# cnb repo events

```
cnb repo events [<repo>] [flags]
```

获取仓库动态内容，支持按天或按小时查询。

不指定仓库路径时，使用当前 Git 目录对应的仓库。

## 选项

`<repo>`
: 仓库路径（如 `org/repo`），可选

`--date <DATE>`, `-d`
: 查询日期（格式：`YYYY-MM-DD`，默认今天）

`--hour <HOUR>`
: 指定小时（0-23）

**继承的全局选项：**

`--json`
: 以 JSON 格式输出

`--domain <DOMAIN>`
: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看今天的仓库动态
$ cnb repo events

# 查看指定日期
$ cnb repo events org/repo --date 2025-01-15

# 查看指定小时
$ cnb repo events --date 2025-01-15 --hour 14
```

## API

| 步骤         | API                                                  | 方法 | 说明         |
| ------------ | ---------------------------------------------------- | ---- | ------------ |
| 获取动态     | `${CNB_API_ENDPOINT}/events/{repo}/-/{date}`         | GET  | 仓库动态     |

**API 详情**（OpenAPI：[`GetEvents`](https://api.cnb.cool/#/operations/GetEvents)）：

## 另请参阅

- [cnb repo](/repo/)
- [cnb repo view](/repo/view)
