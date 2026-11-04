//! Positive facade fixture for `mold::entry`.
//!
//! The expansion is unchanged by the split: the user's function verbatim, plus
//! `#[unsafe(no_mangle)] extern "C" fn clean_ret() { execution::boot::entry(main) }`. The macro
//! itself has no runtime dependency, so this fixture supplies its own test-only
//! `execution::boot::entry` stub and asserts the generated `clean_ret` reaches it.

use std::sync::atomic::{AtomicUsize, Ordering};

static STUB_CALLS: AtomicUsize = AtomicUsize::new(0);
static MAIN_RESULT: AtomicUsize = AtomicUsize::new(0);

/// Test-only stand-in for the `execution::boot::entry` glue the caller must provide.
mod execution {
    pub mod boot {
        use crate::{MAIN_RESULT, STUB_CALLS};
        use std::sync::atomic::Ordering;

        pub fn entry(main: fn() -> u8) {
            STUB_CALLS.fetch_add(1, Ordering::SeqCst);
            MAIN_RESULT.store(usize::from(main()), Ordering::SeqCst);
        }
    }
}

mod app {
    use crate::execution;

    #[mold::entry]
    fn main() -> u8 {
        7
    }

    /// The macro-generated `clean_ret` is private to this module; reach it from here.
    pub fn clean_ret_of_the_fixture() {
        clean_ret()
    }
}

#[test]
fn entry_expansion_calls_execution_boot_entry() {
    assert_eq!(STUB_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(MAIN_RESULT.load(Ordering::SeqCst), 0);
    app::clean_ret_of_the_fixture();
    assert_eq!(STUB_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(MAIN_RESULT.load(Ordering::SeqCst), 7);
}
