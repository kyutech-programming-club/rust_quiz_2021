use super::*;

impl<A> Functor for Option<A> {
    type A = A;
    type Lifted<B> = Option<B>;

    fn fmap<F, T>(self, f: F) -> Self::Lifted<T>
    where
        F: FnOnce(A) -> T,
    {
        self.map(f)
    }
}

impl<A> Applicative for Option<A> {
    fn pure(t: A) -> Option<A> {
        Some(t)
    }

    fn apply<F, B, C>(self, b: Option<B>, f: F) -> Option<C>
    where
        F: FnOnce(A, B) -> C,
    {
        Some(f(self?, b?))
    }
}

impl<A> Monad for Option<A> {
    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: FnOnce(A) -> Option<B>,
    {
        self.and_then(f)
    }
}
