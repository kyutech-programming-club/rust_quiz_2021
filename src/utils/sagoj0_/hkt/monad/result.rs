use super::Monad;
use anyhow::Result;

impl<T> Monad for Result<T> {
    fn bind<U, F>(self, f: F) -> Result<U>
    where
        F: FnOnce(T) -> Result<U>,
    {
        self.and_then(f)
    }
}
