# mold

`mold` 是 shuyra 宏的**门面**（facade）：七个公开路径一处不少，但它自己不实现任何一枚宏。

> 来源：初始源码 snapshot 取自 anreonyr/sqware 提交
> `2c17c3877e3f8c67d9db5634d693fa4303d47f8c` 的 `crates/mold`（见 `PROVENANCE.md`）。
> 本分支把 codec / contract 六枚宏的所有权交给 canonical 的 `kernel-api-macros`，
> 只把入口那一手留在本仓的独立过程宏包 `mold-entry`。消费者切换由后续迁移 PR 记录。

## 拥有者

| 路径 | 包 / lib | 拥有 | 形式 |
| --- | --- | --- | --- |
| `.` | `mold` / `mold` | 门面：无定义，只转出七枚宏 | 普通 `no_std` 库 |
| `entry/` | `mold-entry` / `mold_entry` | `#[entry]` 的全部实现（`entry/src/entry.rs`，从本仓 `src/entry.rs` 逐字搬来） | 过程宏 crate |

canonical owner：**`kernel-api-macros`**，钉在
`https://github.com/shuyra-arche/kernel.git` rev
`a9986c82127cf25183bf5c30e7d42e4a9376b734`。它拥有 `Frame` / `Envcall` / `Fail` /
`WireCodes` / `interface` / `contract` 的实现；本仓不再保存第二份（原 `src/codes.rs`、
`src/contract.rs`、`src/envcall.rs`、`src/fail.rs`、`src/frame.rs`、`src/interface.rs`
已删除）。

## 依赖边

```text
mold (facade) -> kernel-api-macros          (git, rev a9986c8…)
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
| `#[derive(Envcall)]` | 调用枚举 → `slot` / `pack` / `from_wire` / `Ret` / `call` 与逐槽入口 | `kernel-api-macros` |
| `#[derive(Fail)]` | 无字段失败枚举 → `code` / `of_code` / `FailCode` / `Display` | `kernel-api-macros` |
| `#[derive(WireCodes)]` | 失败域与线上 `u8` 码的映射及编译期断言 | `kernel-api-macros` |
| `#[interface]` | 由内联接口模块生成请求帧、回复、授权与标记 | `kernel-api-macros` |
| `#[contract]` | 声明一对带类型的线上请求/回复及显式回复路径 | `kernel-api-macros` |
| `#[entry]` | 把 `main` 接到 `clean_ret`，展开体调用 `execution::boot::entry` | `mold-entry`（本仓 `entry/`） |

`#[entry]` 有意绑定下游：生成的代码引用 `execution::boot::entry`，由调用点提供 `execution`
crate。`mold` 与 `mold-entry` 都不依赖它。

其余宏生成的是 `env` / `env::wire` / `env::marks` / `::wire` 路径下的类型，由调用者提供
对应 `env` / `wire` crate。

## 行为不变

- 七个公开路径不变：`mold::{Frame, Envcall, Fail, WireCodes, interface, contract, entry}`。
- 生成物不变，六枚宏的展开正文全部保留。Root 用旧 `mold` HEAD 逐文件 diff 核对的实际情况是：
  `codes.rs` / `contract.rs` / `envcall.rs` / `fail.rs` / `interface.rs` 逐字相同；
  `frame.rs` **只在文件头第二行 doc 注释**多了「，生成物使它自己就是过线的一格」——没有可执行
  token、属性、诊断文本或生成路径改变。故 `Frame` 默认 codec 仍是 `::env::wire`；`interface`
  仍写 `env::Frame` / `env::Mark` / `::env::marks` / `::wire::Message`；`contract` 仍写
  `::wire::Contract` / `::env::Mark`；`entry` 展开仍是"原函数照留 + `#[unsafe(no_mangle)]
  extern "C" fn clean_ret()` 调 `execution::boot::entry(main)`"。拆分只换了实现的 owner。

## 验收夹具（host only）

| 文件 | 证明 |
| --- | --- |
| `tests/macro_contract.rs` | `mold::{Frame, WireCodes, interface, contract}` 四个定义型路径在 canonical `kernel-api-env` / `kernel-api-wire` 上展开并跑通。由 Kernel `api/tests/macro_contract.rs`（同一 SHA）改写而来：结构体与断言不变，只把宏路径换成 facade 路径。 |
| `tests/entry.rs` | `#[mold::entry]` 正例：文件内自带 test-only `execution::boot::entry` stub（atomics 计数），嵌套模块的 `#[mold::entry] fn main` 经生成的 `clean_ret` 调到 stub，`main` 的返回值被记录。 |
| `tests/macro_paths.rs` | 七个公开路径都能 `use` 到。`Envcall` / `Fail` 只到"路径可用"这一层：它们的展开由 canonical owner 自己的测试覆盖，本仓不声称运行时 / 系统调用路径。 |
| `entry/src/lib.rs` 的 `#[cfg(test)] mod tests` 与 `entry` 上的 `compile_fail` doc test | `#[entry]` 负例：被标的函数带参数时只吐 `compile_error!`，不生成 `clean_ret`。单元测试直接调展开函数（负例断 `compile_error!` 且无 `clean_ret`，正例断形状）；doc test 是编译期探针——stub `execution` 收 `fn(u32)`、被标函数不叫 `main`，故判据一被拆掉它就会编过，`compile_fail` 当场失效。另有独立真实编译器的参数负例，诊断为 `#[entry] 的 main 不收参数`。生产文件 `entry/src/entry.rs` 未改一字。 |

## 构建与测试（Root 拥有 Cargo 与 lock）

`Cargo.lock` 已由 Root 为这个工作区重新生成（保留原 registry 版本，无旧锁残留）；exact Git
pin 已在外部构建通过。测试只在宿主目标上跑，不涉及 QEMU。

```sh
cargo test --workspace --target x86_64-unknown-linux-gnu --locked
cargo check --lib --target riscv64gc-unknown-none-elf --locked
cargo tree --target x86_64-unknown-linux-gnu --locked
```

工具链由本目录的 `rust-toolchain.toml` 钉住。

## 发布闸门

1. 先各自接受两半拆分：外部 `env` / `wire` adapter 闭包与 `mold` facade + `mold-entry`
   分别过宿主宏夹具与 RV 检查，同一个构建图里每个 API owner 只有一份实现。
2. 消费者闭包**一次**整套同步切到 facade 的 `mold`——按**直接＋传递**的整个 `env` / `wire` /
   `mold` 闭包原子切换。只换直接依赖的 `mold`、而旧 `env` 仍传递依赖旧 `mold`（或反之）**不算**
   合法切换：同一个构建图里只允许一个宏 owner，旧 `mold`（proc-macro crate）与本 facade 不得
   共存，否则同一条源码可能各取一份实现，生成物与 codec 语义分叉。旧外部 `mold` 切换后除名。
3. 本 PR 不做生产切换：没有消费者改动、没有 QEMU、没有 GitHub review。

owner 已经通过精确 Git SHA `a9986c82127cf25183bf5c30e7d42e4a9376b734` 公有；本 PR 不含任何
registry 发布步骤。

Root 验证属性：

- 七个路径都能解析，且 `mold` 不再是 proc-macro crate；
- `kernel-api-macros` 在图中只出现一次，`source` 为上述 SHA 的 git 源；
- `env` / `wire` 只出现在 `mold` 的 dev-dependencies 一侧；
- 图中的 `execution` 边为零；
- `entry/src/entry.rs` 与拆分前 `crates/mold/src/entry.rs`（`2c17c387`）逐字相同。

Root 已跑：宿主 7 个测试和新增的 1 个 `compile_fail` doctest 通过（两条继承来的 doctest 仍为 ignore）、
RV facade 通过、`cargo metadata` 只含上述 SHA 的 canonical API 源且 legacy owner 为零；外部
`env` / `wire` / `mold` 的本地 adapter 闭包宿主宏夹具与 RV 通过，每个 API owner 一份；只面向
既有 `execution`（`1b333a28bc4c34beaea3f7986a9bb6e8ab2db414`）的 entry-only RV 探针通过；
搬移后的 `entry/src/entry.rs` 与旧 HEAD 逐字节相同；`Cargo.lock` 已重生成并保留原 registry
版本。声明的工具链 nightly-2026-09-04（rustc 1.100.0-nightly `a69a63265`）由 Root 独立安装后
跑通：7 个宿主测试 + 1 个 `compile_fail` doctest + RISC-V `cargo build --lib` 全过（更早只跑过
环境覆盖的 nightly-2026-10-09；改写后的 doctest 已在声明工具链上重跑通过）。
独立参数负例也由真实编译器拒绝，诊断为 `#[entry] 的 main 不收参数`，没有把其他编译错误当作通过；
即负例证据有两条：改写后的 doc test 与这条直接诊断探针。
