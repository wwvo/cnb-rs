# cnb user activities

```
cnb user activities [<username>] [options]
```

查看用户在指定时间段内的活动汇总。

## 参数

| 参数            | 缩写 | 说明                                   |
|-----------------|------|----------------------------------------|
| `[<username>]`  |      | 用户名（不指定则查看当前用户）         |
| `--date <date>` | `-d` | 查询日期（格式 yyyyMM 或 yyyyMMdd）    |

## 示例

```bash
# 查看当月活动汇总
cnb user activities

# 查看指定月份
cnb user activities --date 202412

# 查看指定日期
cnb user activities --date 20250115

# 查看其他用户
cnb user activities zhangsan --date 202501

cnb user activities --json
```
