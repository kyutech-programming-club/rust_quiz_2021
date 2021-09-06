use super::Functor;
use anyhow::Result;

impl<T> Functor for Result<T> {
    fn fmap<F, U>(self, f: F) -> Result<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f)
    }
}
