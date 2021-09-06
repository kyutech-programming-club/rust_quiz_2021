use super::applicative::Applicative;

pub mod option;
pub mod result;

pub trait Monad: Applicative {
    // 文脈内の値を与えられた関数の引数としてbindする
    fn bind<T, F>(self, f: F) -> Self::Lifted<T>
    where
        F: Fn(Self::Inner) -> Self::Lifted<T>;
}
