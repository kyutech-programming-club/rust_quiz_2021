use std::io::{Read, Write};
use std::fmt::Display;
use std::ops::FnOnce;
use anyhow::Result;

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
