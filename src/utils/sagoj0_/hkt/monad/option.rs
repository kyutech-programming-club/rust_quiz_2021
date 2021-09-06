use super::Monad;

impl<T> Monad for Option<T> {
    fn bind<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        self.and_then(f)
    }
}
