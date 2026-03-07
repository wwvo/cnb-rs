# cnb workspace closed-clean

```
cnb workspace closed-clean
```

清理所有已关闭的云原生工作区。

自动分页获取所有状态为 `closed` 的工作区，逐个执行删除操作。

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb workspace closed-clean
共找到 3 个已关闭的工作区

[INFO] 开始清理工作区 slug=my-org/my-repo pipelineId=abc123
[SUCCESS] 已清理工作区 slug=my-org/my-repo pipelineId=abc123
```

## 另请参阅

- [cnb workspace](/workspace/)
- [cnb workspace delete](/workspace/delete)
