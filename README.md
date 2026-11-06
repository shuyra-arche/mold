# mold

`mold` 是 shuyra 宏的**门面**（facade）：七个公开路径一处不少，但它自己不实现任何一枚宏。

> 来源：初始源码 snapshot 取自 anreonyr/sqware 提交
> `2c17c3877e3f8c67d9db5634d693fa4303d47f8c` 的 `crates/mold`（见 `PROVENANCE.md`）。
> codec / contract 六枚宏由 canonical 的 `kernel-api-macros` 拥有；
> 入口宏留在本仓的独立过程宏包 `mold-entry`。本次升级跟随 Kernel 的 API / Client 分层。

## 拥有者

| 路径 | 包 / lib | 拥有 | 形式 |
| --- | --- | --- | --- |
| `.` | `mold` / `mold` | 门面：无定义，只转出七枚宏 | 普通 `no_std` 库 |
| `entry/` | `mold-entry` / `mold_entry` | `#[entry]` 的全部实现（`entry/src/entry.rs`，从本仓 `src/entry.rs` 逐字搬来） | 过程宏 crate |

canonical owner：**`kernel-api-macros`**，钉在
`https://github.com/shuyra-arche/kernel.git` rev
`90d43dbf3ee43fd146f487718eb39e88f2507c05`。它拥有 `Frame` / `Envcall` / `Fail` /
`WireCodes` / `interface` / `contract` 的实现；本仓不再保存第二份（原 `src/codes.rs`、
`src/contract.rs`、`src/envcall.rs`、`src/fail.rs`、`src/frame.rs`、`src/interface.rs`
已删除）。

## 依赖边

```text
mold (facade) -> kernel-api-macros          (git, rev 90d43db…)
mold (facade) -> mold-entry                 (path: entry/)
mold-entry    -> proc-macro2, quote, syn
mold (dev)    -> env  = kernel-api-env      (git, 同一 SHA)   # 只有验收夹具用
mold (dev)    -> wire = kernel-api-wire     (git, 同一 SHA)   # 只有验收夹具用
```

- `mold` 自己不再是 proc-macro crate：普通 `no_std` 库，正文只有两条 `pub use`。
- `env` / `wire` 只在 `[dev-dependencies]` 里、以 `package` 改名的形式出现，给夹具提供
  canonical 的 `Span` / `Message` / `Contract` / `Mark` 类型；它们不进 `mold` 的运行时依赖图。
- 没有任何一条边指向原 `env` / `wire` / `mold` 仓，也没有任何一条边指向 `execution`。

## 宏

| 导出 | 用途 | 实现 owner |
| --- | --- | --- |
| `#[derive(Frame)]` | 具名字段结构体 → `pub const LEN` + `impl env::wire::Span` | `kernel-api-macros` |
| `#[derive(Envcall)]` | 调用枚举 → `slot` / `pack` / `from_wire` / `Ret` / 纯 `decode_return`，以及 Client 使用的隐藏入口生成宏 | `kernel-api-macros` |
| `#[derive(Fail)]` | 无字段失败枚举 → `code` / `of_code` / `FailCode` / `Display` | `kernel-api-macros` |
| `#[derive(WireCodes)]` | 失败域与线上 `u8` 码的映射及编译期断言 | `kernel-api-macros` |
| `#[interface]` | 由内联接口模块生成请求帧、回复、授权与标记 | `kernel-api-macros` |
| `#[contract]` | 声明一对带类型的线上请求/回复及显式回复路径 | `kernel-api-macros` |
| `#[entry]` | 把 `main` 接到 `clean_ret`，展开体调用 `execution::boot::entry` | `mold-entry`（本仓 `entry/`） |

`#[entry]` 有意绑定下游：生成的代码引用 `execution::boot::entry`，由调用点提供 `execution`
crate。`mold` 与 `mold-entry` 都不依赖它。

`Frame` / `interface` / `contract` 的展开引用 `env` / `env::wire` / `env::marks` /
`::wire` 路径下的类型，由调用者提供对应 `env` / `wire` crate。

## 接口与兼容范围

- 七个公开路径不变：`mold::{Frame, Envcall, Fail, WireCodes, interface, contract, entry}`。
- `Envcall` 的生成接口跟随 canonical API：枚举负责载荷编解码和纯返回解码，隐藏生成宏由
  Client 传入 transport 后生成逐槽函数。枚举上的执行型 `.call()` 和直接生成的逐槽入口已移出
  API；仍使用旧入口的调用方需要迁到 `kernel-client`，不能只按七个宏路径推断源码兼容。
- 调用号、字段布局和返回寄存器约定继续由同一份 canonical ABI 声明决定。门面不复制 ABI
  或提供第二套调用实现；这里的夹具只覆盖下表列出的宏路径与展开行为。
- `Frame` 默认 codec 仍是 `::env::wire`；`interface` 使用
  `env::Frame` / `env::Mark` / `::env::marks` / `::wire::Message`；`contract` 使用
  `::wire::Contract` / `::env::Mark`。`entry` 实现逐字节保留，展开仍是原函数与调用
  `execution::boot::entry(main)` 的 `#[unsafe(no_mangle)] extern "C" fn clean_ret()`。

## 验收夹具（host only）

| 文件 | 证明 |
| --- | --- |
| `tests/macro_contract.rs` | `mold::{Frame, WireCodes, interface, contract}` 四个定义型路径在当前 canonical `kernel-api-env` / `kernel-api-wire` 上展开并跑通。最初由 Kernel `api/tests/macro_contract.rs`（`a9986c8…`）改写而来；本次保留结构体、断言和来源记录。 |
| `tests/entry.rs` | `#[mold::entry]` 正例：文件内自带 test-only `execution::boot::entry` stub（atomics 计数），嵌套模块的 `#[mold::entry] fn main` 经生成的 `clean_ret` 调到 stub，`main` 的返回值被记录。 |
| `tests/macro_paths.rs` | 七个公开路径都能 `use` 到。`Envcall` / `Fail` 只到"路径可用"这一层：它们的展开由 canonical owner 自己的测试覆盖，本仓不声称运行时 / 系统调用路径。 |
| `entry/src/lib.rs` 的 `#[cfg(test)] mod tests` 与 `entry` 上的 `compile_fail` doc test | `#[entry]` 负例：被标的函数带参数时只吐 `compile_error!`，不生成 `clean_ret`。单元测试直接调展开函数（负例断 `compile_error!` 且无 `clean_ret`，正例断形状）；doc test 是编译期探针——stub `execution` 收 `fn(u32)`、被标函数不叫 `main`，故判据一被拆掉它就会编过，`compile_fail` 当场失效。生产文件 `entry/src/entry.rs` 未改一字。 |

## 构建与测试

根 `Cargo.lock` 与三处 manifest Git pin 同步，registry 版本保持原锁定值。
测试在宿主目标上运行，RISC-V 只构建门面库。

```sh
cargo test --workspace --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
cargo metadata --format-version 1 --locked
cargo tree --target x86_64-unknown-linux-gnu --locked
```

本次验证使用环境已安装的 nightly-2026-10-09（显式覆盖）；本目录仍声明
nightly-2026-09-04，本轮未重跑该声明工具链。结果为 7 个宿主测试通过、1 个
`compile_fail` doctest 通过、2 个继承来的 doctest 保持 ignored，RISC-V 门面库构建通过。

## 消费者迁移范围

消费者需要将直接与传递的 `env` / `wire` / `mold` 闭包同步到同一 SDK，确保各 API 包
只有一个 owner；再将执行调用迁到 Client。本仓升级门面依赖，生产产品的构建与运行证据
由消费者迁移记录。

本工作区的 metadata 验证属性：

- 七个路径都能解析，且 `mold` 不再是 proc-macro crate；
- `kernel-api-env` / `kernel-api-wire` / `kernel-api-macros` 各出现一次，
  `source` 均为上述 SHA 的公有 Git 源；
- `env` / `wire` 只出现在 `mold` 的 dev-dependencies 一侧；
- 图中的 `execution` 边为零；
- `entry/` 与 `tests/` 相对本次升级基线未改动，保留入口实现及所有断言。
