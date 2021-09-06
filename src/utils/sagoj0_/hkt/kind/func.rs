use super::{Kind1, Kind2};

type F<T, U, V> = Box<dyn Fn(T) -> U>;

impl<T, U, V> Kind1 for F<T, U, V> {
    type Inner = F<V, U, T>;
    type Lifted<A> = F<A, U, V>;
}
