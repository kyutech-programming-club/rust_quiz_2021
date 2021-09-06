use super::kind1::Kind1;

pub mod option;
pub mod result;

/// 変換処理を一般的に扱うためのトレイト
pub trait Functor: Kind1 {
    /// M<Inner>という構造の中身に対して関数を適用する
    fn fmap<F, U>(self, f: F) -> Self::Lifted<U>
    where
        F: Fn(Self::Inner) -> U;
}
