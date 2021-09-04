//! Option<T>のモナド実装を与える
use super::*;

impl<A> Functor for Option<A>
where
    A: Clone,
{
    type A = A;
    type Lifted<B: Clone> = Option<B>;

    fn fmap<F, T>(self, f: F) -> Option<T>
    where
        F: FnOnce(A) -> T,
    {
        self.map(f)
    }
}

impl<A> Applicative for Option<A>
where
    A: Clone,
{
    fn pure(t: A) -> Option<A> {
        Some(t)
    }

    fn lift_a2<F, B, C>(self, b: Option<B>, f: F) -> Option<C>
    where
        F: FnOnce(A, B) -> C,
    {
        Some(f(self?, b?))
    }

    fn apply<F, B: Clone>(self, f: Option<F>) -> Option<B>
    where
        F: Fn(Self::A) -> B + Clone,
    {
        f.map(|f| self.map(f)).flatten()
    }
}

impl<A> Monad for Option<A>
where
    A: Clone,
{
    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: FnOnce(A) -> Option<B>,
    {
        self.and_then(f)
    }
}
