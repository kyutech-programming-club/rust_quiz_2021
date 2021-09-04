//! Vec<T>のモナド実装を与える
use super::*;

impl<T> Functor for Vec<T>
where
    T: Clone,
{
    type A = T;
    type Lifted<B: Clone> = Vec<B>;

    fn fmap<F, B>(self, f: F) -> Vec<B>
    where
        F: FnMut(T) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<T> Applicative for Vec<T>
where
    T: Clone,
{
    fn pure(t: T) -> Vec<T> {
        vec![t]
    }
    fn lift_a2<F, B: Clone, C>(self, b: Vec<B>, mut f: F) -> Vec<C>
    where
        F: FnMut(T, B) -> C,
    {
        use itertools::Itertools;
        self.iter()
            .cartesian_product(b.iter())
            .map(|(x, y)| f(x.clone(), y.clone()))
            .collect()
    }
    fn apply<F, B: Clone>(self, f: Vec<F>) -> Vec<B>
    where
        F: Fn(Self::A) -> B + Clone,
    {
        use itertools::Itertools;
        self.iter()
            .cartesian_product(f.iter())
            .map(|(x, f)| f(x.clone()))
            .collect()
    }
}

impl<T> Monad for Vec<T>
where
    T: Clone,
{
    fn bind<B: Clone, F>(self, f: F) -> Vec<B>
    where
        F: FnMut(T) -> Vec<B>,
    {
        use itertools::Itertools;
        self.into_iter().map(f).concat()
    }
}
