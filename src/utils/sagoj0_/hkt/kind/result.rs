use super::Kind;
use anyhow::Result;

impl<T> Kind for Result<T> {
    type Inner = T;
    type Lifted<U> = Result<U>;
}
