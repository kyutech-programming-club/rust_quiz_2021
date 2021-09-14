use super::Applicative;

impl<T> Applicative for Option<T> {
    fn pure(t: T) -> Option<T> {
        Some(t)
    }
    fn lift_a2<F, U, V>(self, arg: Option<U>, f: F) -> Option<V>
    where
        F: FnOnce(T, U) -> V,
    {
        Option::pure(f(self?, arg?))
    }

    fn apply<F, U>(self, f: Option<F>) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f?)
    }
}
