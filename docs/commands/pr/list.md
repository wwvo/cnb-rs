# cnb-rs pr list

```
cnb-rs pr list
```

列出与当前用户相关的所有 `open` 状态 Pull Request。

同时查询两类 PR 并合并显示：

- **我发起的**（`ME->`）
- **需要我审查的**（`->ME`）

输出为表格格式，包含 PR 编号、标题、阻塞状态和类型标记。

## 选项

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs pr list
```

## 输出示例

```
NUMBER          TITLE                                                   BLOCKEDON       TYPE
#42             重构认证模块                                                             ME->
#56             修复样式问题                                             ci              ->ME
```

## 另请参阅

- [cnb-rs pr](/pr/)
