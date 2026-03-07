# cnb label

```
cnb label <subcommand>
```

管理仓库标签，以及 Issue / Pull Request 上的标签操作。

## 可用子命令

### 仓库标签

| 子命令   | 说明             |
|----------|------------------|
| list     | 列出仓库所有标签 |
| create   | 创建标签         |
| update   | 更新标签         |
| delete   | 删除标签         |

### Issue 标签

| 子命令       | 说明                 |
|--------------|----------------------|
| issue-list   | 列出 Issue 标签      |
| issue-add    | 为 Issue 添加标签    |
| issue-set    | 替换 Issue 标签      |
| issue-remove | 移除 Issue 单个标签  |
| issue-clear  | 清空 Issue 所有标签  |

### Pull 标签

| 子命令       | 说明                 |
|--------------|----------------------|
| pull-list    | 列出 Pull 标签       |
| pull-add     | 为 Pull 添加标签     |
| pull-set     | 替换 Pull 标签       |
| pull-remove  | 移除 Pull 单个标签   |
| pull-clear   | 清空 Pull 所有标签   |

## 示例

```bash
# 列出仓库所有标签
cnb label list

# 创建标签
cnb label create -n "bug" -c "d73a4a" -d "Bug 修复"

# 为 Issue 添加标签
cnb label issue-add 42 -l "bug" -l "urgent"

# 列出 Pull 标签
cnb label pull-list 10
```

## 另请参阅

- [cnb issue](/issue/)
- [cnb pull](/pull/)
