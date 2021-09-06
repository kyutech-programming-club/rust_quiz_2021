use super::functor::Functor;

pub mod option;
pub mod result;

/// 関数適用を一般的に扱うためのトレイト
pub trait Applicative: Functor {
    /// 一般の値を文脈に包んで返却する
    fn pure(t: Self::Inner) -> Self::Lifted<Self::Inner>;

    /// 文脈に入った関数を文脈に入った値へ適用する．
    fn apply<F, T>(self, f: Self::Lifted<F>) -> Self::Lifted<T>
    where
        F: Fn(Self::Inner) -> T;

    /// 関数を文脈に持ち上げて文脈の値に適用する
    fn lift_a2<F, T, U>(self, b: Self::Lifted<T>, f: F) -> Self::Lifted<U>
    where
        F: Fn(Self::Inner, T) -> U;

    /*fn lift_a<F, T>(self, f: F) -> Self::Lifted<T>
    where
        F: Fn(Self::Inner) -> T,
    {
        self.apply(Self::pure(f))
    }*/
}
