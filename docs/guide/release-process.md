# 发布流程

`cnb` 项目当前采用“准备发版 PR -> 合并后自动打 tag -> CNB 创建 release 并同步 GitHub -> GitHub 构建并回填附件”的四段式流程：

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

1. 在 GitHub 原生 runner 上分别构建 Linux、Windows、macOS 产物；当前默认发布矩阵包含 Linux 4 个目标、Windows 4 个目标（GNU 1 个 x86_64 变体 + gnullvm 1 个 aarch64 变体，MSVC 2 个 x86_64 / aarch64 变体）和 macOS 2 个目标
2. 生成与 CNB 一致的 `LATEST_CHANGELOG.md`
3. 基于最终压缩包生成 `sha256sum.txt`
4. 上传附件和 `sha256sum.txt` 到 GitHub Release
5. 先删除 CNB 对应 Release 的旧附件，再回填同一批新文件

回填完成后，GitHub Release 和 CNB Release 会持有同一组二进制产物以及对应的 SHA-256 校验文件。

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

## 必需配置

- CNB 侧默认通过 key repo `imports` 引入以下变量：
  - `https://cnb.cool/prevailna/secrets/-/blob/main/github-sync/cnb-cli-rs.yml`
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
