# Changelog

此文件由 [git-cliff](https://git-cliff.org/) 自动生成。

## [0.6.2] - 2026-03-17

### ✨ 新功能

- **(release)** 接入 Linux 原生包正式发布链路 ([f058848](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/f05884817c0b61e8cf76cd112aaa53b7a871c5ae))
- **(dist)** 支持 Linux RPM 打包校验链路 ([11cf7ea](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/11cf7ea4c9e7c806dc8b48a6e157aac64144ffb7))
- **(dist)** 支持 Linux DEB 打包校验链路 ([f332401](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/f33240107b482b9b1badb68af114567e9383b1d8))

### 📝 文档

- **(install)** 补充 Scoop 与 Homebrew 安装方式 ([e92a256](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/e92a25654eff282f44c9efe733477df037c2317f))
- **(install)** 补齐 Linux 原生包安装说明 ([fd7cd3b](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/fd7cd3b80cbefb0ab21e1db83f8827547a3d5cd2))

## [0.6.1] - 2026-03-17

### 🐛 Bug 修复

- **(ci)** 将 automerge mergeType 改为 squash ([217c3a1](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/217c3a10b2996faeefbed03be1c691b66322a071))
- **(ci)** 补充 release-issue-checklist workflow include ([d8c2967](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/d8c2967b8f58ba0fb5e5e008e5c854ffe002abac))

### 👷 CI/CD

- **(release)** 提高发布归档压缩级别 ([6bda72c](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/6bda72cd50ca25d15e6d3eb6f940a70d2a18930b))

### 📝 文档

- 收口顶层名称为 cnb-rs ([4c6d3cf](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/4c6d3cf22019c5be7755c3aa82d3bd571b7e6c2f))

### 🔧 杂项

- **(ci)** 将 GitHub Actions 版本钉位切到 v2 ([c02d04c](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/c02d04c797e74296149d76ced1de076b7b43d08f))

## [0.6.0] - 2026-03-16

### ✨ 新功能

- 添加 cnb-rs-docs skill 用于生成 CLI 文档 ([db258b5](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/db258b5fa23b9782c24b7459b261eff66673c629))

### 🐛 Bug 修复

- **(ci)** 对齐 .cnb rust-ci 镜像命名空间 ([4d2bf8d](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/4d2bf8de05d041268fd35b7c39e7e4bbfbc93d70))
- **(migration)** 收尾 #120 主入口 identity 残留 ([5305a41](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/5305a41eb63bf58af79a796f2db5dee554605174))
- **(migration)** 对齐仓库 identity 与 git-sync 配置 ([5367f88](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/5367f88caafbec402417acf620724eafb3ee770d))
- **(release)** 对齐 release issue checklist 收口规则 ([21e7da0](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/21e7da0f4cb4263b3fc553cf41a1aeab3b7bb723))

### 📝 文档

- **(skill)** 优化 cnb-rs-docs skill 文档 ([de9a8b9](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/de9a8b964b38f7ef509c74207ebf06ed9823e0e7))

### 🔧 杂项

- **(cargo)** 对齐根包 package name 到 cnb-rs ([e64c84b](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/e64c84bd581147cb2682a1e15a133289727e5972))
- merge node(merged by CNB) ([27f4ad1](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/commit/27f4ad147521ab77fb69dc370563e9dfee6a3ae0))

## [0.5.0] - 2026-03-15

### 迁移提示

- 命令入口已从 `cnb ...` 改为 `cnb-rs ...`
- 发布产物文件名将从 `cnb-*` 改为 `cnb-rs-*`
- `git-cnb` 二进制入口已移除
- Scoop / Homebrew 外部分发仓库迁移正在跟进 [#114](https://cnb.cool/wwvo/cnb-cli/cnb/-/issues/114)

### ♻️ 重构

- **(cli)** 移除 git-cnb 二进制入口 ([f030c8a](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/f030c8abe1054731dac5eea38d09aef185e9ebf0))

### ✨ 新功能

- **(cli)** 将命令入口改为 cnb-rs ([9d0e382](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/9d0e382061735a99d95353078b1601b55ca90b3d))

### 👷 CI/CD

- **(github)** 恢复 Windows ARM64 gnullvm 实验目标 ([490cff5](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/490cff5918f688248a7756fb8832ab002b513740))
- **(github)** 收缩默认发布矩阵并清理 Node 24 告警 ([06b25b2](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/06b25b23fd37d380deb2e6df5391c6ac55bf8e20))
- **(release)** 新增 issue 收口清单按钮 ([f4282ce](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/f4282ceed2438991d63590a337e856ab4012a4f0))
- **(release)** 过滤附件插件旧式输出 ([aa3633c](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/aa3633cbb49d93ebfd79addf03bd49a139e3abb3))

### 📝 文档

- **(migration)** 补充 cnb-rs 迁移与发版说明 ([cbfba90](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/cbfba90841313c79bdd845e05ed50e9a7d86e933))
- 删除过时的 workflow 设计说明 ([67a8385](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/67a838564ce7610ab8ba84afd6ad1a554edb2a1e))

## [0.4.6] - 2026-03-15

### 👷 CI/CD

- **(release)** 放宽实验性 Windows ARM64 目标 ([32eb863](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/32eb863926895046457359ca3db07b01b34c792a))
- **(github)** 升级 Node 24 兼容 Actions ([5c3d5d2](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/5c3d5d294983aed9365f309717fe6409b4469e25))
- **(github)** 更正 Windows ARM64 GNU 目标 ([2176b48](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/2176b48ecaf4b51aabf06676ae3e2ef06762970e))
- **(image)** 改为下载预编译 git-cliff ([dba5cdf](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/dba5cdf6bc4042d247709f4b21e73babfedd7450))

## [0.4.5] - 2026-03-15

### 👷 CI/CD

- **(release)** 稳定同步 release lockfile ([b00cb05](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/b00cb05c5cb39ddcefc909fd4a8b77e8f538d617))

## [0.4.4] - 2026-03-15

### 👷 CI/CD

- **(release)** 修复 release commit message 换行 ([130cbcc](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/130cbcc44b8fc9527c22b1c566b2f64d1cb10ea7))
- **(release)** 规范 git-cliff 版本标题 ([c690c68](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/c690c68bfd61eb82eec98e7bd4a379f7db78da83))
- **(release)** 同步 release prepare 的 Cargo.lock ([19387d5](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/19387d5d06fdf2e858f8793dc027f1a6e7d17a7c))

## [0.4.3] - 2026-03-15

### 👷 CI/CD

- **(release)** 修复 release PR changelog 版本标题 ([b96202e](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/b96202e8c14d39a6e1b4f6f16513c73e2472d45a))
- **(tag)** 收紧自动打 tag 的版本校验 ([b5bce23](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/b5bce230cb775d88148a812151eee84d8220aab1))
- **(release)** 为 release 添加 SHA-256 校验文件 ([1685582](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/1685582c3af4a9d3f50f1844c53bb76229a4df8b))
- **(github)** 恢复 Windows ARM64 GNU 发布附件 ([97204f1](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/97204f10baffc269881ce7cfc01f5cebc845f09f))
- **(github)** 恢复 Windows ARM64 MSVC 发布附件 ([8d25a42](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/8d25a424b2ad4d98e28cc7a6a3628b6624d0379b))
- **(github)** 恢复 Windows MSVC 发布附件 ([b3925fe](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/b3925feeb401722a817a7b661eaad7a5031e5be9))
- **(workflow)** 切换 CNB 流水线消费 rust-ci 镜像 ([a258ee7](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/a258ee7b89cf86cece5b5f7cb10190998b7347ae))
- **(sync)** 拆分 GitHub main 与 tag 同步 ([2a13c15](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/2a13c15c97144581c7b196e7545a468c321f018a))
- **(image)** 收缩并更名 Rust CI 镜像 ([5b6f79b](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/5b6f79b62a3def25bcf443f6176fede3652c7548))
- **(github)** 兼容空附件删除 ([012e916](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/012e916427d5b549a2e945c98d2d2b054208f066))

### 🔧 杂项

- **(deps)** 升级 clap 依赖 ([0c1e6e5](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/0c1e6e5b20a40539c594498ab78fa23dbcb5bd78))

## [0.4.2] - 2026-03-15

### ♻️ 重构

- 用 paginate() 替换 list_all_issues 手写分页 ([3de5133](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/3de5133cfa5ab4f741d3497d3b5646011b4d31a2))
- 统一 GET 请求使用 send_with_retry 重试机制 ([b0f7012](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/b0f7012f5e101151b973d3f900d5f73f06f34cff))
- **(ci)** 消除 build-image.yml 中的重复 job 定义 ([d80e0fc](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/d80e0fc32cd80f5d3c4b449c77556665385c95e9))

### ✨ 新功能

- 新增代码检查与发版前检查流水线 #38 ([aed69aa](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/aed69aa7ccd31395f1196a9819dbfa553bc65532))
- add GitHub Actions workflow for multi-platform cnb cli build ([929555d](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/929555d133cb13693be922103ae25c887c4a27ea))
- **(ci)** 在 release 流水线中添加更新 CHANGELOG.md 的步骤 ([9519aa2](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/9519aa2abc8198d6f21bd93aa441932d19c29b84))

### 🎨 代码风格

- clean up rustfmt and clippy debt ([53c0cca](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/53c0ccac579bd812693b2853652a5d60ed479325))

### 🐛 Bug 修复

- 修正 CI 触发条件与 automerge 事件 #42 ([6460c55](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/6460c552d864a1f0df5e7dd3234827db4a2b3ccc))
- 修复 CLI 危险操作确认与下载分页缺陷 #34 ([531ea56](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/531ea5640763c6b610484551e34da7ffd84911b3))
- config.rs 解析失败时返回错误而非静默使用默认值 ([d3bc8cc](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/d3bc8ccb19c3efed4ff916f9f4639f4fd9ae4b34))
- validate_file 使用安全类型转换替代 as i64 强转 ([f90d0e8](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/f90d0e8e295b2f65adb8dd32179411ae1131d558))
- cnb_home_dir() 获取 home 目录失败时记录警告日志 ([ec6944a](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/ec6944add868cd0254372af5d928dc5228c3d5cd))
- 将主入口错误输出从 stdout 改为 stderr ([ded319f](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/ded319f6e5311964616e8878731e91d482f98aa1))
- 为 ListReposOptions::query_string() 添加 URL 编码 ([53c91c7](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/53c91c713df496e15d03a0ef194856b8078d0e5e))
- 修复字符串截断的 UTF-8 安全问题 ([a52e901](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/a52e9011c1cf433dcee27b6180c6017a43cef430))
- 修复字符串截断的 UTF-8 安全问题 ([1cc40b6](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/1cc40b62032c5830c25040142f6b37931078f4cc))
- **(docker)** 修复框架桩目录创建使用 sh 不支持的花括号扩展 ([9e7b6bc](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/9e7b6bcbcec03061dd091beb89ad24c037ca4101))
- **(docker)** 创建 macOS 框架桩文件解决交叉编译链接错误 ([87b8698](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/87b869897858ee8ec59260e6c26aaa51b92e39c5))

### 👷 CI/CD

- **(release)** 将 tag 发布切换为 GitHub 同步 ([063436a](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/063436ab0a2ffad3ec56d64a78ac326b688d9c2a))
- **(workflow)** 添加通用流水线辅助脚本 ([8b9fd3e](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/8b9fd3eda7d7159c8c5996c3d21abcbe69da82de))
- decouple release workflow from protected main ([38387d2](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/38387d2730434117e8b8ab81c093981c8bd378ec))
- 添加自动合并工作流 #33 ([646fbae](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/646fbae317e06e799d8bed8088609d0724c10d09))

### 📦 构建

- **(deps)** 同步 cnb-tui 的 Cargo.lock 依赖记录 ([91caa9f](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/91caa9f3cf331505e5d705c1ed37631308ad4f8f))

### 🔧 杂项

- 统一仓库文本文件行尾规则 #36 ([3056199](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/3056199b36b5331aa8f674124d14267f1afa5641))
- 更新仓库路径 prevailna/cnb → wwvo/cnb-cli/cnb ([c8b3f4a](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/c8b3f4ab4176784a98fb712be05341a1a6b32e19))
- **(ci)** 添加临时 macOS 构建测试工作流 ([e13df04](https://cnb.cool/wwvo/cnb-cli/cnb/-/commit/e13df0440a40ca3fe4986236cd1b7e3b5a50e246))

## [0.4.1] - 2026-03-07

### ♻️ 重构

- **(config,repo)** 移除 SSH 克隆支持，git_protocol 仅接受 https ([55284aa](https://cnb.cool/prevailna/cnb/-/commit/55284aa5147a61a61e4735c0a71c5f62952d5523))

### 📝 文档

- **(auth)** 选项格式从定义列表转为无序列表 ([933d5cd](https://cnb.cool/prevailna/cnb/-/commit/933d5cdefb47aa4235ea641c7aeee945756ea24d))
- **(config,clone)** 添加 CNB 暂不支持 SSH 克隆说明 ([97b9940](https://cnb.cool/prevailna/cnb/-/commit/97b994038d8575f7762b2ee9adcd467b618da67c))

### 🔧 杂项

- **(docs)** 增强 VitePress 站点配置 ([887df12](https://cnb.cool/prevailna/cnb/-/commit/887df125386f7bbbff82c073e4dee73439445621))

## [0.4.0] - 2026-03-07

### ♻️ 重构

- **(docs)** 将命令文档移入 commands/ 目录 + Route Rewrites ([3ac3789](https://cnb.cool/prevailna/cnb/-/commit/3ac3789fb59d2168770783c8a5460343c68df70d))

### ✨ 新功能

- **(registry)** 注册 registry 命令到 CLI ([e995cb6](https://cnb.cool/prevailna/cnb/-/commit/e995cb64c7bd338b7e9b3642143f16f2a9e517ad))
- **(registry)** 实现 registry tag delete 子命令 ([5ad6b8d](https://cnb.cool/prevailna/cnb/-/commit/5ad6b8d81c7aba62692d7431658829d7a1ffeac2))
- **(registry)** 实现 registry tag detail 子命令 ([4fefb4e](https://cnb.cool/prevailna/cnb/-/commit/4fefb4e5c2add6cdb6329d3287d367fe2f34ae7b))
- **(registry)** 实现 registry tag list 子命令 ([ff9de46](https://cnb.cool/prevailna/cnb/-/commit/ff9de4692e8ec95ccc00214f054cb59718975c5f))
- **(registry)** 实现 registry package delete 子命令 ([f18a161](https://cnb.cool/prevailna/cnb/-/commit/f18a1617ddd53dede2bfc6f64849fa7b9cde2f2c))
- **(registry)** 实现 registry package detail 子命令 ([d6b994a](https://cnb.cool/prevailna/cnb/-/commit/d6b994acdc5f1c6978128ffd0a362476c8c9b3c0))
- **(registry)** 实现 registry package list 子命令 ([b3e682b](https://cnb.cool/prevailna/cnb/-/commit/b3e682b7c4f0f0dc488758b530787ab2f03e7527))
- **(registry)** 实现 registry set-visibility 子命令 ([c637d63](https://cnb.cool/prevailna/cnb/-/commit/c637d639f79a13356297cb83ed517714c911e051))
- **(registry)** 实现 registry delete 子命令 ([3607dfc](https://cnb.cool/prevailna/cnb/-/commit/3607dfcc8543adb6ff7b6e57317df0c9ba083689))
- **(registry)** 实现 registry list 子命令 ([2540866](https://cnb.cool/prevailna/cnb/-/commit/2540866c53201bc14bd6aaf025cd2a7d4132556c))
- **(registry)** 添加 Registry 类型定义和 API 客户端方法 ([82b4cb0](https://cnb.cool/prevailna/cnb/-/commit/82b4cb06ed1bd9f67ae7fb9ce81a359ac90d70ab))
- **(mission)** 注册 mission 命令到 CLI ([ed7fd88](https://cnb.cool/prevailna/cnb/-/commit/ed7fd888e7fef39134c97c11f5f2ae34c8a3f30e))
- **(mission)** 实现 mission view sort 子命令 ([9d093b8](https://cnb.cool/prevailna/cnb/-/commit/9d093b85866cd831e0457a850595c5fc03ae6ff5))
- **(mission)** 实现 mission view add 子命令 ([7c8a275](https://cnb.cool/prevailna/cnb/-/commit/7c8a27523948c220893ae48abc769b5be6a13bde))
- **(mission)** 实现 mission view set 子命令 ([eafcbf8](https://cnb.cool/prevailna/cnb/-/commit/eafcbf810ebb3b375d667cac61201b18829d8c7f))
- **(mission)** 实现 mission view get 子命令 ([b688d7f](https://cnb.cool/prevailna/cnb/-/commit/b688d7f405c359eb6d13d17c11fa28ac91118706))
- **(mission)** 实现 mission view list 子命令 ([8f84f7f](https://cnb.cool/prevailna/cnb/-/commit/8f84f7fdd51db680badc7b0d9284407352a1a765))
- **(mission)** 实现 mission set-visibility 子命令 ([dbdeb99](https://cnb.cool/prevailna/cnb/-/commit/dbdeb996420191b380dda5ddaf1f2434fbd23fe7))
- **(mission)** 实现 mission delete 子命令 ([c894903](https://cnb.cool/prevailna/cnb/-/commit/c8949035dc5755b01ad19486eae70e1588e58b27))
- **(mission)** 实现 mission create 子命令 ([83093fd](https://cnb.cool/prevailna/cnb/-/commit/83093fda8ac7586e27bb49b177967cda00b6645f))
- **(mission)** 实现 mission list 子命令 ([2f77819](https://cnb.cool/prevailna/cnb/-/commit/2f7781981f3970a9e1ecebb0b79fdacc1c6cdd81))
- **(mission)** 添加 Mission 类型定义和 API 客户端方法 ([c59a893](https://cnb.cool/prevailna/cnb/-/commit/c59a89382ee58d5ae3de2af0d021876647468c57))
- **(member)** 注册 member 命令到 CLI + 修复类型冲突 ([333f34e](https://cnb.cool/prevailna/cnb/-/commit/333f34e7242cc3d2368c01787da4053753f019a4))
- **(member)** 实现 member collaborator-remove 子命令 ([2cff8c7](https://cnb.cool/prevailna/cnb/-/commit/2cff8c7ec1cf169336f3e40d46ac4c74edf4da14))
- **(member)** 实现 member collaborator-update 子命令 ([c93f8cc](https://cnb.cool/prevailna/cnb/-/commit/c93f8cc603ca8eb6a2c1c815e00c3b3c73ed69d2))
- **(member)** 实现 member collaborator-list 子命令 ([9effccf](https://cnb.cool/prevailna/cnb/-/commit/9effccf1b0f5ba26060b610ea800a577887d19c1))
- **(member)** 实现 member group-inherited 子命令 ([41f84a3](https://cnb.cool/prevailna/cnb/-/commit/41f84a3a23416e38ca1daef698efd51ebd7d8c65))
- **(member)** 实现 member group-user-access 子命令 ([fd138d6](https://cnb.cool/prevailna/cnb/-/commit/fd138d61388a8d623a8860a3e6cbe7830500409e))
- **(member)** 实现 member group-access-level 子命令 ([6ceb8eb](https://cnb.cool/prevailna/cnb/-/commit/6ceb8ebfbc90a71078484aabfbc77f14be9f1c1d))
- **(member)** 实现 member group-remove 子命令 ([663855d](https://cnb.cool/prevailna/cnb/-/commit/663855deec2ff54539a5910e1e267a005adb4e95))
- **(member)** 实现 member group-update 子命令 ([32a2c72](https://cnb.cool/prevailna/cnb/-/commit/32a2c7272a487530ecbc5795971ca6335c37a9bc))
- **(member)** 实现 member group-add 子命令 ([2bc3efb](https://cnb.cool/prevailna/cnb/-/commit/2bc3efb2361d5d2cab51b1d63039c4762fdab4ee))
- **(member)** 实现 member group-list 子命令 ([bb8eb49](https://cnb.cool/prevailna/cnb/-/commit/bb8eb490518c0fdbfef861b2086e9d09f9840fed))
- **(member)** 实现 member repo-all 子命令 ([b88704a](https://cnb.cool/prevailna/cnb/-/commit/b88704a505a5826cd268737442014004f99ba271))
- **(member)** 实现 member repo-inherited 子命令 ([3dc3fa9](https://cnb.cool/prevailna/cnb/-/commit/3dc3fa916a674c6331d4fbb808996e23f8663caf))
- **(member)** 实现 member repo-user-access 子命令 ([3b46f62](https://cnb.cool/prevailna/cnb/-/commit/3b46f6297bdfbe33f29fb93d342e42a12f482faa))
- **(member)** 实现 member repo-access-level 子命令 ([0715919](https://cnb.cool/prevailna/cnb/-/commit/0715919839e15f15686cac900a15a4dc87d06c4d))
- **(member)** 实现 member repo-remove 子命令 ([921d457](https://cnb.cool/prevailna/cnb/-/commit/921d45702f72f65ef94f706b1e722a18e13e3d41))
- **(member)** 实现 member repo-update 子命令 ([969bf09](https://cnb.cool/prevailna/cnb/-/commit/969bf095c7537b9bc720d5f7b7d5b366acf16054))
- **(member)** 实现 member repo-add 子命令 ([f263e2e](https://cnb.cool/prevailna/cnb/-/commit/f263e2e97bc4f6464d2028efe6db690665df9fee))
- **(member)** 实现 member repo-list 子命令 ([3ec125d](https://cnb.cool/prevailna/cnb/-/commit/3ec125d7acebf6af08a29ce95a99f1881bc14859))
- **(member)** 添加 Member 类型定义和 API 客户端方法 ([45baa41](https://cnb.cool/prevailna/cnb/-/commit/45baa4179bba7c006e5c81b1d864ac324587bf29))
- **(user)** 注册 user 命令到 CLI ([83027ca](https://cnb.cool/prevailna/cnb/-/commit/83027caa9708c62167311f3b8dca3f5e1064d4e6))
- **(user)** 实现 user activity-detail 子命令 ([816ad65](https://cnb.cool/prevailna/cnb/-/commit/816ad6511479d28933edb35549b78c4c010ab65f))
- **(user)** 实现 user activities 子命令 ([c363cbf](https://cnb.cool/prevailna/cnb/-/commit/c363cbf2de7f564ecaf0426f10f50e5e064ec1a7))
- **(user)** 实现 user following 子命令 ([6bdd35e](https://cnb.cool/prevailna/cnb/-/commit/6bdd35e0ef76210ed68207a72610f05a353a09a8))
- **(user)** 实现 user followers 子命令 ([1f27b08](https://cnb.cool/prevailna/cnb/-/commit/1f27b082ce1ec9041e438bb657b4bb6fa7b06852))
- **(user)** 添加 User 社交/活动类型和 API 客户端方法 ([84ba364](https://cnb.cool/prevailna/cnb/-/commit/84ba364492b34eb105a47a3812a93bb1f5f6daa8))
- **(badge)** 注册 badge 命令到 CLI ([5edd96c](https://cnb.cool/prevailna/cnb/-/commit/5edd96ccb2110fee0b4cb317b022fb9b256a8fc4))
- **(badge)** 实现 badge upload 子命令 ([41dfac0](https://cnb.cool/prevailna/cnb/-/commit/41dfac039412af6d7927986843585180d82b2fc7))
- **(badge)** 实现 badge list 子命令 ([2b3c77f](https://cnb.cool/prevailna/cnb/-/commit/2b3c77f32e710ddaac800b88c515eb15af52d713))
- **(badge)** 实现 badge get 子命令 ([dfaa23b](https://cnb.cool/prevailna/cnb/-/commit/dfaa23b718f3cfbc8d82b5869d5f4c40f9b842b0))
- **(badge)** 添加 Badge 类型定义和 API 客户端方法 ([28ccf20](https://cnb.cool/prevailna/cnb/-/commit/28ccf2027e9fc804479517a736fd540373cf4bda))
- **(gpg-key)** 注册 gpg-key 命令到 CLI ([f795bbe](https://cnb.cool/prevailna/cnb/-/commit/f795bbeb8b7373521890d17d176ea12f288152ed))
- **(gpg-key)** 实现 gpg-key list 子命令 ([c58deda](https://cnb.cool/prevailna/cnb/-/commit/c58deda7a90680ee66d779c62a8912fe187a2c6a))
- **(gpg-key)** 添加 GpgKey 类型定义和 API 客户端方法 ([b94d4a3](https://cnb.cool/prevailna/cnb/-/commit/b94d4a348e9d52b414b619a0eb911b2e5e6103a3))
- **(build)** 注册 build 命令到 CLI ([1e35df1](https://cnb.cool/prevailna/cnb/-/commit/1e35df172e24f4dbcac96af73ade73d3c31726de))
- **(build)** 实现 build stage/download-log/delete-log/crontab-sync 子命令 ([941b356](https://cnb.cool/prevailna/cnb/-/commit/941b356b22eca889e45748bbcb085135b7dfca38))
- **(build)** 实现 build list 子命令 ([1e4c797](https://cnb.cool/prevailna/cnb/-/commit/1e4c797f7b816a36bc1c071f53537f0233917bb3))
- **(build)** 实现 build start/stop/status 子命令 ([3433d26](https://cnb.cool/prevailna/cnb/-/commit/3433d2638bc83689dde9bde70e8e63bd7295e5c9))
- **(build)** 添加 Build 类型定义和 API 客户端方法 ([3c182f6](https://cnb.cool/prevailna/cnb/-/commit/3c182f60c8196528502df8cc294b90a028d98fa5))
- **(label)** 注册 label 命令到 CLI ([151e659](https://cnb.cool/prevailna/cnb/-/commit/151e6591c0582cbb104141fcd215930ccfc8e073))
- **(label)** 实现 Pull 标签子命令 pull-list/add/set/remove/clear ([e9260dd](https://cnb.cool/prevailna/cnb/-/commit/e9260dd6158915fb1f4aa51901a8a6a2811cf1a8))
- **(label)** 实现 Issue 标签子命令 issue-list/add/set/remove/clear ([f84ecb9](https://cnb.cool/prevailna/cnb/-/commit/f84ecb95655ebbb745cb95c11c0d71e0112b141c))
- **(label)** 实现仓库标签子命令 list/create/update/delete ([ed93318](https://cnb.cool/prevailna/cnb/-/commit/ed933184d0c31bfb18c489619567d1ea1d123bf1))
- **(label)** 添加 Label 类型定义和 API 客户端方法 ([0d15ffb](https://cnb.cool/prevailna/cnb/-/commit/0d15ffb84199c91c2831a01ad1ca26a594f70e7a))
- **(version)** 增强版本信息输出 ([64c08d0](https://cnb.cool/prevailna/cnb/-/commit/64c08d09db193611c1046c614000478a6af52cb0))
- **(workspace)** 实现 workspace detail 子命令 ([8014799](https://cnb.cool/prevailna/cnb/-/commit/8014799cf9a1a0e9272d00f613b7d26229afeb11))
- **(workspace)** 实现 workspace delete 子命令 ([4cb4a45](https://cnb.cool/prevailna/cnb/-/commit/4cb4a45cd32f5d63a196a31d2a826c0e4c4eb015))
- **(workspace)** 实现 workspace stop 子命令 ([edf6e14](https://cnb.cool/prevailna/cnb/-/commit/edf6e14db35a814368567ac062b563fec77ac831))
- **(workspace)** 实现 workspace start 子命令 ([e38f950](https://cnb.cool/prevailna/cnb/-/commit/e38f950088f3edaaf3f99755dd3a27907d49a24f))
- **(workspace)** 实现 workspace list 子命令 ([699ec70](https://cnb.cool/prevailna/cnb/-/commit/699ec706e9d6f6d7cda7f1ad9a11dfd8b1ce95c7))
- **(workspace)** 扩展 Workspace 类型定义和 API 方法 ([57c586f](https://cnb.cool/prevailna/cnb/-/commit/57c586fbc32b6abcaa2d3af4a51c9f56eefc8bcf))
- **(release)** 实现 release asset-delete 子命令 ([5ca46ea](https://cnb.cool/prevailna/cnb/-/commit/5ca46ea6f7c199715da20fff9cbfc1aa746eeb9b))
- **(release)** 实现 release download 子命令 ([fa261fc](https://cnb.cool/prevailna/cnb/-/commit/fa261fcb45ed20322d687d103aafdbc1b228620d))
- **(release)** 实现 release latest 子命令 ([73ebadc](https://cnb.cool/prevailna/cnb/-/commit/73ebadc683ad6ac5593b55445c76a880c4f1d9aa))
- **(release)** 实现 release delete 子命令 ([b520c26](https://cnb.cool/prevailna/cnb/-/commit/b520c261264b72d80393e414563e210590e64954))
- **(release)** 实现 release update 子命令 ([ce02291](https://cnb.cool/prevailna/cnb/-/commit/ce022914a66df8b379ad4d0270e63d71f75da610))
- **(release)** 实现 release view 子命令 ([3af6c5c](https://cnb.cool/prevailna/cnb/-/commit/3af6c5c35bf1931a7d59590e06427a5437d9b3b5))
- **(release)** 扩展 Release 类型定义和 API 方法 ([8dcd35f](https://cnb.cool/prevailna/cnb/-/commit/8dcd35ffe02653b5f15c492a0028c52b6994ef2d))
- **(group)** 完善 member 子命令，对齐设计文档 ([12ad19b](https://cnb.cool/prevailna/cnb/-/commit/12ad19ba9dc792ab1b28cc6df936e9593eacdb32))
- **(group)** 添加 group member 子命令，支持成员管理 ([d823116](https://cnb.cool/prevailna/cnb/-/commit/d823116f5f8fa68e612604928566afe078ec85b0))
- **(group)** 添加 group quota 子命令，支持查看组织特权额度 ([2669d39](https://cnb.cool/prevailna/cnb/-/commit/2669d393761208b209b740c9230d265d9c90031b))
- **(group)** 添加 group settings 子命令，支持查看/更新组织配置 ([bcf8554](https://cnb.cool/prevailna/cnb/-/commit/bcf8554f6dd486d15adf3955e4beb8937db51c15))
- **(group)** 添加 group sub-groups 子命令，支持列出子组织 ([a22d0dc](https://cnb.cool/prevailna/cnb/-/commit/a22d0dce1481ac544881aefd552efbf2b08e1b04))
- **(group)** 添加 group transfer 子命令，支持转移组织 ([9741c10](https://cnb.cool/prevailna/cnb/-/commit/9741c100b17aecee7b34a35d555122b187162814))
- **(group)** 添加 group delete 子命令，支持删除组织 ([44269f6](https://cnb.cool/prevailna/cnb/-/commit/44269f6c3aab67101e0ecfdefb0bc95328c1e5e6))
- **(group)** 添加 group update 子命令，支持更新组织信息 ([273d520](https://cnb.cool/prevailna/cnb/-/commit/273d52070b905921f9421eeb460e20e867d33c5b))
- **(group)** 添加 group create 子命令，支持创建组织 ([a6f3140](https://cnb.cool/prevailna/cnb/-/commit/a6f3140f0250be6c470a0107cb86720049f395f0))
- **(group)** 添加 group view 子命令，支持查看组织详情 ([cb6edcb](https://cnb.cool/prevailna/cnb/-/commit/cb6edcb004ccdd2963f066fabee3f7b5ff7fdf0a))
- **(group)** 添加 group list 子命令，支持列出我的组织 ([43df38c](https://cnb.cool/prevailna/cnb/-/commit/43df38cf8767eaab83979dd7f3e5901b555c2be5))
- **(group)** 扩展 Group 类型定义和 API 方法，为新子命令奠定基础 ([f613cff](https://cnb.cool/prevailna/cnb/-/commit/f613cffb32046db05ae769ed3d396e6912024c89))

### 🐛 Bug 修复

- **(group)** 修复 quota.rs clippy redundant clone 错误 ([967d476](https://cnb.cool/prevailna/cnb/-/commit/967d4760efd8f81e8cdc952fabc0e652bbc2de7e))

### 📝 文档

- **(download,chat)** 优化文档格式与 auth 风格统一 ([c64837a](https://cnb.cool/prevailna/cnb/-/commit/c64837af2c54c7c51c72dd12f81f2ff8a99c6f3a))
- **(knowledge)** 优化文档格式与 auth 风格统一 ([bf282f5](https://cnb.cool/prevailna/cnb/-/commit/bf282f542fff0d82219760138ffcd9a071142d67))
- **(config)** 优化文档格式与 auth 风格统一 ([18acbda](https://cnb.cool/prevailna/cnb/-/commit/18acbda0e93841a77dcb23b349129f1c94e395dd))
- **(commit)** 优化文档格式与 auth 风格统一 ([246d43a](https://cnb.cool/prevailna/cnb/-/commit/246d43a70bfeaf180313f2b63a74eb898fb4191b))
- **(release)** 优化文档格式与 auth 风格统一 ([0455231](https://cnb.cool/prevailna/cnb/-/commit/04552317179b435b28dade1a73be243e05ab6301))
- **(pull)** 优化文档格式与 auth 风格统一 ([0451dd6](https://cnb.cool/prevailna/cnb/-/commit/0451dd61220b28d7b1bf85faac58f2d88734bf35))
- **(repo)** 优化文档格式与 auth 风格统一 ([c7b95e9](https://cnb.cool/prevailna/cnb/-/commit/c7b95e922e880ca54cd4887340f933fc1e5b1897))
- **(issue)** 优化文档格式与 auth 风格统一 ([d9b7875](https://cnb.cool/prevailna/cnb/-/commit/d9b78751fbc6b61e5364b52991d9eb024c6ce6af))
- **(gpg-key)** 优化文档格式与 auth 风格统一 ([007880b](https://cnb.cool/prevailna/cnb/-/commit/007880b57d6de76648a13915ef2046215029f6b6))
- **(workspace)** 优化文档格式与 auth 风格统一 ([4f2c73f](https://cnb.cool/prevailna/cnb/-/commit/4f2c73f477a3d94fbe998acd97aa82244dab31f6))
- **(group)** 优化文档格式与 auth 风格统一 ([8afa838](https://cnb.cool/prevailna/cnb/-/commit/8afa83870eba29ead514486eaefe1aef3326301a))
- **(label)** 优化文档格式与 auth 风格统一 ([44bf1bf](https://cnb.cool/prevailna/cnb/-/commit/44bf1bf7f0cd40c15183d854711d7ad50aa31ddc))
- **(build)** 优化文档格式与 auth 风格统一 ([f0f61bc](https://cnb.cool/prevailna/cnb/-/commit/f0f61bc8d1e18740c9de103b2ae578945939872a))
- **(browse)** 优化文档格式与 auth 风格统一 ([509aad1](https://cnb.cool/prevailna/cnb/-/commit/509aad14e4e26caa980404d8d1e132f64e5489a2))
- **(registry)** 优化文档格式与 auth 风格统一 ([9d1db3f](https://cnb.cool/prevailna/cnb/-/commit/9d1db3fc19da1b4ca87ad542fc66131d1a3c194d))
- **(mission)** 优化文档格式与 auth 风格统一 ([8d6423d](https://cnb.cool/prevailna/cnb/-/commit/8d6423d5474644e20a9260ed20bd0ad164f594d8))
- **(member)** 优化文档格式与 auth 风格统一 ([b16a2c0](https://cnb.cool/prevailna/cnb/-/commit/b16a2c0c3a6db4ae0900dd9e47231ea762d3adab))
- **(user)** 优化文档格式与 auth 风格统一 ([bf6a8cd](https://cnb.cool/prevailna/cnb/-/commit/bf6a8cded2520ed717abe923b8cbc316da0c3f37))
- **(badge)** 优化文档格式与 auth 风格统一 ([41b770e](https://cnb.cool/prevailna/cnb/-/commit/41b770ee7bace02286be5d5e060c574822e89f69))
- 补全侧边栏 browse 命令 + 按功能重新分类 ([ddab5be](https://cnb.cool/prevailna/cnb/-/commit/ddab5be0677526c917947b12d576c2f9563422b9))
- **(browse)** 创建 browse 命令文档修复死链接 ([14f2027](https://cnb.cool/prevailna/cnb/-/commit/14f20275eee02b6e23e9b1b17b17a5bb2ef381a9))
- **(guide)** 补全 cnb.md 命令列表 ([82158fb](https://cnb.cool/prevailna/cnb/-/commit/82158fbf40c093dea300ada16412da9ee6e8520a))
- **(registry)** 编写 registry 命令文档 + 侧边栏更新 ([4c47b1b](https://cnb.cool/prevailna/cnb/-/commit/4c47b1bd419f8f37114d691b49427c992436b91b))
- **(mission)** 编写 mission 命令文档 + 侧边栏更新 ([acf183f](https://cnb.cool/prevailna/cnb/-/commit/acf183f388c54bbee9dafbe44d939a78a7c9b9ec))
- **(member)** 编写 member 命令文档 + 侧边栏更新 ([3dd3d12](https://cnb.cool/prevailna/cnb/-/commit/3dd3d124088b868e960473a12eb034241221ada3))
- **(user)** 编写 user 命令文档 + 侧边栏更新 ([b30d3af](https://cnb.cool/prevailna/cnb/-/commit/b30d3afa967142da74182a1b089ee58bf2850a96))
- **(badge)** 编写 badge 命令文档 + 侧边栏更新 ([55fe0ea](https://cnb.cool/prevailna/cnb/-/commit/55fe0ea52d935faf123cbdb346d0bea168799c52))
- **(gpg-key)** 编写 gpg-key 命令文档 + 侧边栏更新 ([73e634e](https://cnb.cool/prevailna/cnb/-/commit/73e634eb1e11ec2c8ba80c7a61cffcc35cb3af70))
- 更新文档站描述为「一个非官方的 CNB 命令行工具」 ([1c870d6](https://cnb.cool/prevailna/cnb/-/commit/1c870d616e3662d7f782d8c081f553d8afa4372a))
- **(build)** 编写 build 命令文档 + 侧边栏更新 ([ff9c71a](https://cnb.cool/prevailna/cnb/-/commit/ff9c71aa5d2027ba547d8e0e4f11f5d6354a9e16))
- **(label)** 编写 label 命令文档 + 侧边栏更新 ([e164407](https://cnb.cool/prevailna/cnb/-/commit/e16440715eaa47fdde7dbc78c4df38326fd4b931))
- **(version)** 更新 version 文档站页面 ([59a4212](https://cnb.cool/prevailna/cnb/-/commit/59a42121a686dcd0bc9ec12da24a08cc71683a4d))
- **(workspace)** 添加新子命令文档并更新侧边栏 ([28206dc](https://cnb.cool/prevailna/cnb/-/commit/28206dc0a655b59f20181998f909123aea82fb99))
- **(release)** 添加新子命令文档并更新侧边栏 ([22217cb](https://cnb.cool/prevailna/cnb/-/commit/22217cb7b255243ce571c6d1252618381a9bc67d))
- **(group)** 更新 member 文档，添加 --inherited 和 access-level ([51ec90a](https://cnb.cool/prevailna/cnb/-/commit/51ec90abec75b47a91e1e061bd0583a2842c66a4))
- **(group)** 添加所有 group 子命令文档和侧边栏导航 ([40c1edf](https://cnb.cool/prevailna/cnb/-/commit/40c1edf2931b91972dfda8f5fef368dffea8752e))

### 🔧 杂项

- **(cli)** 更新工具描述为「一个非官方的 CNB 命令行工具」 ([54a929b](https://cnb.cool/prevailna/cnb/-/commit/54a929bbc2bf53b37034be57948b036376dedb62))

## [0.3.0] - 2026-03-06

### ♻️ 重构

- **(auth)** 统一使用 cnb-tui 输出宏替代 eprintln ([dab2204](https://cnb.cool/prevailna/cnb/-/commit/dab2204e1adb1b60b03f34cbfa2801b219f0e6f2))
- 修复全部 clippy pedantic warning（31 处） ([f32f2e5](https://cnb.cool/prevailna/cnb/-/commit/f32f2e57f2ec8a535b98c3b4376515feb9fa6715))
- **(api)** ai_chat_stream 复用 map_error_status 消除重复错误处理 ([8905f22](https://cnb.cool/prevailna/cnb/-/commit/8905f22455187ab3a0df23d448a9d5bffa7fb0de))
- **(api)** 提取 map_error_status 消除 handle_response 和 handle_empty_response 的重复逻辑 ([d14b27d](https://cnb.cool/prevailna/cnb/-/commit/d14b27dd361ece1b8effb8206d5f7cbc917876db))
- **(commit)** 去掉 CommitSubcommand 枚举变体的 Asset 前缀 ([6bfa4ec](https://cnb.cool/prevailna/cnb/-/commit/6bfa4ecdb7de25005a77d096c8c9cba3d9242185))
- **(cnb-tui,chat)** 抽象 ANSI 转义码到 style 模块 ([7de70ce](https://cnb.cool/prevailna/cnb/-/commit/7de70ce1972b60af58ac8af68d1478be2cbf6ee4))
- **(cnb-tui,chat)** 抽象 ANSI 转义码到 style 模块 ([ba44efe](https://cnb.cool/prevailna/cnb/-/commit/ba44efeae751d1afffd1dcc7163a2e05cdca9e78))
- **(release)** asset-stats 使用 Table 组件替代手动对齐 ([04058b2](https://cnb.cool/prevailna/cnb/-/commit/04058b2a2e2b283cdf05ea2b5062c58c26c7bd83))
- **(cnb-api)** 统一 404 错误表示为 ApiError::NotFound ([2e329f1](https://cnb.cool/prevailna/cnb/-/commit/2e329f12e030a3e106fb5f07c764b2060c66870f))

### ⚡️ 性能优化

- **(workspace)** 并发清理已关闭的工作区 ([03fcaab](https://cnb.cool/prevailna/cnb/-/commit/03fcaab4c817a5dcd5a9b46b09aa24f08c01a220))
- **(info)** 并行获取用户和仓库信息 + 添加 --json 支持 ([ccd2c15](https://cnb.cool/prevailna/cnb/-/commit/ccd2c156027cadc7728ae75810c867b12e70daa9))
- **(release,commit)** 附件删除改为并发执行 ([13bb28c](https://cnb.cool/prevailna/cnb/-/commit/13bb28c0597774fafbef0c2dfeca1823d06eb3a0))
- **(issue)** 并行化 issue download 的网络请求 ([6fe4d84](https://cnb.cool/prevailna/cnb/-/commit/6fe4d84bd362a16f233758076ee0a55c33f6b4d2))

### ✨ 新功能

- **(issue)** 增强 issue list 子命令，支持丰富的过滤和排序参数 ([6fea648](https://cnb.cool/prevailna/cnb/-/commit/6fea6487f18ea964ea5f71a70db8d3b8c528326e))
- **(issue)** 添加 issue label 子命令，支持标签管理（list/add/set/remove/clear） ([ff3e733](https://cnb.cool/prevailna/cnb/-/commit/ff3e7331e85ad3c66ae676636f37c21bbcb9bc31))
- **(issue)** 添加 issue reopen 子命令，支持重新打开已关闭的 Issue ([1586c09](https://cnb.cool/prevailna/cnb/-/commit/1586c09971f4a2fbd6d8e877712ea1a9c2f9468a))
- **(issue)** 添加 issue edit 子命令，支持编辑 Issue 信息 ([80d0866](https://cnb.cool/prevailna/cnb/-/commit/80d08663279ce4e57028aa662725965ea39740b4))
- **(issue)** 添加 issue view 子命令，支持查看 Issue 详情 ([2dd2ac2](https://cnb.cool/prevailna/cnb/-/commit/2dd2ac2226083385567aee41e56e015926c53824))
- **(issue)** 扩展 Issue 类型定义和 API 方法，为新子命令奠定基础 ([bb88eff](https://cnb.cool/prevailna/cnb/-/commit/bb88eff18a1bae2c58bb35e3ac1b92d9e147bbb3))
- **(repo)** create 添加 --clone / -c 标志，创建后自动克隆到本地 ([a258a71](https://cnb.cool/prevailna/cnb/-/commit/a258a719b3e389fd8199e3196ec20626b2e18b91))
- **(repo)** 添加 settings 子命令组（branch-protection/pull-request/push-limit/pipeline） ([ccec456](https://cnb.cool/prevailna/cnb/-/commit/ccec4565c2cabe0a60be67867d935666549ccdb7))
- **(repo)** 添加 contributor/security/top-contributors/events/asset 五个数据查询子命令 ([3a7355a](https://cnb.cool/prevailna/cnb/-/commit/3a7355a272d183a3233754ada95e60f5fdbb393d))
- **(repo)** 添加 repo pin 子命令，支持查看和设置仓库墙 ([55b67e3](https://cnb.cool/prevailna/cnb/-/commit/55b67e3f454d0cd03e77cb00f147d0e3f6862cee))
- **(repo)** 添加 archive/unarchive/visibility/transfer 四个运维子命令 ([ca1ec46](https://cnb.cool/prevailna/cnb/-/commit/ca1ec46250cf853a148690479d057456fe064f40))
- **(repo)** 添加 repo clone 子命令，支持克隆仓库到本地 ([45a1576](https://cnb.cool/prevailna/cnb/-/commit/45a157689ce7c0898f04dcb9b5448c29ff187bcd))
- **(repo)** 添加 repo fork 子命令，支持查看仓库 Fork 列表 ([e17719a](https://cnb.cool/prevailna/cnb/-/commit/e17719a233f40ae53f7017fc415b8a81b93c042e))
- **(repo)** 添加 repo delete 子命令，支持交互确认删除仓库 ([0c57d18](https://cnb.cool/prevailna/cnb/-/commit/0c57d1830ccc1013987fc6fc17501628fb142f8b))
- **(repo)** 添加 repo edit 子命令，支持编辑仓库描述/许可证/站点/主题 ([884e8ba](https://cnb.cool/prevailna/cnb/-/commit/884e8bab82872d9e2cb4b4900a8c13f492bcd325))
- **(repo)** 添加 repo create 子命令，支持创建仓库 ([2795045](https://cnb.cool/prevailna/cnb/-/commit/27950454dbc5e23d023b8dc87fb29bfc1fad8268))
- **(repo)** 添加 repo list 子命令，支持列出用户/组织仓库 ([5608b0f](https://cnb.cool/prevailna/cnb/-/commit/5608b0f0f351e8fa7969aeb4ca5f9fc96f8c07e7))
- **(repo)** 添加 repo 命令骨架和 repo view 子命令 ([bfe1ad3](https://cnb.cool/prevailna/cnb/-/commit/bfe1ad3f138d072a9ec9adfb7efdbae74aee9a79))
- **(knowledge)** list-models 添加 --json 支持 ([e3a3c67](https://cnb.cool/prevailna/cnb/-/commit/e3a3c676b81c84558bba18dd852d9376b30d9988))
- **(commit)** asset-stats 添加 Table 展示和 --json 支持 ([4235cfc](https://cnb.cool/prevailna/cnb/-/commit/4235cfce45509b36a09a59e6bd04a7cd83a53225))
- **(knowledge,release)** 统一添加 --json 输出支持 ([d7cdd1a](https://cnb.cool/prevailna/cnb/-/commit/d7cdd1ad3f4b1274ddac063700f33523888bc0c9))

### 🎨 代码风格

- **(issue)** 修复 view.rs clippy redundant_closure 警告 ([b8d5714](https://cnb.cool/prevailna/cnb/-/commit/b8d57146c07be550c5817a190218d068d4ac5e96))
- **(docs)** 将 API 章节移至示例和错误处理之后，优化阅读顺序 ([143e4e7](https://cnb.cool/prevailna/cnb/-/commit/143e4e7a4448ea85a6cd6f20efb60a44f2f8f7e1))
- **(docs)** 使用 VitePress code-group 优化环境变量和配置文件展示 ([d7e9423](https://cnb.cool/prevailna/cnb/-/commit/d7e9423b766c6b2c05129fdcdf0993061d518744))
- **(download)** 将 use std::io::Write 从函数体内移到文件顶部 ([e0c0841](https://cnb.cool/prevailna/cnb/-/commit/e0c08411d8e8033b86415e6832758d347555c483))
- **(stars)** 将 import 语句从文件中间移到顶部 ([9e0236b](https://cnb.cool/prevailna/cnb/-/commit/9e0236b68583891c8788f85ceae36273a9c9bdfa))

### 🐛 Bug 修复

- **(repo)** 修复 clippy 警告（struct_excessive_bools、single_match_else、redundant_closure） ([0b5b2e2](https://cnb.cool/prevailna/cnb/-/commit/0b5b2e2119f707dd40a2f9fcc6a5fd4f927f145a))
- **(docs)** 更正 Token 管理页面链接为 cnb.cool/profile/token ([3f9021e](https://cnb.cool/prevailna/cnb/-/commit/3f9021e50677b73a8967a620e317d59a1f3f20a3))
- **(docs)** 修复 status.md 内容错误，重写为正确的 auth status 文档 ([bb5deb5](https://cnb.cool/prevailna/cnb/-/commit/bb5deb5ef8f5c7e989240b3ee8fabc1fd9d324fb))
- **(pull)** list 命令传播 API 错误而非静默忽略 ([1e1ee6e](https://cnb.cool/prevailna/cnb/-/commit/1e1ee6e61dde604fa1f397873e92b984e8008d46))
- **(issue)** download 命令传播评论获取错误而非静默忽略 ([c90e4b8](https://cnb.cool/prevailna/cnb/-/commit/c90e4b8bd9e5e0b6e98f9b7f8087d47f56baea21))
- **(issue)** mine 命令传播 API 错误而非静默忽略 ([d90e6d5](https://cnb.cool/prevailna/cnb/-/commit/d90e6d5fa58d2af9afe8f58b47c1bf38fe6289df))
- **(cnb-chat)** 修复 clippy lint 警告 ([2fc49da](https://cnb.cool/prevailna/cnb/-/commit/2fc49da70207fc5593b42d0744418e71d0087144))
- 修复全部 clippy lint 问题 ([d3e8c3f](https://cnb.cool/prevailna/cnb/-/commit/d3e8c3f13d3260a56a01ca275c4600e598aac545))
- **(pull)** 修复 PR 列表未去重导致同一 PR 重复显示 ([e5c0b48](https://cnb.cool/prevailna/cnb/-/commit/e5c0b48f1f1e210374dff7c49e528050d7918d8d))

### 📝 文档

- **(issue)** 全面重写和增强 Issue 命令组文档 ([651789f](https://cnb.cool/prevailna/cnb/-/commit/651789f0303efdd7c572f3f34271034e08f59127))
- **(repo)** 添加 settings 子命令文档并更新侧边栏 ([b367c90](https://cnb.cool/prevailna/cnb/-/commit/b367c9028dc4e102517fca9eed6c739728c6a673))
- **(repo)** 添加 contributor/security/top-contributors/events/asset 文档并更新侧边栏 ([c71583d](https://cnb.cool/prevailna/cnb/-/commit/c71583d14c6f659120be4ac9d35b29ffa3d1c4f8))
- **(repo)** 添加 repo pin 文档并更新侧边栏 ([a774112](https://cnb.cool/prevailna/cnb/-/commit/a774112bc0a874ed7abde8fb137ca0a9b7fd8270))
- **(repo)** 补充 fork/clone/archive/unarchive/visibility/transfer 文档并更新侧边栏 ([9c06df0](https://cnb.cool/prevailna/cnb/-/commit/9c06df02d2b1fe25656986b54379db3280dff12e))
- **(repo)** 添加 repo 命令组 VitePress 文档（index/view/list/create/edit/delete） ([a2295e7](https://cnb.cool/prevailna/cnb/-/commit/a2295e732154eda9b547227a0d531ee0a1a80d87))
- **(auth)** 完善 logout.md 增加选项说明、Token 撤销提示和跨平台示例 ([4102c7e](https://cnb.cool/prevailna/cnb/-/commit/4102c7e93ccb96c8ff8b8e191c714b47052836d4))
- **(auth)** 完善 login.md 增加 API 详情、cURL 示例和错误处理 ([cc76a05](https://cnb.cool/prevailna/cnb/-/commit/cc76a05a0d6a336aec15bfdd1c3bf5ea13c36a19))
- **(auth)** 完善 index.md 增加环境变量、配置文件和安全说明 ([e3f1500](https://cnb.cool/prevailna/cnb/-/commit/e3f1500ad9fb66a1d27f267a7b4446f1876f8542))

### 📦 构建

- **(deps)** 升级 8 个依赖到最新版本 ([fa080a6](https://cnb.cool/prevailna/cnb/-/commit/fa080a6ccfebaf7811b104f6bbe06ced52b1341c))

### 🔧 杂项

- **(docs)** VitePress 侧边栏添加 repo 命令组导航 ([4997d2b](https://cnb.cool/prevailna/cnb/-/commit/4997d2b3fbca23519b453a05424b5bf306c4735d))
- 添加 clippy::all = warn 并整理 lint 配置分组 ([471b572](https://cnb.cool/prevailna/cnb/-/commit/471b57284e80a5b47057942f32e4ce890598641e))
- 启用 clippy::pedantic + rust-version + unsafe_code lint 配置 ([0a8c692](https://cnb.cool/prevailna/cnb/-/commit/0a8c6929e770292e35b959ff452f9742e9ad4b1f))

## [0.2.0] - 2026-03-06

### ♻️ 重构

- download 复用 CnbClient 的 http_plain 客户端 ([f7d9e2d](https://cnb.cool/prevailna/cnb/-/commit/f7d9e2d289c981d3bf26ecbfb7bba1de57142dbc))
- **(types)** 为 Issue/PullRequest/Release 响应类型添加 Serialize ([1e9b86d](https://cnb.cool/prevailna/cnb/-/commit/1e9b86d85e2f44fcb683ce2fcc325dfff758f8e8))
- **(cli)** version 子命令合并到 --version，包含 target 信息 ([d9ece57](https://cnb.cool/prevailna/cnb/-/commit/d9ece5769162a52501abe36b56726b1f7f371d7f))
- **(pull)** list --state 从 String 改为 ValueEnum 枚举 ([e6f8b14](https://cnb.cool/prevailna/cnb/-/commit/e6f8b14f3151a0230b630e171fcd48ce9691cc80))
- **(pull)** update --state 从 String 改为 ValueEnum 枚举 ([84081b5](https://cnb.cool/prevailna/cnb/-/commit/84081b546a3ec4ef2976f4855b8df197e5ebbc0e))
- **(types)** ContentType 从 String 改为枚举 ([c04c6c5](https://cnb.cool/prevailna/cnb/-/commit/c04c6c5717f65fd6d963dcea854d130bf9d7f5c5))
- **(issue)** close reason 从 String 改为 ValueEnum 枚举 ([9929959](https://cnb.cool/prevailna/cnb/-/commit/99299597005af39e666f8cb230a4dd493a09ff29))
- **(issue)** Issue 编号从命名参数改为位置参数 ([38acf14](https://cnb.cool/prevailna/cnb/-/commit/38acf144b9c2ecbdeeea4fdb0453241de0f10c9b))
- **(cli)** 修复 -s 短选项跨命令语义冲突 ([1a5f7b5](https://cnb.cool/prevailna/cnb/-/commit/1a5f7b5bcd18f2dee67c6cabb60e3f0f7cf2a864))
- **(release)** 统一 Tag 相关参数命名 ([70e8aff](https://cnb.cool/prevailna/cnb/-/commit/70e8aff56a51852763705b3f5ec920446f882b24))
- **(issue)** assignees 参数类型从 String 改为 Vec<String> ([71e4620](https://cnb.cool/prevailna/cnb/-/commit/71e462023468c54622a1a8354a260ccabce86a39))
- **(output)** P3 迁移空状态和进度消息到 stderr ([33df57b](https://cnb.cool/prevailna/cnb/-/commit/33df57b60688f60dbd6e0e83bb28fadc9fdcb86d))
- **(output)** P2 迁移 7 个命令的操作结果消息使用统一输出宏 ([6959a57](https://cnb.cool/prevailna/cnb/-/commit/6959a57ff778b3b60cc95d25780818caec56e7e3))
- **(output)** P1 迁移 workspace/commit/release asset-clean 使用统一输出宏 ([f5d3b29](https://cnb.cool/prevailna/cnb/-/commit/f5d3b29bf5348695fd4eb02bbd060b0d7d6b13e8))
- **(commands)** Chat 和 Download 命令增加 execute() 方法 ([eb734ea](https://cnb.cool/prevailna/cnb/-/commit/eb734ea9ba97cc280867f1373b648e8dbeea8d0c))
- **(api)** 按领域拆分 CnbClient 为 9 个独立文件 ([761cb26](https://cnb.cool/prevailna/cnb/-/commit/761cb26589902f1ee8d04253dcac780328727eea))
- **(pull)** merge_style 从 String 改为 MergeStyle 枚举 ([21bd2f6](https://cnb.cool/prevailna/cnb/-/commit/21bd2f633b693ffc42b2029011c1fbf8d2b69183))
- **(api)** 提取 paginate 泛型方法消除分页逻辑重复 ([2da4394](https://cnb.cool/prevailna/cnb/-/commit/2da43941260fc6ae18077dd5f3587f18e3791e8c))
- **(pull)** UpdatePullRequest 字段从 String 改为 Option<String> ([9aece6b](https://cnb.cool/prevailna/cnb/-/commit/9aece6ba0d4a9f7c1b7566d69ebf918b361a08ca))
- **(chat)** 将 --do 参数重命名为 --ask 提升语义清晰度 ([7c22862](https://cnb.cool/prevailna/cnb/-/commit/7c228624b3c9051fae90072bd7d69e41ef72f4ab))
- **(cmd)** 为各命令组实现 execute 方法，简化 main.rs 路由 ([7ecd3c9](https://cnb.cool/prevailna/cnb/-/commit/7ecd3c9618f043f321797349eaf89fdb32c2320f))
- **(tui)** 提取 start_of_week 到 cnb-tui::time 公共模块 ([8c5266b](https://cnb.cool/prevailna/cnb/-/commit/8c5266b284ff42be9b6148e960c03155fc7f752c))
- **(api)** 提取 handle_empty_response 消除 7 处重复状态码检查 ([f9423b1](https://cnb.cool/prevailna/cnb/-/commit/f9423b10c54975b3c7d96afc17d9890c34e81cd6))
- **(core)** 提取文件上传通用逻辑到 cnb-core::upload ([a7c291e](https://cnb.cool/prevailna/cnb/-/commit/a7c291e7ed0a6008e757c13a4e926277f503dfe2))
- **(tui)** 提取确认删除流程到 confirm 公共模块 ([4708f4a](https://cnb.cool/prevailna/cnb/-/commit/4708f4ac4b32b137dcbef1afae387514aa3913b1))
- **(tui)** 提取 format_bytes 和 format_rfc3339 到公共模块 ([74c2242](https://cnb.cool/prevailna/cnb/-/commit/74c2242076936d2b742e3b7c83806e8d528b9e7e))
- **(tui)** 提取 TerminalGuard 解决 TUI panic 终端恢复问题 ([bc43e80](https://cnb.cool/prevailna/cnb/-/commit/bc43e801f01504c3778292d3231e3947b25fb67d))

### ⚡️ 性能优化

- **(api)** 添加请求自动重试机制 ([b4b8b12](https://cnb.cool/prevailna/cnb/-/commit/b4b8b12ad4a439c0f369cb4cfb3508bac1b75885))
- 大文件流式上传（替代全量读入内存） ([ad2e447](https://cnb.cool/prevailna/cnb/-/commit/ad2e447dfc6d3a33472effb9ba75fde1bdcb2d32))
- **(chat)** 复用 HTTP 客户端并缓存正则 ([5b2db0a](https://cnb.cool/prevailna/cnb/-/commit/5b2db0a4d246668f2223abe035616dd854653257))
- **(api-upload)** 复用 HTTP 客户端进行附件上传 ([7ae9f22](https://cnb.cool/prevailna/cnb/-/commit/7ae9f22f808e059654fbd0388d6c4ec1621194de))
- **(commit-download)** 并行化附件统计与下载 ([1587118](https://cnb.cool/prevailna/cnb/-/commit/158711877688a0032d2714d4d6ce86f5fbecd9ef))
- **(core)** is_git_dir() 结果用 OnceLock 缓存，避免重复调用 git rev-parse ([f89cd9a](https://cnb.cool/prevailna/cnb/-/commit/f89cd9a02dd8110dd3ffb8d034345c36600dcc29))
- **(download)** 复用 reqwest::Client 替代每次 LFS 下载都新建 ([f79e511](https://cnb.cool/prevailna/cnb/-/commit/f79e5113f39b79b0db24758edf4759ea0180a429))
- **(git)** 使用 LazyLock 缓存正则表达式避免重复编译 ([89627c1](https://cnb.cool/prevailna/cnb/-/commit/89627c1c1090c514c76f51dae04ae8472adcc3d9))
- **(issue)** issue mine 使用 tokio::join 并行请求 ([833d35a](https://cnb.cool/prevailna/cnb/-/commit/833d35ab36ad90ed8a70f80bbdbd9d9905752df1))
- **(api)** 复用无认证 HTTP 客户端替代 post_to_cos 每次新建 ([60bfba4](https://cnb.cool/prevailna/cnb/-/commit/60bfba40e609e67e4442616822b245268b24731c))

### ✅ 测试

- 补充 cnb-api 模块单元测试（40 个） ([cbd365d](https://cnb.cool/prevailna/cnb/-/commit/cbd365d62986ba10ac1fc0dbbb6c7865b1aefa2f))
- **(git)** 补充 parse_git_url 的 8 个单元测试 ([c1453ad](https://cnb.cool/prevailna/cnb/-/commit/c1453addb16fccacb8ea22a8b2ec483ce16469d6))

### ✨ 新功能

- 新增 cnb browse 命令（在浏览器中打开仓库页面） ([af49d21](https://cnb.cool/prevailna/cnb/-/commit/af49d21b459cb164499b188a5b4361e15f08f3ed))
- **(core)** 添加 open crate 依赖和浏览器打开功能 ([45068e3](https://cnb.cool/prevailna/cnb/-/commit/45068e395edaf28738fdc9fca13444df26f85f7c))
- **(issue)** assigners get 命令支持 --json 输出 ([9b9e9f0](https://cnb.cool/prevailna/cnb/-/commit/9b9e9f0921e68d2330181cfc8045ae91384c195f))
- **(cli)** issue/pull/release list 命令支持 --json 输出 ([ba8de67](https://cnb.cool/prevailna/cnb/-/commit/ba8de6773e37d04bb5ada1f800bf2554768dd27a))
- **(cli)** 新增 --json 全局参数（基础设施） ([18dbcb2](https://cnb.cool/prevailna/cnb/-/commit/18dbcb299471ac4ca9309f520d5498c29327f085))
- **(pull)** list 命令新增 --author 和 --reviewer 过滤参数 ([c397af1](https://cnb.cool/prevailna/cnb/-/commit/c397af1b852326ad7c692a41d690ef6ffa93f7fd))
- **(issue)** create 命令新增 --start-date 和 --end-date 参数 ([d5af9bb](https://cnb.cool/prevailna/cnb/-/commit/d5af9bb49ca93209f96a3fd8abf5b1320acd9a51))
- **(pull)** pull create 补充 -t/-b 短选项 ([745d776](https://cnb.cool/prevailna/cnb/-/commit/745d776a16fa39fc4698cfa2afb2c7ef1d381ee0))
- **(tui)** 新增 output 模块，定义统一输出宏 success!/fail!/warn!/info! ([d519b1a](https://cnb.cool/prevailna/cnb/-/commit/d519b1ac53377e669e5dfcaae6c4c0ab49df0fb9))
- **(version)** version 子命令增加 target 平台信息 ([93dd64b](https://cnb.cool/prevailna/cnb/-/commit/93dd64be48bf6288574f137b98952552a13fbb3a))
- **(pull)** pull list 新增 --state 过滤参数并改为并行请求 ([c93e119](https://cnb.cool/prevailna/cnb/-/commit/c93e119d191189a874226de8b5d51c184ace8023))
- **(git)** 支持解析 SSH 格式的 Git remote URL ([ca7fe08](https://cnb.cool/prevailna/cnb/-/commit/ca7fe08c0a9704a71d78a6670e86c4fd28d03294))
- **(knowledge)** 为 knowledge clean 添加确认提示 ([a4e2f69](https://cnb.cool/prevailna/cnb/-/commit/a4e2f699418b7ff6df5f3e515490c2249e89930f))
- **(issue)** 增强 issue close 和 issue create 命令参数 ([ced4e62](https://cnb.cool/prevailna/cnb/-/commit/ced4e62fb86bd93f012cc7ff4e2b88a520e566d4))

### 🎨 代码风格

- 统一中文标点和换行符格式 ([b3b31bd](https://cnb.cool/prevailna/cnb/-/commit/b3b31bd496a5d36db6f0b0b2e6ea8e761cce3cda))
- **(config)** 为 git_protocol 定义独立常量消除语义混淆 ([e47dd15](https://cnb.cool/prevailna/cnb/-/commit/e47dd15d2ccc55ef6ed1dce7a92b2978a09e778e))
- **(release)** 修复 make-latest 默认值并添加 SAFETY 注释 ([25e6b30](https://cnb.cool/prevailna/cnb/-/commit/25e6b3056be9c8c6c9a2a308c00a7b2b6f59508e))

### 🐛 Bug 修复

- **(release)** make_latest 参数类型从 String 改为 bool ([428b82e](https://cnb.cool/prevailna/cnb/-/commit/428b82e8d7229336591f182030a6848d23a96fe3))
- **(api)** 对 URL 中的用户输入参数做 urlencoding 编码 ([a54c348](https://cnb.cool/prevailna/cnb/-/commit/a54c3485ee0ff76fe3343bcba32d97b6cf07db44))
- **(api)** list_issue_comments 新增自动分页，修复超百条评论丢失 ([19427a6](https://cnb.cool/prevailna/cnb/-/commit/19427a6ab0245e2a40a594678506abe098a18b54))
- **(api)** list_star_users 改为自动分页，修复超万条数据静默丢失 ([3555980](https://cnb.cool/prevailna/cnb/-/commit/35559802b72acacded848afc04243d9c5f740286))
- **(release)** release list 改用自动分页避免超过 100 条时丢失数据 ([82e0f29](https://cnb.cool/prevailna/cnb/-/commit/82e0f29df34d3496258310cf2b0ca0da361ce9ac))
- **(config)** 配置文件解析失败时输出警告日志而非静默吞噬 ([47732f1](https://cnb.cool/prevailna/cnb/-/commit/47732f1c5455a64a3f880625a47126f87dd82248))
- **(download)** 将提示信息从 stdout 改为 stderr 输出 ([6c63ffc](https://cnb.cool/prevailna/cnb/-/commit/6c63ffc2b8145487408a2eb26c0ffe23eb771339))
- **(stars)** 过滤时间解析失败的 Star 记录避免图表异常 ([0252c20](https://cnb.cool/prevailna/cnb/-/commit/0252c206974aaa135c42d71266ee8394a82855b2))
- **(issue)** 修复 get_feedback_repo 缺乏防御性编码 ([d230cbb](https://cnb.cool/prevailna/cnb/-/commit/d230cbbb7685ae64196f29dd7a149c74047ae050))
- **(issue)** 区分 Issue 不存在与其他 API 错误 ([ab16aac](https://cnb.cool/prevailna/cnb/-/commit/ab16aac364ae1d6626a7066ef2af422271eaa33b))

## [0.1.0] - 2026-03-05

### ♻️ 重构

- **(core)** 添加 TokenSource 枚举和 get_token_with_source() ([3ef726b](https://cnb.cool/prevailna/cnb/-/commit/3ef726bfb6b80188f4cfb21753e119d687cfd49d))
- **(cmd)** 用 rustyline 替换 stdin 实现交互式输入 ([fb727eb](https://cnb.cool/prevailna/cnb/-/commit/fb727ebefc632b75d196c480e74985022623536d))
- **(cmd)** 重构 issue/pull/release list 使用 Table 组件输出 ([f3e809c](https://cnb.cool/prevailna/cnb/-/commit/f3e809c4fa695099a8d40a4422bbe73f68c3d4af))

### ⚡️ 性能优化

- 添加 Release 构建体积优化配置 ([20661ec](https://cnb.cool/prevailna/cnb/-/commit/20661ec09f893007aad743d62c022120df322c9d))

### ✨ 新功能

- **(cmd)** 实现 config set 子命令 ([a9cdfd1](https://cnb.cool/prevailna/cnb/-/commit/a9cdfd17916053df9ac65fb5070f8c8fffa39834))
- **(cmd)** 实现 config get 子命令 ([6d278da](https://cnb.cool/prevailna/cnb/-/commit/6d278da07b0e03ae2812799910d1e30abda303b3))
- **(cmd)** 实现 config list 子命令 ([6e09036](https://cnb.cool/prevailna/cnb/-/commit/6e09036778eb5b1cacbacb7f6eba1ac9f5d97a14))
- **(core)** Config 添加 get_value/set_value/VALID_KEYS ([4667cc5](https://cnb.cool/prevailna/cnb/-/commit/4667cc5f3e461e0640c859e3f3b5545fc098adff))
- **(cmd)** 实现 auth logout 子命令 ([29f92cf](https://cnb.cool/prevailna/cnb/-/commit/29f92cf0feb0ead8233f9d39a140ca373cd0221f))
- **(cmd)** 实现 auth status 子命令 ([9a2f2dc](https://cnb.cool/prevailna/cnb/-/commit/9a2f2dc743bbd404699ab2526d1a3ea731214c96))
- **(cmd)** 实现 auth login 子命令 ([86faa60](https://cnb.cool/prevailna/cnb/-/commit/86faa60aa8d5f9f78de6aaa33b57a9f11eb6211a))
- **(core)** Config 添加 Serialize 和 save_auth()/remove_auth() ([573e35c](https://cnb.cool/prevailna/cnb/-/commit/573e35cb1524f2a0b669cad87c5d99804c8de1b3))
- **(cmd)** 实现 completion 命令行补全脚本生成 ([47628b1](https://cnb.cool/prevailna/cnb/-/commit/47628b1455acbd63695495dc7d46b536052b63e3))
- **(cmd)** 实现 chat 交互式 REPL 模式 ([a6d4756](https://cnb.cool/prevailna/cnb/-/commit/a6d47566bb6ada04c99cf76319c989c4688efcde))
- **(cmd)** 实现 chat 命令框架（--do 一次性模式 + Agent 循环 + SSE 流式输出） ([59c3dfc](https://cnb.cool/prevailna/cnb/-/commit/59c3dfcb3f3feba25c1b3710f83889791b6ef6a4))
- **(chat)** 实现 OpenAPI Skills 引擎（cnb-chat crate） ([686c3eb](https://cnb.cool/prevailna/cnb/-/commit/686c3eb701de59c1fb98fa04236e862f15d701fb))
- **(api)** 添加 AI Chat Completions API 类型定义和端点 ([67d5fc0](https://cnb.cool/prevailna/cnb/-/commit/67d5fc098baddbbcddc22404eae072d1c2a57a59))
- **(cmd)** 实现 workspace 云原生工作区子命令组 ([7308928](https://cnb.cool/prevailna/cnb/-/commit/73089287254f294acbf35a1d5438a6249adf3d14))
- **(cmd)** 实现 group 组织管理子命令组 ([aa71709](https://cnb.cool/prevailna/cnb/-/commit/aa71709726696c945e5621f67143bb3fc0688848))
- **(cmd)** 实现 knowledge 知识库子命令组（list-models/info/clean/query） ([d1089c8](https://cnb.cool/prevailna/cnb/-/commit/d1089c8024271b7da9e5215af80e496708d8e1d9))
- **(api)** 添加 Knowledge 知识库 API 类型和端点 ([5b96d76](https://cnb.cool/prevailna/cnb/-/commit/5b96d76a285245b7f1aeea8577e71ab8163d865b))
- **(cmd)** 实现 stars Star 趋势图（ratatui 累积折线图） ([5ee9c52](https://cnb.cool/prevailna/cnb/-/commit/5ee9c52ba366cb20c18bfcab0e66c8946a5bc298))
- **(cmd)** 实现 stats 提交统计仪表盘（ratatui 排行榜 + 折线图） ([b7644c1](https://cnb.cool/prevailna/cnb/-/commit/b7644c11b6078ff5118dfa9800d74ad55fff9636))
- **(cmd)** download 命令添加 include/exclude glob 过滤支持 ([10e2040](https://cnb.cool/prevailna/cnb/-/commit/10e2040125ea6d43e65922b5373da59892bef0fe))
- **(cmd)** download 命令添加并发下载 + 进度条 + LFS 流式下载 ([cfc2373](https://cnb.cool/prevailna/cnb/-/commit/cfc2373678446d29f74ab05a5ba4519d4d91f88c))
- **(cmd)** 实现 download 命令基础框架（API 类型 + blob 文件下载） ([ede40c7](https://cnb.cool/prevailna/cnb/-/commit/ede40c7806e241d48e36d82c651b83eb234d62e5))
- **(tui)** 实现表格输出组件，支持 CJK 宽字符和自动截断 ([a15cea0](https://cnb.cool/prevailna/cnb/-/commit/a15cea08126b188103555a7b86581195fe9f30b4))
- **(cmd)** 实现 cnb commit asset-upload 子命令 ([ed650a4](https://cnb.cool/prevailna/cnb/-/commit/ed650a41d668268b8d7a704472811111c6ceffdc))
- **(cmd)** 实现 cnb commit asset-clean 子命令 ([689b2d8](https://cnb.cool/prevailna/cnb/-/commit/689b2d8e498466ad8dc339b133e35491e422e3ae))
- **(cmd)** 实现 cnb commit asset-stats 子命令 ([aa2b20d](https://cnb.cool/prevailna/cnb/-/commit/aa2b20d9c89c0405037b0c7b582a1423c1f21cf3))
- **(api)** 添加 Commit API 类型定义和端点封装 ([22ba6e1](https://cnb.cool/prevailna/cnb/-/commit/22ba6e1dc888a66264ea71c3abf6f28d5df1be34))
- **(cmd)** 实现 cnb release asset-upload 子命令 ([791714b](https://cnb.cool/prevailna/cnb/-/commit/791714bd9708bb8ad1db28b974a43a99cfe6ac6c))
- **(cmd)** 实现 cnb release asset-clean 子命令 ([36079b1](https://cnb.cool/prevailna/cnb/-/commit/36079b16ee1e7a9887366a18b8f25031f2db9275))
- **(cmd)** 实现 cnb release asset-stats 子命令 ([ac606fa](https://cnb.cool/prevailna/cnb/-/commit/ac606fa35105d38a0dd8d9abb7bf877856c0e7ae))
- **(cmd)** 实现 cnb release create 子命令 ([54e62b2](https://cnb.cool/prevailna/cnb/-/commit/54e62b2a710431a8900d3463f4c2f37ea2ae5879))
- **(cmd)** 实现 cnb release list 子命令 ([f9c99a6](https://cnb.cool/prevailna/cnb/-/commit/f9c99a6a55a3c2cdafce7ddbb6d74c6c5a03f803))
- **(api)** 添加 Release API 端点封装 ([68f76b4](https://cnb.cool/prevailna/cnb/-/commit/68f76b4c3dce8622e4d2054a53de628b88ee3140))
- **(api)** 添加 Release 相关类型定义 ([03662dc](https://cnb.cool/prevailna/cnb/-/commit/03662dc3135906f164f30db6fa4240c1ab5c36d7))
- **(cmd)** 实现 cnb pull update 和 cnb pull merge 子命令 ([2bf89ff](https://cnb.cool/prevailna/cnb/-/commit/2bf89ff72eccc0d83dd98540550431eaf5eca6d8))
- **(cmd)** 实现 cnb pull create 子命令 ([f8e7ce0](https://cnb.cool/prevailna/cnb/-/commit/f8e7ce0a31bf3642015411d3092096e0bf257476))
- **(cmd)** 实现 cnb pull list 子命令 ([64f14fb](https://cnb.cool/prevailna/cnb/-/commit/64f14fb91afa77ea3dae76628ebe4fdd22673388))
- **(api)** 添加 Pull Request API 端点封装 ([7045181](https://cnb.cool/prevailna/cnb/-/commit/7045181a25135cab99f0e1b5696f03d23d75a74a))
- **(api)** 添加 Pull Request 相关类型定义 ([142f438](https://cnb.cool/prevailna/cnb/-/commit/142f4388d75de4f608e364823e5891f72fccf89c))
- **(cmd)** 实现 cnb issue assigners 子命令 ([ee48ba0](https://cnb.cool/prevailna/cnb/-/commit/ee48ba065098e92bf2c341513421e568a6d320d1))
- **(cmd)** 实现 cnb issue download 子命令 ([0ea4f26](https://cnb.cool/prevailna/cnb/-/commit/0ea4f261b43458e9fab1e2feaa031c3f0656db20))
- **(cmd)** 实现 cnb issue exist 子命令 ([80f5fd8](https://cnb.cool/prevailna/cnb/-/commit/80f5fd871c5ec5085ffbe95ab6678930e62ac89f))
- **(cmd)** 实现 cnb issue comment 子命令 ([367f8dd](https://cnb.cool/prevailna/cnb/-/commit/367f8dd3e7d1c00cd77d8ae37929c0ca5c73cbad))
- **(cmd)** 实现 cnb issue close 子命令 ([592784c](https://cnb.cool/prevailna/cnb/-/commit/592784ca83b062eda4fe13c13777b4ff16f489fc))
- **(cmd)** 实现 cnb issue create 子命令 ([545a7a6](https://cnb.cool/prevailna/cnb/-/commit/545a7a6d7cf9728ae62b3edd192f4a357817dfd8))
- **(cmd)** 实现 cnb issue mine 子命令 ([9ebc41b](https://cnb.cool/prevailna/cnb/-/commit/9ebc41bdd840a71e1ae7ee296dc62e4c8b712171))
- **(cmd)** 实现 cnb issue list 子命令 ([40d9386](https://cnb.cool/prevailna/cnb/-/commit/40d93867457d5726d6626647ea157efa2f4c05e8))
- **(api)** 添加 Issue API 端点封装 ([351c73c](https://cnb.cool/prevailna/cnb/-/commit/351c73c0d762a48937a6dac08c568ec5394bab0d))
- **(api)** 添加 Issue 相关类型定义 ([f5bc3b0](https://cnb.cool/prevailna/cnb/-/commit/f5bc3b049e20ce53626d06aff75c47291f65d1c7))
- **(cli)** 添加 Windows UTF-8 控制台输出支持 ([5119b5e](https://cnb.cool/prevailna/cnb/-/commit/5119b5ec8ee1938c26601bee0a20a9d4b28362ff))
- **(cmd)** 添加 AppContext 懒加载和 cnb info 命令 ([ed2de5f](https://cnb.cool/prevailna/cnb/-/commit/ed2de5f9498b2c805d6650bfa3806615ee40700d))
- **(git)** 添加 Git 本地操作封装 ([166c78f](https://cnb.cool/prevailna/cnb/-/commit/166c78fc64ab1a81c4a04ffbfd318d697d9063d9))
- **(api)** 添加 CNB API 客户端和类型定义 ([02e63d6](https://cnb.cool/prevailna/cnb/-/commit/02e63d6567f737a50792d1a1555e2abef3eb04d0))
- **(auth)** 添加三级 Token 认证系统 ([4a0a98a](https://cnb.cool/prevailna/cnb/-/commit/4a0a98a585fd9ce5aa4526d704a47159e5703ce2))
- **(config)** 添加 TOML 配置系统 ([43a62aa](https://cnb.cool/prevailna/cnb/-/commit/43a62aa66f2b7b3c249d1ad378af1f449285ff97))
- **(cli)** 添加 clap CLI 定义和全局参数 ([7f8ed9d](https://cnb.cool/prevailna/cnb/-/commit/7f8ed9dc330efcc93d2fb42aca82f6dd18ca6bcc))

### 🌀 其他

- 创建 Cargo Workspace 骨架和空 crate 结构 ([1658001](https://cnb.cool/prevailna/cnb/-/commit/1658001c99e09c9fde7375818bd728b0c2db8c00))

### 🐛 Bug 修复

- **(ci)** 修复文档构建缺少 git 导致 VitePress 构建失败 ([67215d7](https://cnb.cool/prevailna/cnb/-/commit/67215d7b632727da83bbdddacec1fd8b6c43e00a))
- **(cmd)** 修复中文字符串截断导致的 UTF-8 panic ([a8b5d55](https://cnb.cool/prevailna/cnb/-/commit/a8b5d5553cb40cc5b9f765681dbc0d48511d7e88))
- **(api)** 优化 API 错误提示和 401 认证引导 ([b9e604f](https://cnb.cool/prevailna/cnb/-/commit/b9e604fcba564ad410956a99b064b64982f05e04))

### 👷 CI/CD

- **(docs)** 重构流水线配置，拆分部署工作流到独立文件 ([34bbed5](https://cnb.cool/prevailna/cnb/-/commit/34bbed59cbcb3aeb9e329e6406855d435ef7a460))
- **(docs)** 配置 CNB 流水线部署 VitePress 文档到 Cloudflare Pages ([5f0b060](https://cnb.cool/prevailna/cnb/-/commit/5f0b0608e5b32ed3f43bd86d882642c50d831a49))

### 📝 文档

- **(readme)** 精简 NOTE 提示，移除非官方声明 ([a5787e9](https://cnb.cool/prevailna/cnb/-/commit/a5787e9686c2521b5c0e0d1b1e436063db0f00a8))
- **(readme)** 在 NOTE 中补充官方 CLI 工具链接 ([6234254](https://cnb.cool/prevailna/cnb/-/commit/6234254a5c4b0b2d3357edcb75e4f0ed5eae0995))
- **(readme)** 重写 README，居中布局 + 多 badge + 快速跳转链接 ([f86a860](https://cnb.cool/prevailna/cnb/-/commit/f86a8605d162024165819fe6e22c3a9a3e4a2424))
- **(readme)** 重写 README，标注为非官方社区项目 ([1a5244d](https://cnb.cool/prevailna/cnb/-/commit/1a5244ddfd3da7060d886a5c660fd46358fc3097))
- **(readme)** 生成项目 README ([acc1a33](https://cnb.cool/prevailna/cnb/-/commit/acc1a333c4d0b2207d8cb745a563f2f63bf19b4c))
- **(site)** 更新站点配置和命令参考主页 ([abdd715](https://cnb.cool/prevailna/cnb/-/commit/abdd715e2b58cb53542f72a2de9eaadf38a97ed9))
- **(workspace)** 生成 workspace 子命令文档 ([5b8cd06](https://cnb.cool/prevailna/cnb/-/commit/5b8cd06bb88c15d11ab01f0a0ec94180b4ad31e4))
- **(knowledge)** 生成 knowledge 子命令文档 ([b0dcbf0](https://cnb.cool/prevailna/cnb/-/commit/b0dcbf062f635a6b1a79ed3c7286c74549d84129))
- **(group)** 生成 group 子命令文档 ([95ee6db](https://cnb.cool/prevailna/cnb/-/commit/95ee6db30afa276b113d54a0e93cb16258110b83))
- **(misc)** 生成 info、stats、stars、completion、version 命令文档 ([5863381](https://cnb.cool/prevailna/cnb/-/commit/58633815d2b39f405750f25238abe7c831803be0))
- **(download)** 生成 download 命令文档 ([e01acf2](https://cnb.cool/prevailna/cnb/-/commit/e01acf2166c7aaefa543404b9ac413dddba6e207))
- **(commit)** 生成 commit 子命令文档 ([8ce49c4](https://cnb.cool/prevailna/cnb/-/commit/8ce49c4552a0026a003cff64f99f341368464e7d))
- **(release)** 生成 release 子命令文档 ([56b71f3](https://cnb.cool/prevailna/cnb/-/commit/56b71f31d24e452e4cb954c3609cacb902a538fc))
- **(pull)** 生成 pull 子命令文档 ([4df8868](https://cnb.cool/prevailna/cnb/-/commit/4df8868ab7f942e314c3a0c7cf2d7550d8ec0a2b))
- **(issue)** 生成 issue 子命令文档 ([484c921](https://cnb.cool/prevailna/cnb/-/commit/484c92192e673267c6a5a37cfdeece88c2d464b4))
- **(config)** 生成 config 子命令文档 ([31115ac](https://cnb.cool/prevailna/cnb/-/commit/31115ace542cd7709599097112ed7e38deba39b5))
- **(chat)** 生成 chat 命令文档 ([c4dfc3a](https://cnb.cool/prevailna/cnb/-/commit/c4dfc3af299907b12be43b74774f22f64edb78f2))
- **(auth)** 生成 auth 子命令文档 ([8b99172](https://cnb.cool/prevailna/cnb/-/commit/8b991727017f8ec8cb97c9978f2f17c5b1788089))
- 搭建 VitePress 文档站点雏形 ([0861256](https://cnb.cool/prevailna/cnb/-/commit/086125661dd96c85c7cdf8ebd31345c93315c054))
- **(issue)** 添加 Issue 命令参考文档 ([c27e13b](https://cnb.cool/prevailna/cnb/-/commit/c27e13b480c431a6d17b157b3a8af039228bbd2d))

### 🔧 杂项

- 更新 .gitignore 忽略 IDE 和工具目录 ([641663b](https://cnb.cool/prevailna/cnb/-/commit/641663b003dad21c841e678a430229616651e7d1))
- **(license)** 添加 Apache-2.0 许可证文件 ([88c6ef7](https://cnb.cool/prevailna/cnb/-/commit/88c6ef7012d7c9b6fc2284b3ee81828c59e325ab))
