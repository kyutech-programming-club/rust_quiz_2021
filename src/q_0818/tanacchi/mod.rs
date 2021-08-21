use std::io::{Read, Write};
use anyhow::Result;

#[allow(dead_code)]
fn main(read_buf: &mut impl Read, write_buf: &mut impl Write) -> Result<()> {
    let mut s = String::new();
    read_buf.read_to_string(&mut s)?;
    let n = s.trim().parse::<i32>()?.rem_euclid(2);
    writeln!(write_buf, "{}", n)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        main();
    }
}
