//! A bridge between [proptest](https://crates.io/crates/proptest) and the
//! [rand](https://crates.io/crates/rand) ecosystem.
//!
//! # Problem
//!
//! Proptest's [`TestRng`] implements
//! `RngCore`, but it does not implement proptest's own
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
//!         let _ = rng.next_u32();
//!     }
//! }
//! ```

use std::ops::{Deref, DerefMut};

use proptest::prelude::*;
use proptest::test_runner::TestRng;

/// A wrapper around proptest's [`TestRng`] that implements both
/// [`Arbitrary`] and [`RngCore`].
///
/// Use `any::<ProptestRng>()` in a `proptest!` block to get a deterministic,
/// reproducible RNG that works with any crate in the `rand` ecosystem.
///
/// This type implements [`Deref`] and [`DerefMut`] to `TestRng`, so all
/// `TestRng` methods are available directly.
#[derive(Debug, Clone)]
pub struct ProptestRng(TestRng);

impl ProptestRng {
    /// Unwraps the wrapper, returning the inner [`TestRng`].
    pub fn into_inner(self) -> TestRng {
        self.0
    }
}

impl From<TestRng> for ProptestRng {
    fn from(rng: TestRng) -> Self {
        Self(rng)
    }
}

impl From<ProptestRng> for TestRng {
    fn from(rng: ProptestRng) -> Self {
        rng.0
    }
}

impl Deref for ProptestRng {
    type Target = TestRng;

    fn deref(&self) -> &TestRng {
        &self.0
    }
}

impl DerefMut for ProptestRng {
    fn deref_mut(&mut self) -> &mut TestRng {
        &mut self.0
    }
}

impl Arbitrary for ProptestRng {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: ()) -> Self::Strategy {
        any::<()>()
            .prop_perturb(|_, rng| ProptestRng(rng))
            .boxed()
    }
}
