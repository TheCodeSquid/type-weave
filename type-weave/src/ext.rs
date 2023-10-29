use crate::{Layer, Merge};

pub trait LayerExt<T, U> {
    fn layer(self, upper: U) -> T;
}

impl<S, T, U> LayerExt<T, U> for S
where
    T: Layer<S, U>,
{
    fn layer(self, upper: U) -> T {
        T::into_layered(self, upper)
    }
}

pub trait MergeExt<T, U> {
    fn merge(self, upper: U) -> T;
}

impl<S, T, U> MergeExt<T, U> for S
where
    T: Merge<S, U>,
{
    fn merge(self, upper: U) -> T {
        T::into_merged(self, upper)
    }
}
