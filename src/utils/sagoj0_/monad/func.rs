/// closureのモナド実装を与える
use super::*;
use std::marker::PhantomData;
struct Func<A, B> {
    inner: Box<dyn Fn(A) -> B>,
}

impl<Src, Dst> Functor for Func<Src, Dst>
where
    Src: Clone,
    Dst: Clone,
{
    type A = Dst;
    type Lifted<T: Clone> = Func<T, Dst>;
    fn fmap<F, B: Clone>(self, f: F) -> Lifted<T, B>
    where
        F: Fn(Src) -> B,
    {
        Func {
            inner: |r| self.inner.call(f(r)),
            _marker_a: PhantomData,
            _marker_b: PhantomData,
        }
    }
}
