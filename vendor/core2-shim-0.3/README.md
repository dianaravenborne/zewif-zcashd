# core2-shim-0.3 (TEMPORARY)

Local shim that re-exports [`corez`](https://github.com/zcash/corez) under the
`core2` name to satisfy a transitive dependency pinned to `core2 ^0.3`. This
crate exists **solely** to keep `cargo build` working without a committed
`Cargo.lock` while the dependency graph still contains crates that haven't
migrated to `corez`.

## Background

All `core2` versions on crates.io were yanked by their author on 2026-04-14.
Cargo refuses to resolve to yanked versions during a fresh resolve (one
without a `Cargo.lock`).

`corez` is the Zcash ecosystem's clean-room replacement for `core2`,
maintained at <https://github.com/zcash/corez>. It is API-compatible with the
subset of `core2::io` that downstream crates use.

## Why we need it here

`zewif-zcashd` deliberately pins its dependencies to match the `zcashd 0.6.2`
dependency set for `wallet.dat` bit-compatibility (see the comment in
`Cargo.toml`). That pin transitively reaches `equihash 0.2.2` (via
`zcash_primitives 0.19`), which declares `core2 = "^0.3"`.

Independently, the entire `orchard 0.x < 0.14` line was yanked by zcash in
mid-2026, forcing this crate to bump `orchard` to `0.14`. The yanked
`equihash 0.2.2 → core2 0.3.x` chain remains via the rest of the still-pinned
`zcashd 0.6.2` set. This shim closes that hole without forcing the broader
dependency stack forward.

## When to remove this shim

The shim and its `[patch.crates-io]` entry in `Cargo.toml` can be deleted once
`equihash 0.2.x` drops out of the dep graph -- in practice, once
`zcash_primitives` is bumped past `0.27` (which uses `equihash 0.3.0 → corez`).
Doing so would also break the `wallet.dat 0.6.2 bit-compat` pin invariant, so
this shim is expected to live for as long as that invariant does.

To verify removal is safe at any point in the future:

```sh
cargo tree -i core2
```

If the command reports "package ID specification `core2` did not match any
packages", the shim can be deleted along with its `[patch.crates-io]` entry.

## Do not depend on this crate from any zewif-zcashd code

Importing `core2` directly from zewif-zcashd sources defeats the purpose:
when the shim is removed, those imports will silently break. Use `std::io`
(or `corez` directly) instead.
