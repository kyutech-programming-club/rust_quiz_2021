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
