# 输出示例模板

用于生成命令输出格式示例。

## JSON 输出

```bash
# JSON 格式输出
$ cnb-rs issue view 42 --json
{
  "number": 42,
  "title": "修复登录问题",
  "state": "open",
  "author": {
    "username": "zhangsan",
    "nickname": "张三"
  },
  "created_at": "2024-01-15T10:30:00Z"
}
```

## 表格输出

```bash
# 表格格式输出
$ cnb-rs issue list
| 编号 | 状态   | 标题           | 作者     |
| ---- | ------ | -------------- | -------- |
| #42  | open   | 修复登录问题   | zhangsan |
| #41  | closed | 添加用户管理   | lisi     |
| #40  | open   | 优化性能       | wangwu   |
```

## 简洁输出

```bash
# 简洁格式输出（默认）
$ cnb-rs issue list
#42  [open]   修复登录问题
#41  [closed] 添加用户管理
#40  [open]   优化性能
```

## URL 输出

```bash
# 创建命令返回 URL
$ cnb-rs issue create -t "新 Issue"
https://cnb.cool/org/repo/-/issues/42
```

## 详细输出

```bash
# 查看详情
$ cnb-rs issue view 42
编号: #42
标题: 修复登录问题
状态: open
优先级: P1
作者: zhangsan (张三)
创建时间: 2024-01-15 10:30:00

标签:
  - bug
  - backend

描述:
登录页面在移动端显示异常...

评论数: 5
```

## 输出规则

1. **JSON 输出**：使用 `--json` 标志，格式化显示
2. **表格输出**：适用于列表类命令
3. **简洁输出**：默认格式，一行一条
4. **URL 输出**：创建类命令返回资源 URL
5. **详细输出**：查看类命令展示完整信息

## 特殊输出

### 错误输出

```bash
$ cnb-rs issue view 999
错误: Issue #999 不存在

$ cnb-rs issue create
错误: 缺少必填参数: --title
```

### 分页输出

```bash
$ cnb-rs issue list --limit 10
显示 1-10 / 共 45 条
使用 --page 2 查看更多
```

### 进度输出

```bash
$ cnb-rs repo clone org/repo
正在克隆仓库...
接收对象: 100% (1234/1234), 2.5 MiB | 1.2 MiB/s, 完成.
解析增量: 100% (567/567), 完成.
```
