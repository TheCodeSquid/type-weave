//! A library for merging data in useful ways
//!
//! This crate is in early development, so there are a number of missing features:
//! - user-defined functions for extracting field values
//! - derive support for non-struct data types

pub mod ext;

#[cfg(feature = "derive")]
pub use type_weave_derive::Weave;

pub mod prelude {
    pub use crate::{ext::WeaveBelow as _, Weave};
}

pub trait Weave<T = Self> {
    fn over(self, other: T) -> Self;

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
