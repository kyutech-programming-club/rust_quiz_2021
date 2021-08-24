use std::io::{Read, Write};
use std::fmt::Display;
use std::ops::FnOnce;
use anyhow::Result;

/**
 * src から入力を読んで
 * その入力を f に渡して呼び出し，
 * 返り値を dst に書き込む．
 *
 * src には [`std::io::Read`] が，
 * dst には [`std::io::Write`] が実装されている必要がある．
 * f は [`std::string::String`] を引数にとり
 * [`anyhow::Result`] を返す [`std::ops::FnOnce`] である．
 */
pub fn apply<T, F>(src: &mut impl Read, dst: &mut impl Write, f: F) -> Result<()>
where
T: Display,
F: FnOnce(String) -> Result<T> {
    let mut s = String::new();
    src.read_to_string(&mut s)?;
    let result = f(s)?;
    writeln!(dst, "{}", result)?;
    Ok(())
}
