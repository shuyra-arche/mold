//! `mold-entry`：入口那一手的独立过程宏 crate。
//!
//! `mold` 的 codec/contract 六个宏由 canonical 的 `kernel-api-macros` 提供；只有
//! `#[entry]` 留在这里——它是唯一绑死下游 `execution::boot::entry` 的那一枚，故不进
//! canonical 宏 crate，也不进任何运行时依赖图。
//!
//! `mold` facade 以 `pub use mold_entry::entry;` 把这一格拼回原路径 `mold::entry`。

mod entry;

use proc_macro::TokenStream;

/// **入口那一手**：把你写的 `main` 接到汇编要调的符号上（原函数留着，另加
/// `extern "C" fn clean_ret`）。
///
/// 详见 [`entry`]。
///
/// 负例（编译期）：被标的函数收参数时只吐 `compile_error!`。下面这段自带 stub `execution`：
/// 它收 `fn(u32)`，被标的函数叫 `task`（不叫 `main`，避开 Rust 对 main 签名的拒绝）——故一旦
/// 本宏的参数判据被拆掉，展开出的 `clean_ret` 调 `execution::boot::entry(task)` 就能编过，
/// 这条 `compile_fail` 当场失效，正好只盯住判据本身。
///
/// ```compile_fail
/// fn main() {}
///
/// mod execution {
///     pub mod boot {
///         pub fn entry(task: fn(u32)) {
///             task(0)
///         }
///     }
/// }
///
/// #[mold_entry::entry]
/// fn task(_argument: u32) {}
/// ```
#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    entry::expand(item.into()).into()
}

#[cfg(test)]
mod tests {
    use quote::quote;

    /// 负例：`main` 收参数时只吐 `compile_error!`，不生成 `clean_ret`（展开见 `entry.rs` 文件头）。
    #[test]
    fn rejects_main_with_arguments() {
        let expanded = crate::entry::expand(quote!(
            fn main(arg: u32) {}
        ))
        .to_string();
        assert!(expanded.contains("compile_error"), "{expanded}");
        assert!(!expanded.contains("clean_ret"), "{expanded}");
    }

    /// 正例（只查形状）：无参 `main` 原样保留，另加调用 `execution::boot::entry` 的 `clean_ret`。
    #[test]
    fn accepts_main_without_arguments() {
        let expanded = crate::entry::expand(quote!(
            fn main() -> u8 {
                0
            }
        ))
        .to_string();
        assert!(!expanded.contains("compile_error"), "{expanded}");
        assert!(expanded.contains("clean_ret"), "{expanded}");
        assert!(expanded.contains("execution"), "{expanded}");
        assert!(expanded.contains("boot"), "{expanded}");
    }
}
