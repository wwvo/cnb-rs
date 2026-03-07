# cnb info

```
cnb info
```

显示当前用户和仓库的详细信息。

通过 API 获取当前认证用户信息和仓库信息，输出以下字段：

**用户信息：**

- `MyID` — 用户 ID
- `MyUserName` — 用户名
- `MyNickName` — 昵称
- `MyEmail` — 邮箱

**仓库信息：**

- `RepoName` — 仓库路径
- `RepoID` — 仓库 ID
- `RepoLicense` — 许可证
- `RepoStars` — Star 数
- `RepoForks` — Fork 数
- `RepoDescription` — 仓库描述

## 示例

```bash
# 显示当前仓库信息
$ cnb info

# 指定仓库
$ cnb --repo wwvo/cnb-cli/cnb info
```

## 输出示例

```
  MyID             12345
  MyUserName       zhangsan
  MyNickName       张三
  MyEmail          zhangsan@example.com
  RepoName         wwvo/cnb-cli/cnb
  RepoID           67890
  RepoLicense      MIT
  RepoStars        128
  RepoForks        32
  RepoDescription  一个非官方的 CNB 命令行工具
```

## 另请参阅

- [cnb](/guide/cnb)
