//! 编码、通信接口、环境调用与程序入口的过程宏——**facade，不自己实现任何一枚**。
//!
//! 七个公开路径一处不少，实现分属两个 owner：
//!
//! - codec / contract 六枚（`Frame` / `Envcall` / `Fail` / `WireCodes` / `interface` /
//!   `contract`）来自 canonical 的 `kernel-api-macros`（Kernel 仓，SHA 钉在 `Cargo.toml`）；
//! - `entry` 来自本仓的过程宏成员包 `mold-entry`（`entry/`）——它是唯一绑死下游
//!   `execution::boot::entry` 的那一枚，故不进 canonical 宏 crate。
//!
//! 本 crate 是**普通的 `no_std` 库**，不是 proc-macro crate：它只转出这七枚宏，没有任何
//! 定义，也不连 `execution`（展开体里的 `execution::boot::entry` 由调用点提供）。
//!
//! 转出的宏生成物与拆分前逐字相同：默认 codec 路径仍是 `::env::wire`，`interface` /
//! `contract` 仍写 `env` / `::env` / `::wire`，`entry` 仍写 `execution::boot::entry`。

#![no_std]

pub use kernel_api_macros::{Envcall, Fail, Frame, WireCodes, contract, interface};
pub use mold_entry::entry;
