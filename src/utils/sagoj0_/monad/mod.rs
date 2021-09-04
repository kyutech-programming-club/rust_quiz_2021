//! monadに関する実装を提供する．

pub mod func;
pub mod option;
pub mod result;
pub mod vec;

/// 変換処理を一般的に扱うためのトレイト
pub trait Functor {
    type A: Clone;
    type Lifted<B: Clone>: Functor;

    /// M<T>という構造の中身に対して関数を適用する
    fn fmap<F, B: Clone>(self, f: F) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> B;
}

/// 関数適用を一般的に扱うためのトレイト
pub trait Applicative: Functor {
    /// 一般の値を文脈に包んで返却する
    fn pure(t: Self::A) -> Self::Lifted<Self::A>;

    /// 文脈に入った関数を文脈に入った値へ適用する．
    fn apply<F, B: Clone>(self, f: Self::Lifted<F>) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> B + Clone;

    /// 関数を文脈に持ち上げて文脈の値に適用する
    fn lift_a2<F, B: Clone, C: Clone>(self, b: Self::Lifted<B>, f: F) -> Self::Lifted<C>
    where
        F: Fn(Self::A, B) -> C + Clone;

    fn lift_a<F, B: Clone>(self, f: F) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> B,
    {
        self.apply(pure(f))
    }
}

/// 手続きの結合を一般的に扱うためのトレイト
pub trait Monad: Applicative {
    // 文脈内の値を与えられた関数の引数としてbindする
    fn bind<B: Clone, F>(self, f: F) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> Self::Lifted<B>;
}
