# 基本用法示例模板

用于生成最简单的命令调用示例：

```bash
# 基本<动作>
$ cnb-rs <command> <required-args>
<expected-output>
```

## 示例

```bash
# 创建简单 Issue
$ cnb-rs issue create -t "修复登录问题"
https://cnb.cool/org/repo/-/issues/42

# 查看仓库信息
$ cnb-rs info
仓库名称: my-repo
默认分支: main

# 列出最近的 Issue
$ cnb-rs issue list
#42  [open]   修复登录问题
#41  [closed] 添加用户管理
```
