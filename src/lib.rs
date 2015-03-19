#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # Expect
//!
//! A flexible library for adding assertions to types.
//!

/// An assertion that can be applied to a type.
pub trait Assertion<T: ?Sized>: Sized {
    /// Assert some property about T.
    ///
    /// ## Panics
    ///
    /// This method SHOULD panic if the property is not fulfilled.
    #[inline]
    fn assert(self, _: &T) { }
}

/// Extension trait for applying assertions.
pub trait Expect {
    /// Check an assertion on Self.
    #[inline]
    fn expect<A: Assertion<Self>>(self, assertion: A) -> Self where Self: Sized {
        assertion.assert(&self);
        self
    }

    /// Check an assertion on a reference to Self.
    #[inline]
    fn expect_ref<A: Assertion<Self>>(&self, assertion: A) -> &Self {
        assertion.assert(self);
        self
    }
}

impl<E> Expect for E {}

mod impls;

