//! A library for merging layered data.
//!
//! This crate is primarily intended for helping with configuration,
//! as multiple instances of a struct may need to be joined in a layered fashion.
//!
//! See [`Weave`] for more information on its usage and default type implementations.
//!
//! # Features
//!
//! The `derive` feature (enabled by default) adds a derive macro for `Weave`,
//! which implements the trait for structs entirely composed of fields that
//! also implement the trait.
//!
//! [`Weave`]: crate::Weave

#[cfg(feature = "derive")]
pub use type_weave_derive::Weave;

/// Trait for layered type merging.
pub trait Weave<T = Self> {
    /// Merge the values, prioritizing `self`.
    ///
    /// # Example
    /// ```
    /// # use type_weave::Weave;
    /// # fn main() {
    /// let a = Some("a");
    /// let b = Some("b");
    ///
    /// assert_eq!(a.over(b), Some("a"));
    /// # }
    fn over(self, other: T) -> Self;

    /// Merge the values, prioritizing `other`.
    ///
    /// # Example
    /// ```
    /// # use type_weave::Weave;
    /// # fn main() {
    /// let a = Some("a");
    /// let b = Some("b");
    ///
    /// assert_eq!(a.under(b), Some("b"));
    /// # }
    fn under(self, other: T) -> Self;
}

impl Weave for bool {
    fn over(self, other: Self) -> Self {
        self || other
    }

    fn under(self, other: Self) -> Self {
        self || other
    }
}

impl<T> Weave for Option<T> {
    fn over(self, other: Self) -> Self {
        self.or(other)
    }

    fn under(self, other: Self) -> Self {
        other.or(self)
    }
}
