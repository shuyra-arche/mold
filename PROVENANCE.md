# Provenance

## Source

- Repository: <https://github.com/anreonyr/sqware>
- Commit: `2c17c3877e3f8c67d9db5634d693fa4303d47f8c`
- Branch at extraction: `codex/shuyra-mold-extraction`
- Extracted path: `crates/mold`

This directory is the initial source snapshot of that commit. The Rust sources
under `src/` are byte-for-byte copies; `rust-toolchain.toml` is copied unchanged.
`Cargo.toml` is a new standalone manifest (the upstream manifest inherited
`version` and `edition` from the workspace), and `Cargo.lock` was generated for
this crate with `unicode-ident` pinned to the source-verified 1.0.24. At
extraction time, no source consumer in the upstream repository was changed.

## File mapping

| Upstream | Here |
| --- | --- |
| `crates/mold/src/codes.rs` | `src/codes.rs` |
| `crates/mold/src/contract.rs` | `src/contract.rs` |
| `crates/mold/src/entry.rs` | `src/entry.rs` |
| `crates/mold/src/envcall.rs` | `src/envcall.rs` |
| `crates/mold/src/fail.rs` | `src/fail.rs` |
| `crates/mold/src/frame.rs` | `src/frame.rs` |
| `crates/mold/src/interface.rs` | `src/interface.rs` |
| `crates/mold/src/lib.rs` | `src/lib.rs` |
| `rust-toolchain.toml` | `rust-toolchain.toml` |

The upstream `crates/mold/Cargo.toml` is intentionally not copied; this crate has
its own explicit `version = "0.1.0"` and `edition = "2024"`.

## Attribution

- anreonyr <88045926+anreonyr@users.noreply.github.com>
- anreonyr <anreonyr@gmail.com>

## History

The original repository history is retained at the source URL above. This
extraction is a snapshot without a rewritten or synthesized history.

## Licensing

The source project declares no license. At commit
`2c17c3877e3f8c67d9db5634d693fa4303d47f8c` no tracked `LICENSE`, `COPYING`, or
`NOTICE` file matched the project. No license text or license metadata is added
by this extraction; do not assume a license that the source does not grant.

## Snapshot

The initial source snapshot was taken from the full commit
`2c17c3877e3f8c67d9db5634d693fa4303d47f8c` above. Consumer switch is recorded by
subsequent migration PRs in anreonyr/sqware.
