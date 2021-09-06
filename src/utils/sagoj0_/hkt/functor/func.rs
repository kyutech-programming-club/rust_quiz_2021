use super::Functor;

type F<T, U> = Box<dyn Fn(T) -> U>;

impl<T, U> Functor for F<T, U> {}
