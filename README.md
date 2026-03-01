# proptest-rng

A bridge between [proptest](https://crates.io/crates/proptest) and the [rand](https://crates.io/crates/rand) ecosystem.

## Problem

When using proptest with crates like [fake](https://crates.io/crates/fake), you need an RNG that implements `rand::Rng`. Proptest's internal `TestRng` implements `RngCore`, but not proptest's `Arbitrary` trait — so you can't write `any::<TestRng>()` to get one as a test parameter.

Common workarounds involve manual seed bridging or `prop_perturb` hacks, both of which obscure test intent.

## Solution

This crate provides `ProptestRng`, a newtype wrapper around `TestRng` that implements both `Arbitrary` and `RngCore`.

```rust
use proptest::prelude::*;
use proptest_rng::ProptestRng;
use fake::{Fake, faker::internet::en::SafeEmail};

proptest! {
    #[test]
    fn valid_emails_are_parsed(mut rng in any::<ProptestRng>()) {
        let email: String = SafeEmail().fake_with_rng(&mut rng);
        // email is deterministic and reproducible via proptest's seed
    }
}
```

No extra dependencies needed — just `proptest` and this crate. The `rand` interop comes through `rand_core`, which proptest already depends on.

## When to use this

- You use **proptest** for property-based testing
- You use **fake**, **rand**, or any crate that accepts `impl Rng`
- You want deterministic, reproducible random values in your tests

## Version compatibility

| proptest-rng | proptest | rand_core |
|:---:|:---:|:---:|
| 0.1.x | 1.x | 0.9.x |

## License

MIT
