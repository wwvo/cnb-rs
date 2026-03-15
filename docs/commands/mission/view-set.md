# cnb-rs mission view set

```
cnb-rs mission view set <mission> --file <path>
cnb-rs mission view set <mission> --stdin
```

通过 JSON 文件或标准输入设置视图配置。

支持两种输入方式：通过 `--file` 指定 JSON 文件，或通过 `--stdin` 从标准输入读取。

## 选项

- `<mission>`: 任务集路径，格式 `group/mission`（必填）
- `--file <PATH>`: 从 JSON 文件读取配置
- `--stdin`: 从标准输入读取配置

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 从文件读取配置
$ cnb-rs mission view set my-org/sprint-2025-q1 --file view-config.json

# 从标准输入读取
$ cat view-config.json | cnb-rs mission view set my-org/sprint-2025-q1 --stdin
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission view get](/mission/view-get)
- [cnb-rs mission view add](/mission/view-add)
