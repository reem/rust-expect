#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # Expect
//!
//! A flexible library for adding assertions to types.
//!
//! ```
//! # use expect::{Expect, Assertion};
//! # use std::default::Default;
//!
//! #[derive(Default)]
//! struct Point {
//!     x: f64,
//!     y: f64
//! }
//!
//! // Clockwise from the top left.
//! #[derive(Default)]
//! struct Square(Point, Point, Point, Point);
//!
//! #[derive(Default)]
//! struct Contains(Point);
//!
//! impl Assertion<Square> for Contains {
//!     fn assert(self, square: &Square) {
//!         assert!(self.0.x > square.0.x && self.0.x < square.1.x
//!              && self.0.y > square.2.y && self.0.y < square.1.y);
//!     }
//! }
//!
//! let square = Square(
//!     Point { x: 0.0, y: 1.0 },
//!     Point { x: 1.0, y: 1.0 },
//!     Point { x: 1.0, y: 0.0 },
//!     Point { x: 0.0, y: 0.0 },
//! );
//!
//! square
//!     .expect(Contains(Point { x: 0.5, y: 0.18 }))
//!     .expect(Contains(Point { x: 0.12, y: 0.9 }))
//!
//!     // You can also use tuples of Assertions.
//!     .expect((Contains(Point { x: 0.63, y: 0.4 }),
//!              Contains(Point { x: 0.7, y: 0.85 })));
//! ```
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

