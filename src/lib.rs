//! A bridge between [proptest](https://crates.io/crates/proptest) and the
//! [rand](https://crates.io/crates/rand) ecosystem.
//!
//! # Problem
//!
//! Proptest's [`TestRng`](proptest::test_runner::TestRng) implements
//! [`RngCore`](rand_core::RngCore), but it does not implement proptest's own
//! [`Arbitrary`] trait. This means you cannot write `any::<TestRng>()` in a
//! `proptest!` block to get a deterministic RNG for use with crates like
//! [`fake`](https://crates.io/crates/fake).
//!
//! # Solution
//!
//! This crate provides [`ProptestRng`], a newtype wrapper around `TestRng` that
//! implements both `Arbitrary` and `RngCore`. This lets you request a
//! deterministic RNG directly as a proptest parameter.
//!
//! # Example
//!
//! ```rust
//! use proptest::prelude::*;
//! use proptest_rng::ProptestRng;
//! // use fake::{Fake, faker::internet::en::SafeEmail};
//!
//! proptest! {
//!     #[test]
//!     fn test_with_rng(mut rng in any::<ProptestRng>()) {
//!         // Pass `rng` to any function expecting `impl Rng`:
//!         // let email: String = SafeEmail().fake_with_rng(&mut rng);
//!         let _: u32 = rand_core::RngCore::next_u32(&mut rng);
//!     }
//! }
//! ```

use proptest::prelude::*;
use proptest::test_runner::TestRng;
use rand_core::RngCore;

/// A wrapper around proptest's [`TestRng`] that implements both
/// [`Arbitrary`] and [`RngCore`].
///
/// Use `any::<ProptestRng>()` in a `proptest!` block to get a deterministic,
/// reproducible RNG that works with any crate in the `rand` ecosystem.
#[derive(Debug, Clone)]
pub struct ProptestRng(TestRng);

impl Arbitrary for ProptestRng {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: ()) -> Self::Strategy {
        any::<()>()
            .prop_perturb(|_, rng| ProptestRng(rng))
            .boxed()
    }
}

impl RngCore for ProptestRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }
}
