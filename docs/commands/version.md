# cnb-rs version

```
cnb-rs version
```

显示 cnb-rs 的详细版本信息，包括版本号、Git commit hash、提交日期、Rust 编译器版本、目标平台和构建 profile。

所有信息在编译期通过 `build.rs` 嵌入，不涉及网络请求。

## 输出示例

```bash
$ cnb-rs version
cnb-rs 0.1.0 (a5eec5d 2026-02-26)
rustc 1.93.1 (01f6ddf75 2026-02-11)
target: x86_64-pc-windows-msvc
profile: release
```

## 输出说明

| 行                               | 说明                                   |
| -------------------------------- | -------------------------------------- |
| `cnb-rs 0.1.0 (a5eec5d 2026-02-26)` | 版本号 + Git commit 短 hash + 提交日期 |
| `rustc 1.93.1 (...)`             | Rust 编译器版本                        |
| `target: ...`                    | 编译目标平台三元组                     |
| `profile: release`               | 构建 profile（debug / release）        |

## 另请参阅

- [cnb-rs](/guide/cnb)
