#![allow(dead_code)]

use anyhow::{anyhow, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    logic(&mut stdout)
}

fn logic(dst: &mut impl io::Write) -> Result<()> {
    (1..=100)
        .map(|n| match n {
            n if n % 15 == 0 => "FizzBuzz".to_owned(),
            n if n % 3 == 0 => "Fizz".to_owned(),
            n if n % 5 == 0 => "Buzz".to_owned(),
            _ => format!("{}", n),
        })
        .try_for_each(|s| writeln!(dst, "{}", s).map_err(|e| anyhow!(e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use difference::assert_diff;
    #[test]
    #[allow(non_snake_case)]
    fn 正_1から100までのFizzBuzzを出力する() {
        let mut buf = vec![];
        let res = logic(&mut buf);
        assert!(res.is_ok());

        // writelnで出力されているのでfrom_utf8がerrorになることはない
        let buf = std::str::from_utf8(&buf[..]).unwrap();

        for (i, s) in buf.lines().enumerate() {
            let n = i + 1;
            match n {
                n if n % 15 == 0 => assert_diff!(s, "FizzBuzz", "\n", 0),
                n if n % 3 == 0 => assert_diff!(s, "Fizz", "\n", 0),
                n if n % 5 == 0 => assert_diff!(s, "Buzz", "\n", 0),
                _ => {
                    let expected = format!("{}", n);
                    assert_diff!(s, &expected, "\n", 0);
                }
            }
        }
    }
}
