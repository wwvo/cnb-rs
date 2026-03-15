# cnb-rs member repo-user-access

```
cnb-rs member repo-user-access <username>
```

查看指定成员在仓库的权限层级（含继承链）。

显示该用户在仓库中的直接权限和通过组织继承获得的权限层级。

## 选项

- `<username>`: 用户名（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs member repo-user-access zhangsan

# JSON 格式输出
$ cnb-rs member repo-user-access zhangsan --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member repo-access-level](/member/repo-access-level)
