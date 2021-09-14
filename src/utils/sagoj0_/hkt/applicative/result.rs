use super::Applicative;
use anyhow::Result;

impl<T> Applicative for Result<T> {
    fn pure(t: T) -> Result<T> {
        Ok(t)
    }
    fn lift_a2<F, U, V>(self, arg: Result<U>, f: F) -> Result<V>
    where
        F: FnOnce(T, U) -> V,
    {
        Result::pure(f(self?, arg?))
    }

    fn apply<F, U>(self, f: Result<F>) -> Result<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f?)
    }
}
