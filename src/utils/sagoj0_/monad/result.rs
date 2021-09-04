//! anyhow::Result<T>のモナド実装を与える
use super::*;
use anyhow::Result;

impl<A> Functor for Result<A>
where
    A: Clone,
{
    type A = A;
    type Lifted<B: Clone> = Result<B>;

    fn fmap<F, T>(self, f: F) -> Result<T>
    where
        F: FnOnce(A) -> T,
    {
        self.map(f)
    }
}

impl<A> Applicative for Result<A>
where
    A: Clone,
{
    fn pure(t: A) -> Result<A> {
        Ok(t)
    }

    fn lift_a2<F, B, C>(self, b: Result<B>, f: F) -> Result<C>
    where
        F: FnOnce(A, B) -> C,
    {
        Ok(f(self?, b?))
    }

    fn apply<F, B: Clone>(self, f: Self::Lifted<F>) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> B + Clone,
    {
        self.map(f?)
    }
}

impl<A> Monad for Result<A>
where
    A: Clone,
{
    fn bind<B, F>(self, f: F) -> Result<B>
    where
        F: FnOnce(A) -> Result<B>,
    {
        self.and_then(f)
    }
}
