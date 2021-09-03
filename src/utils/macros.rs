//! 任意のマクロを配置する．
#![allow(unused_macros)]

/// anyhowのエラーをダウンキャストし，
/// 実際のエラー値を検証する．
/// (written by `sagoj0_`)
///
/// ## Example
/// ```
/// #[test]
/// fn should_error_when_parse() {
///     let result = Err(LackOfInputOnParseError);
///     assert_eq!(result.is_err(), true);
///     let error = result.unwrap_err();
///     assert_error_match!(error, QuizSolveError, LackOfInputOnParseError);    
/// }
/// ```
#[macro_export]
macro_rules! assert_error_match {
    ($error:expr, $error_type:ty, $expected:expr) => {{
        pretty_assertions::assert_eq!($error.is::<$error_type>(), true);
        let error = $error.downcast_ref::<$error_type>().unwrap();
        pretty_assertions::assert_eq!(*error, $expected);
    }};
}

/// and_thenメソッドが実装されたmonadに対してdo記法を再現する．
/// 不明点はHaskellのdo記法やScalaのfor-yieldを参照されたし．
///(written be sagoj0_)
///
/// ## Example
/// ```
/// let pure = |x| Some(x);
/// let result = mdo! {
///     x <- pure(1), // x: usize == 1
///     y <- pure(2), // y: usize == 2
///     z <- pure(3), // z: usize == 3
///     pure(x + y + z)
/// }
/// assert_eq!(result, Some(6));
/// ```
///
/// パターンにすると<-演算子が使えなくなるため=<<記号にせざるを得ない．
/// 現状パターンを利用するか不明なため，利用する際に実装を追加する．
/// ```
/// ($p: pat =<< $e: expr , $( $t: tt )*) => {
///     $e.and_then(move |$p| mdo! { $( $t )* })
/// };
/// ```
#[macro_export]
macro_rules! mdo {
    ($i: ident <= $e:expr , $($t:tt)*) => {
        $e.and_then(move |$i| mdo!($($t)*))
    };
    ($e: expr, $($t:tt)*) => {
        $e.and_then(move |_| mdo!($($t)*))
    };
    (ret $e: expr) => (pure($e));
    ($e: expr) => ($e);
}
