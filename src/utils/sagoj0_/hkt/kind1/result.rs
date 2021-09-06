use super::Kind1;
use anyhow::Result;

impl<T> Kind1 for Result<T> {
    type Inner = T;
    type Lifted<U> = Result<U>;
}
