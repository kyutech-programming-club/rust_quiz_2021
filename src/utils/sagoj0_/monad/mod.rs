pub mod option;
pub mod result;

pub trait Functor {
    type A;
    type Lifted<B>: Functor;

    fn fmap<F, B>(self, f: F) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> B;
}

pub trait Applicative: Functor {
    fn pure(t: Self::A) -> Self::Lifted<Self::A>;
    fn apply<F, B, C>(self, b: Self::Lifted<B>, f: F) -> Self::Lifted<C>
    where
        F: Fn(Self::A, B) -> C;
}

pub trait Monad: Applicative {
    fn bind<B, F>(self, f: F) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> Self::Lifted<B>;
}
