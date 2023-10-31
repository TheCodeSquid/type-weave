use crate::Weave;

pub trait WeaveBelow: Sized {
    fn below<T>(self, other: T) -> T
    where
        T: Weave<Self>,
    {
        other.over(self)
    }
}

impl<T> WeaveBelow for T {}
