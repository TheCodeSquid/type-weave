//! A library that simplifies the combination of custom data types.
//!
//! This crate is in early development, so there are a number of missing features:
//! - user-defined functions for extracting field values
//! - derive support for non-struct data types

pub mod ext;

#[cfg(feature = "derive")]
pub use type_weave_derive::Weave;

pub mod prelude {
    pub use super::{ext::*, Layer, Merge};

    #[cfg(feature = "derive")]
    pub use super::Weave;
}

pub trait Layer<T, U> {
    fn into_layered(lower: T, upper: U) -> Self;
}

impl Layer<bool, bool> for bool {
    fn into_layered(lower: bool, upper: bool) -> Self {
        upper || lower
    }
}

impl<T> Layer<Option<T>, Option<T>> for Option<T> {
    fn into_layered(lower: Option<T>, upper: Option<T>) -> Self {
        upper.or(lower)
    }
}

pub trait Merge<T, U> {
    fn into_merged(left: T, right: U) -> Self;
}
