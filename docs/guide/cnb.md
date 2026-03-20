# cnb-rs

在命令行中高效管理你的 CNB 平台资源。

> [!WARNING]
> 旧命令 `cnb` 已改为 `cnb-rs`。如果你是从旧版本升级，请先阅读 [从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)。

## 常用命令

- [cnb-rs auth](/auth/) — 认证管理
- [cnb-rs chat](/chat) — 使用自然语言与 CNB OpenAPI 交互
- [cnb-rs config](/config/) — 配置管理
- [cnb-rs issue](/issue/) — Issue 管理
- [cnb-rs pr](/pr/) — PR 管理
- [cnb-rs release](/release/) — Release 管理
- [cnb-rs commit](/commit/) — Commit 管理

## 仓库命令

- [cnb-rs repo](/repo/) — 仓库管理
- [cnb-rs build](/build/) — 构建管理
- [cnb-rs label](/label/) — 标签管理
- [cnb-rs badge](/badge/) — 徽章管理
- [cnb-rs member](/member/) — 成员管理
- [cnb-rs browse](/browse) — 在浏览器中打开仓库页面
- [cnb-rs download](/download) — 下载仓库文件
- [cnb-rs info](/info) — 显示仓库和用户信息
- [cnb-rs stats](/stats) — 提交统计仪表盘
- [cnb-rs stars](/stars) — Star 趋势图

## 组织与制品管理

- [cnb-rs group](/group/) — 组织管理
- [cnb-rs mission](/mission/) — 任务集管理
- [cnb-rs registry](/registry/) — 制品库管理
- [cnb-rs knowledge](/knowledge/) — 知识库管理

## 用户与安全

- [cnb-rs user](/user/) — 用户信息
- [cnb-rs gpg-key](/gpg-key/) — GPG 密钥管理

## 其他命令

- [cnb-rs workspace](/workspace/) — 云原生工作区管理
- [cnb-rs completion](/completion) — 生成命令行补全脚本
- [cnb-rs version](/version) — 显示版本信息

## 选项

```bash
--domain <DOMAIN>     指定 CNB 域名（默认: cnb.cool）
--repo <OWNER/REPO>   指定仓库路径
--version             显示版本号
-h, --help            显示帮助信息
```

## 示例

```bash
$ cnb-rs auth login
$ cnb-rs issue list
$ cnb-rs chat --do "查看我的仓库列表"
$ cnb-rs download --files README.md
$ cnb-rs pr create
```
