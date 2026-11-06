# mold

Shuyra 的宏兼容门面。`src/lib.rs` 重导出 Kernel 的 `kernel-api-macros`；
`entry/` 独立实现程序入口宏 `mold-entry`。

公开路径为 `Frame`、`Envcall`、`Fail`、`WireCodes`、`interface`、`contract`、`entry`。
前六者的实现与 ABI 定义归 Kernel；本仓只拥有 `entry`。
宏展开所用的 `env`、`wire` 和 `execution::boot::entry` 路径由调用者提供。
`Envcall` 生成载荷与返回编解码，实际执行调用由 Kernel Client 提供。

`tests/` 验证宏展开与兼容路径；`entry/src/lib.rs` 包含入口宏的正例、参数拒绝
测试和编译失败示例。测试入口使用 stub，不证明真实程序启动成功。

## 工具链与维护

使用 `rust-toolchain.toml` 固定的 `nightly-2026-09-04`，首次运行需要 Rustup
下载工具链及 RV64 目标。依赖版本以 Cargo 清单和锁文件为准。
下列命令从仓库根目录执行；宿主测试不代表真实内核运行验证。

```sh
cargo test --workspace --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
```

## 贡献与来源

维护与 PR 规则见 [Shuyra 贡献规范](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md)。
来源、作者与许可情况见 [PROVENANCE.md](PROVENANCE.md)。
