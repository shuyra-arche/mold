# mold

`mold` 是一个过程宏 crate，集中生成 shuyra 的线上帧、调用、失败码与接口声明。

> 来源：初始源码 snapshot 取自 anreonyr/sqware 提交
> `2c17c3877e3f8c67d9db5634d693fa4303d47f8c` 的 `crates/mold`。
> 消费者切换由 anreonyr/sqware 后续迁移 PR 记录。

## 宏

| 导出 | 用途 |
| --- | --- |
| `#[derive(Frame)]` | 具名字段结构体 → `pub const LEN` + `impl env::wire::Span` |
| `#[derive(Envcall)]` | 调用枚举 → `slot` / `pack` / `from_wire` / `Ret` / `call` 与逐槽入口 |
| `#[derive(Fail)]` | 无字段失败枚举 → `code` / `of_code` / `FailCode` / `Display` |
| `#[derive(WireCodes)]` | 失败域与线上 `u8` 码的映射及编译期断言 |
| `#[entry]` | 把 `main` 接到 `clean_ret`，展开体调用 `execution::boot::entry` |
| `#[interface]` | 由内联接口模块生成请求帧、回复、授权与标记 |
| `#[contract]` | 声明一对带类型的线上请求/回复及显式回复路径 |

`#[entry]` 有意绑定下游：生成的代码引用 `execution::boot::entry`。
调用者必须自行提供 `execution` crate；`mold` 不依赖它，也不在仓库内携带它。

其余宏生成的是 `env` / `env::wire` / `env::marks` 路径下的类型，由调用者提供对应 `env` crate。

## 独立构建与测试

本 crate 只面向宿主构建，不依赖工作区或跨仓 path 依赖：

```sh
cd mold
cargo build --target x86_64-unknown-linux-gnu --locked
cargo test  --target x86_64-unknown-linux-gnu --locked
```

依赖已缓存后，可离线复验同一组命令：

```sh
cargo build --target x86_64-unknown-linux-gnu --locked --offline
cargo test  --target x86_64-unknown-linux-gnu --locked --offline
```

工具链由本目录的 `rust-toolchain.toml` 钉住。测试在宿主目标上运行，不涉及 QEMU。
