pub mod option {
    pub fn bind<T, U, F>(m: Option<T>, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        m.and_then(f)
    }

    pub fn empty<T>() -> Option<T> {
        None
    }

    pub fn pure<T>(v: T) -> Option<T> {
        Some(v)
    }
}

pub mod result {
    use anyhow::Result;

    pub fn bind<T, U, F>(m: Result<T>, f: F) -> Result<U>
    where
        F: FnOnce(T) -> Result<U>,
    {
        m.and_then(f)
    }
    pub fn pure<T>(v: T) -> Result<T> {
        Ok(v)
    }
}
