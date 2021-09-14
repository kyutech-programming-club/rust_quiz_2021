use super::Kind;

impl<T> Kind for Option<T> {
    type Inner = T;
    type Lifted<U> = Option<U>;
}
