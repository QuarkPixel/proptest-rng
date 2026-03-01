# CLAUDE.md

## Project

`proptest-rng` — a minimal bridge crate between proptest and the rand ecosystem.

Provides `ProptestRng`, a newtype wrapper around `proptest::test_runner::TestRng` that implements both `proptest::arbitrary::Arbitrary` and `rand_core::RngCore`.

## Commands

```bash
cargo build
cargo test        # includes doc-tests
cargo doc --open  # view generated docs
```

## Architecture

Single file: `src/lib.rs`. Three impl blocks:

- `#[derive(Debug, Clone)]` on the newtype
- `Arbitrary` — uses `prop_perturb` to capture proptest's internal `TestRng`
- `RngCore` — delegates all methods to the inner `TestRng`

## Dependencies

Only two:
- `proptest = "1"` — for `Arbitrary`, `TestRng`, `BoxedStrategy`
- `rand_core = "0.9"` — for the `RngCore` trait

Both must use the same `rand_core` version. If proptest upgrades to a new `rand_core` major version, this crate must follow.

## Publishing

```bash
cargo login       # first time only
cargo publish --dry-run
cargo publish
```
