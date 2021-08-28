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
