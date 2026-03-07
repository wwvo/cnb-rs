# cnb

在命令行中高效管理你的 CNB 平台资源。

## 常用命令

- [cnb auth](/auth/) — 认证管理
- [cnb chat](/chat) — 使用自然语言与 CNB OpenAPI 交互
- [cnb config](/config/) — 配置管理
- [cnb issue](/issue/) — Issue 管理
- [cnb pull](/pull/) — Pull Request 管理
- [cnb release](/release/) — Release 管理
- [cnb commit](/commit/) — Commit 管理

## 仓库命令

- [cnb repo](/repo/) — 仓库管理
- [cnb build](/build/) — 构建管理
- [cnb label](/label/) — 标签管理
- [cnb badge](/badge/) — 徽章管理
- [cnb member](/member/) — 成员管理
- [cnb browse](/browse) — 在浏览器中打开仓库页面
- [cnb download](/download) — 下载仓库文件
- [cnb info](/info) — 显示仓库和用户信息
- [cnb stats](/stats) — 提交统计仪表盘
- [cnb stars](/stars) — Star 趋势图

## 组织与制品管理

- [cnb group](/group/) — 组织管理
- [cnb mission](/mission/) — 任务集管理
- [cnb registry](/registry/) — 制品库管理
- [cnb knowledge](/knowledge/) — 知识库管理

## 用户与安全

- [cnb user](/user/) — 用户信息
- [cnb gpg-key](/gpg-key/) — GPG 密钥管理

## 其他命令

- [cnb workspace](/workspace/) — 云原生工作区管理
- [cnb completion](/completion) — 生成命令行补全脚本
- [cnb version](/version) — 显示版本信息

## 选项

```bash
--domain <DOMAIN>     指定 CNB 域名（默认: cnb.cool）
--repo <OWNER/REPO>   指定仓库路径
--version             显示版本号
-h, --help            显示帮助信息
```

## 示例

```bash
$ cnb auth login
$ cnb issue list
$ cnb chat --do "查看我的仓库列表"
$ cnb download --files README.md
$ cnb pull create
```
