use super::Kind1;

impl<T> Kind1 for Option<T> {
    type Inner = T;
    type Lifted<U> = Option<U>;
}
