# 发布流程

`cnb-rs` 项目当前采用“准备发版 PR -> 合并后自动打 tag -> CNB 创建 release 并同步 GitHub -> GitHub 构建并回填附件”的四段式流程：

- `release prepare` 负责自动推导下一个版本号、更新 `Cargo.toml` 与 `CHANGELOG.md`、创建 release PR
- `main push` 负责在 release PR 合并后先同步 GitHub 镜像仓库的 `main` 分支，再自动创建对应 tag
- `tag release` 负责在 CNB 校验标签、生成发布说明、创建 CNB Release，并只向 GitHub 推送当前 release tag
- `github release assets` 负责在 GitHub 原生 runner 上构建 Linux / Windows / macOS 附件，上传到 GitHub Release，并回填到 CNB Release

这样既兼容受保护的 `main` 分支，也避免继续在 CNB 的 Linux 容器里承担 Windows / macOS 的最终发布构建。

## 1. 准备 release PR

在 `main` 分支页面点击 `准备 release PR` 按钮后，流水线会自动执行：

1. 读取当前 `Cargo.toml` 中的 workspace version
2. 查找对应的 `v<current_version>` 标签
3. 基于该标签以来的提交自动推导版本升级级别
4. 自动 bump 到下一个版本
5. 重新生成项目级 `CHANGELOG.md`
6. 推送 `release/vX.Y.Z` 分支
7. 自动创建指向 `main` 的 release PR

版本升级规则：

- 检测到 `BREAKING CHANGE` 或 `type!:` 时视为 `major`
- 检测到 `feat` 时视为 `minor`
- 其他提交默认视为 `patch`
- 对 `0.x` 版本，即使检测到 `major`，也会按 `minor` 处理，避免直接跳到 `1.0.0`

release PR 仍然需要正常 review；`CHANGELOG.md` 作为项目历史文档，会在这个 PR 中被审阅后进入 `main`。

## 2. main 合并后自动打 tag

release PR 合并到 `main` 后，`main push` 流水线会自动执行：

1. 运行 `rustfmt`、`cargo check`、`cargo clippy`、`cargo test`
2. 在代码检查通过后，将 GitHub 镜像仓库的 `main` 分支同步到最新状态
3. 在 `main` 已完成 GitHub 同步后，检查本次提交是否同时修改了 `Cargo.toml` 与 `CHANGELOG.md`
4. 如果当前 workspace version 对应的 tag 不存在，则自动创建 `vX.Y.Z`

这一步不再需要手工执行 `git tag` 和 `git push origin <tag>`。

CNB 会单独维护一个 `rust-ci` 镜像，用于预热 Linux 校验依赖，并在 `Cargo.lock` 变更时重建。`fmt/check/clippy/test`、自动打 tag、release prepare、pre-release-check、CNB release 说明生成等流程默认都消费这张镜像。

## 3. tag 在 CNB 创建 release 并同步到 GitHub

标签创建后，CNB 的 `tag_push` 流水线会自动执行以下步骤：

1. 运行发版前检查
2. 校验标签格式与 `Cargo.toml` 版本一致
3. 运行 release 镜像内的 `cargo test --workspace --target x86_64-unknown-linux-gnu`
4. 生成本次发布说明 `LATEST_CHANGELOG.md`
5. 创建或更新 CNB Release
6. 只将当前 release tag 推送到 GitHub 镜像仓库

这样可以避免 `tag_push` 一次性夹带 `main` 分支同步，从而让 GitHub 侧的 `push.tags: v*` 触发更稳定。

## 4. GitHub 构建并回填 release 附件

GitHub 镜像仓库收到 `v*` tag 后，会触发 `.github/workflows/build.yml`：

1. 在 GitHub 原生 runner 上分别构建 Linux、Windows、macOS 产物；当前默认发布矩阵包含 Linux 4 个目标、Windows 4 个目标（GNU 1 个 x86_64 变体 + gnullvm 1 个 aarch64 实验性变体，MSVC 2 个 x86_64 / aarch64 变体）和 macOS 2 个目标
2. 对 `x86_64-unknown-linux-gnu` 和 `aarch64-unknown-linux-gnu`，除 `.tar.gz` 外，还会额外生成 Linux 原生包：`.deb` 和 `.rpm`
3. 对 `x86_64-pc-windows-msvc`、`aarch64-pc-windows-msvc` 和 `x86_64-pc-windows-gnu`，除 `.zip` 外，还会额外生成 Windows 原生安装包：`.msi`
4. 对 Linux 原生包执行等价发布校验，确认包架构字段和内置文件列表符合预期
5. 生成与 CNB 一致的 `LATEST_CHANGELOG.md`
6. 基于最终 release 附件生成 `sha256sum.txt`，其中包含 `.tar.gz`、`.zip`、`.msi`、`.deb`、`.rpm`
7. 上传附件和 `sha256sum.txt` 到 GitHub Release
8. 先删除 CNB 对应 Release 的旧附件，再回填同一批新文件

`aarch64-pc-windows-gnullvm` 当前仍被视为实验性非阻塞目标。它不再走 `cross` 的默认镜像路径，而是在 Ubuntu runner 上通过单独的 cross toolchain setup 准备 LLVM MinGW / gnullvm 工具链；如果它单独构建失败，GitHub Release 与 CNB Release 仍会继续发布其余成功产物，因此该目标的附件可能暂时缺席。

当前 Linux 原生包的发布范围说明：

- 当前对 `x86_64-unknown-linux-gnu` 和 `aarch64-unknown-linux-gnu` 发布 `.deb` / `.rpm`
- 其他 Linux 目标当前仍仅提供 `.tar.gz`
- 当前仍未提供 apt / yum 软件源，用户通过 release 页面下载 `.deb` / `.rpm` 安装

当前 Windows 原生安装包的发布范围说明：

- 当前对 `x86_64-pc-windows-msvc`、`aarch64-pc-windows-msvc` 和 `x86_64-pc-windows-gnu` 发布 `.msi`
- `aarch64-pc-windows-gnullvm` 当前仍仅提供 `.zip`
- 当前 `.msi` 与 `.zip` 会同时保留，避免影响已有使用方式

回填完成后，GitHub Release 和 CNB Release 会持有同一组二进制产物以及对应的 SHA-256 校验文件。

## CLI 改名类 breaking change 约定

像 `cnb -> cnb-rs` 这种会直接影响用户调用方式的变更，不应仅仅作为普通重构处理。后续如果再出现类似 breaking change，release PR 和正式发布时至少需要满足下面 4 点：

1. 在 release PR 中显式审阅迁移影响，而不是只看代码是否能编译通过
2. 在 `CHANGELOG.md` 顶部增加单独的迁移提示，明确写出旧命令与新命令的对应关系
3. 在 release notes 中重复同样的迁移提示，避免用户只看发布页时遗漏 breaking change
4. 如果希望继续支持旧输入习惯，应通过外部分发仓库的 note、wrapper 或安装说明处理，不在主程序里恢复旧入口

对 `cnb-rs` 这次改名，推荐在紧随其后的正式 release 中至少写清：

- `cnb ...` 现已改为 `cnb-rs ...`
- 发布产物文件名已从 `cnb-*` 改为 `cnb-rs-*`
- 如需保留旧习惯，请自行配置 alias，或参照外部分发仓库的安装说明

## 标签规则

发布标签必须满足以下要求：

- 以 `v` 开头
- 符合 `vX.Y.Z`、`vX.Y.Z-alpha.1`、`vX.Y.Z-beta.1` 这类格式
- 去掉前缀 `v` 后，版本号必须与根目录 `Cargo.toml` 中的 workspace version 完全一致

不符合规则的标签会在发版前检查阶段直接失败。

## CHANGELOG 约定

项目中有两类 changelog 产物：

- `CHANGELOG.md`
  - 项目级历史文档
  - 在 `release-prepare` 流水线中生成并提交到 release PR
  - 进入 `main` 前需要审阅
- `LATEST_CHANGELOG.md`
  - 单次发布说明
  - 由 CNB tag 发布流水线和 GitHub tag workflow 按同一规则临时生成
  - 作为 Release 描述使用，不回写仓库

## Issue 版本标签约定

对于明确承诺“随某个版本交付”的 issue，使用版本标签管理，而不是在 PR 合并后立即关闭：

- `target:vX.Y.Z`
  - 表示计划在该版本交付
  - issue 在开发过程中保持打开
- `released:vX.Y.Z`
  - 表示已经确认随该版本发布
  - 一般在版本真正发布成功后，由人工添加

辅助标签约定：

- `epic`
- `tracking`
- `keep-open`

带上述辅助标签的 issue 默认不因为 release 直接关闭，即使它们关联了版本标签，也更适合作为长期跟踪项保留。

首次使用某个版本标签前，先在仓库里创建对应 label，避免后续 `issue-add` 时因为标签不存在而中断。推荐颜色：

```bash
cnb-rs --repo wwvo/cnb-rs/cnb-rs label create -n "target:v0.4.6" -c "1d76db" -d "计划在 v0.4.6 交付"
cnb-rs --repo wwvo/cnb-rs/cnb-rs label create -n "released:v0.4.6" -c "2da44e" -d "已随 v0.4.6 发布"
```

## Issue / PR 使用约定

- issue 创建或 triage 时，如果已经确定目标版本，就为 issue 添加 `target:vX.Y.Z`
- PR 合并到 `main` 不等于 issue 应立即关闭；对于 release 相关或用户可见交付，优先等正式版本发布后再确认关闭
- 只把版本标签用于明确属于某个版本交付的 issue；不要给长期 epic、tracking、纯重构或日常 cleanup 强行挂版本标签

## Release 后 issue 收口

版本真正发布成功后，建议按以下顺序整理 issue 收口候选，再交给后续 release 收口流水线处理：

1. 在 CNB 页面点击 `生成 release issue 候选清单`
2. 输入已经成功发布的稳定版 tag，例如 `v0.4.6`
3. 查看当前 `target:v0.4.6` 的 open / closed issue 列表
4. 对已随该版本交付的 issue：
   - 添加 `released:v0.4.6`
   - 移除 `target:v0.4.6`
   - 交给 release 收口流水线评论并关闭
5. 对未赶上该版本的 issue：
   - 移除 `target:v0.4.6`
   - 迁移到下一个目标版本标签，例如 `target:v0.4.7`

推荐由流水线使用的关闭评论模板：

```md
已随 `v0.4.6` 正式发布，关闭。

- CNB Release: <链接>
- GitHub Release: <链接>
```

如果只是留档不关闭，推荐评论模板：

```md
该 issue 关联的改动已包含在 `v0.4.6` 中，当前保持打开。
```

候选清单阶段建议直接使用仓库内置 CLI 做标签整理：

```bash
cnb-rs --repo wwvo/cnb-rs/cnb-rs label issue-add <NUMBER> -l "released:v0.4.6"
cnb-rs --repo wwvo/cnb-rs/cnb-rs label issue-remove <NUMBER> "target:v0.4.6"
```

## 辅助流水线

仓库提供了一个手动按钮 `生成 release issue 候选清单`，用途是生成收口候选清单，辅助后续流水线收口：

- 输入 `RELEASE_TAG` 后，流水线会列出当前 `target:vX.Y.Z` 的 open / closed issue 清单
- 可选输入 `NEXT_TARGET_TAG`，用于在日志中生成更明确的迁移提示
- 流水线开始前会先校验对应的 CNB / GitHub Release 是否已经存在；如果 release 还没准备好，直接失败
- 流水线本身不会自动评论、改标签或关闭 issue；它只负责生成候选清单与操作提示

这条辅助流水线的目标是让 release 管理更清楚，并避免再以“手工 close issue”的方式收口主仓库 issue。

## 必需配置

- CNB 侧默认通过 key repo `imports` 引入以下变量：
  - `https://cnb.cool/prevailna/secrets/-/blob/main/github-sync/cnb-rs.yml`
  - `GITHUB_SYNC_TARGET_URL`
  - `GITHUB_SYNC_USERNAME`
  - `GITHUB_SYNC_TOKEN`
- GitHub 侧需要配置仓库 secret：
  - `CNB_TOKEN`
    - 需具备 CNB `repo-contents` 读写权限，用于回填 Release 附件

## 重跑约定

- CNB 的 `git:release` 采用覆盖更新模式，避免重跑时删除已有 Release 元数据
- GitHub 在回填 CNB 附件前会先删除同 tag 下的旧附件，再上传新文件，保证 release 重跑后的产物集保持一致

## 为什么 release 流程要拆成四段

仓库的 `main` 分支启用了保护策略。如果在正式 release 流水线中直接修改或推送 `main`，一旦分支保护拦截，就会连带阻塞附件上传。

拆成“release PR / main 自动打 tag / CNB 创建 release 并同步 GitHub / GitHub 原生构建附件”之后：

- 项目级变更文档仍然通过 PR 审阅进入 `main`
- CNB 保持版本编排和 Release 生命周期控制
- GitHub 负责真正适合其 runner 的多平台附件构建
- release PR 合并后无需再手工打 tag
