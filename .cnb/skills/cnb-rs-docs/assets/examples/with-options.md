# 带选项的示例模板

用于生成包含可选参数的命令示例。

## 模板

```bash
# 带<特定选项>的用法
$ cnb-rs <command> --option value

# 带多个选项的完整用法
$ cnb-rs <command> -s value --long value --flag
```

## 示例

### Issue 创建

```bash
# 创建带描述的 Issue
$ cnb-rs issue create -t "新功能需求" -b "详细描述..."

# 创建带优先级的 Issue
$ cnb-rs issue create -t "紧急 Bug" -p P0

# 创建带标签和负责人的 Issue
$ cnb-rs issue create -t "优化性能" -l "enhancement" -a "zhangsan"

# 创建完整的 Issue
$ cnb-rs issue create -t "紧急 Bug" -b "页面崩溃" -p P0 -l "bug,urgent" -a "zhangsan,lisi"
```

### Issue 列表

```bash
# 列出所有 Issue
$ cnb-rs issue list

# 列出特定状态的 Issue
$ cnb-rs issue list --state closed

# 列出带标签过滤的 Issue
$ cnb-rs issue list -l bug

# 列出并指定数量
$ cnb-rs issue list --limit 50
```

### PR 创建

```bash
# 创建简单 PR
$ cnb-rs pr create -t "添加新功能"

# 创建带描述的 PR
$ cnb-rs pr create -t "修复 Bug" -b "修复了登录问题"

# 创建指定分支的 PR
$ cnb-rs pr create -t "新功能" --head feature-branch --base main
```

## 格式规则

1. 每个示例以 `#` 注释开头说明目的
2. 从简单到复杂排列
3. 展示常见选项组合
4. 包含实际输出（如有）
