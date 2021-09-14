use super::Functor;

impl<T> Functor for Option<T> {
    fn fmap<F, U>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f)
    }
}
