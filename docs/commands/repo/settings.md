# cnb-rs repo settings

```
cnb-rs repo settings <subcommand>
```

管理仓库的 Git 行为设置，包括分支保护规则、合并请求设置、推送限制和流水线构建设置。

## 子命令

- [branch-protection](#branch-protection) — 分支保护规则管理
- [pull-request](#pull-request) — 合并请求设置
- [push-limit](#push-limit) — 推送限制设置
- [pipeline](#pipeline) — 流水线构建设置

---

## branch-protection

```
cnb-rs repo settings branch-protection <list|get|create|update|delete>
```

管理仓库分支保护规则。

### `list`

```bash
# 列出分支保护规则
$ cnb-rs repo settings branch-protection list
$ cnb-rs repo settings branch-protection list org/repo

# JSON 输出
$ cnb-rs repo settings branch-protection list --json
```

### `get <id>`

```bash
# 查看规则详情
$ cnb-rs repo settings branch-protection get bp-001
$ cnb-rs repo settings branch-protection get bp-001 --repo org/repo
```

### `create`

```bash
# 创建 main 分支保护：管理员推送、需 2 人评审、需状态检查
$ cnb-rs repo settings branch-protection create \
    --rule "main" \
    --allow-master-pushes \
    --require-pr \
    --require-review --review-count 2 \
    --require-master-approve \
    --require-status-checks

# 创建通配符规则
$ cnb-rs repo settings branch-protection create \
    --rule "release/*" \
    --require-pr \
    --require-review --review-count 1
```

**选项：**

- `-r, --rule <PATTERN>`: 规则名称（支持通配符，必填）
- `--allow-pushes`: 允许所有人推送
- `--allow-master-pushes`: 仅允许管理员推送
- `--allow-force-pushes`: 允许所有人强推
- `--allow-master-force-pushes`: 仅允许管理员强推
- `--require-pr`: 必须通过 PR 推送
- `--require-review`: 需要代码评审
- `--review-count <N>`: 评审者数量（1-5）
- `--review-ratio <N>`: 评审通过率（1-100）
- `--require-master-approve`: 需管理员批准
- `--require-status-checks`: 需通过状态检查
- `--require-linear-history`: 仅允许线性提交

### `update <id>`

```bash
$ cnb-rs repo settings branch-protection update bp-001 \
    --review-count 3 --review-ratio 100
```

参数与 `create` 相同，所有参数均为可选。

### `delete <id>`

```bash
$ cnb-rs repo settings branch-protection delete bp-003
$ cnb-rs repo settings branch-protection delete bp-003 --yes
```

**选项：** `--yes`, `-y` 跳过确认提示

---

## pull-request

```
cnb-rs repo settings pull-request <get|set>
```

管理合并请求（PR）设置。

### `get`

```bash
$ cnb-rs repo settings pull-request get
$ cnb-rs repo settings pull-request get --json
```

### `set`

```bash
# 仅允许压缩合并，使用 PR 标题作为提交信息
$ cnb-rs repo settings pull-request set \
    --allow-merge-commit=false \
    --allow-rebase=false \
    --allow-squash \
    --squash-message-style pull_request_title
```

**选项：**

- `--allow-merge-commit`: 允许直接提交合并
- `--allow-rebase`: 允许变基合并
- `--allow-squash`: 允许压缩合并
- `--auto-reviewer`: 自动添加管理员为评审者
- `--merge-message-style <STYLE>`: 直接合并提交信息风格
- `--squash-message-style <STYLE>`: 压缩合并提交信息风格

**提交信息风格可选值：** `default`, `pull_request_title`, `pull_request_title_with_body`

---

## push-limit

```
cnb-rs repo settings push-limit <get|set>
```

管理推送限制设置。

### `get`

```bash
$ cnb-rs repo settings push-limit get
$ cnb-rs repo settings push-limit get --json
```

### `set`

```bash
# 限制单次推送最多 20 个分支，提交者必须是推送者本人
$ cnb-rs repo settings push-limit set \
    --check-push-number \
    --max-push-number 20 \
    --commit-must-be pusher

# 仅管理员可管理标签
$ cnb-rs repo settings push-limit set --master-only-tag
```

**选项：**

- `--check-push-number`: 开启单次推送数量限制
- `--max-push-number <N>`: 单次推送最大数量
- `--master-only-tag`: 仅管理员可推送/删除标签和版本
- `--commit-must-be <POLICY>`: 提交检查策略：`any`、`registered`、`pusher`

---

## pipeline

```
cnb-rs repo settings pipeline <get|set>
```

管理流水线（云原生构建）设置。

### `get`

```bash
$ cnb-rs repo settings pipeline get
$ cnb-rs repo settings pipeline get --json
```

### `set`

```bash
# 开启自动构建，但禁止 Fork 仓库自动触发
$ cnb-rs repo settings pipeline set \
    --auto-trigger \
    --fork-auto-trigger=false
```

**选项：**

- `--auto-trigger`: 按 .cnb.yml 自动触发构建
- `--fork-auto-trigger`: 允许 Fork 仓库自动触发构建

---

## API

| 子命令                     | API                                          | 方法   | 权限             |
| -------------------------- | -------------------------------------------- | ------ | ---------------- |
| `branch-protection list`   | `/{repo}/-/settings/branch-protections`      | GET    | `repo-manage:r`  |
| `branch-protection get`    | `/{repo}/-/settings/branch-protections/{id}` | GET    | `repo-manage:r`  |
| `branch-protection create` | `/{repo}/-/settings/branch-protections`      | POST   | `repo-manage:rw` |
| `branch-protection update` | `/{repo}/-/settings/branch-protections/{id}` | PATCH  | `repo-manage:rw` |
| `branch-protection delete` | `/{repo}/-/settings/branch-protections/{id}` | DELETE | `repo-manage:rw` |
| `pull-request get`         | `/{repo}/-/settings/pull-request`            | GET    | `repo-manage:r`  |
| `pull-request set`         | `/{repo}/-/settings/pull-request`            | PUT    | `repo-manage:rw` |
| `push-limit get`           | `/{repo}/-/settings/push-limit`              | GET    | `repo-manage:r`  |
| `push-limit set`           | `/{repo}/-/settings/push-limit`              | PUT    | `repo-manage:rw` |
| `pipeline get`             | `/{repo}/-/settings/cloud-native-build`      | GET    | `repo-manage:r`  |
| `pipeline set`             | `/{repo}/-/settings/cloud-native-build`      | PUT    | `repo-manage:rw` |

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo edit](/repo/edit)
