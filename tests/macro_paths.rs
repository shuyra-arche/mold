//! Availability check for the two facade paths that intentionally have no runtime fixture here:
//! `mold::Envcall` and `mold::Fail`.
//!
//! The `use` list below is the whole test: if any of the seven public macro paths stops resolving,
//! this fixture fails to compile. Expansion of `Envcall` / `Fail` is covered by the canonical
//! owner's own tests; this repo makes no runtime or syscall claim for them.

#![allow(unused_imports)]

use mold::{Envcall, Fail, Frame, WireCodes, contract, entry, interface};

#[test]
fn facade_reexports_the_seven_public_macro_paths() {}
