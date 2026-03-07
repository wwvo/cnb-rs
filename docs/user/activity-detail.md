# cnb user activity-detail

```
cnb user activity-detail [<username>] --type <type> --repo <repo> --date <date>
```

查看用户在指定仓库中某类活动的详细记录。

## 参数

| 参数             | 缩写 | 说明                                       |
|------------------|------|--------------------------------------------|
| `[<username>]`   |      | 用户名（不指定则查看当前用户）             |
| `--type <type>`  | `-t` | 活动类型（commit/pr/issue/review）         |
| `--repo <repo>`  | `-r` | 仓库路径                                   |
| `--date <date>`  | `-d` | 查询日期（格式 yyyyMM 或 yyyyMMdd）        |

## 示例

```bash
# 查看当前用户在指定仓库的提交详情
cnb user activity-detail --type commit --repo org/repo --date 202501

# 查看指定用户的 PR 活动
cnb user activity-detail zhangsan --type pr --repo org/repo --date 20250115

# JSON 输出
cnb user activity-detail --type review --repo org/repo --date 202501 --json
```
