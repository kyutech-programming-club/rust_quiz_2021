#![allow(dead_code)]

use crate::utils::sagoj0_::io_util;
use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<usize> {
    Ok(input.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn 正_文字列の長さを数える() {
        let (input, expected) = ("hello", 5);
        let result = logic(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
