use std::io::Read;
use anyhow::Result;

#[allow(dead_code)]
fn main(read_buf: &mut impl Read) -> Result<i32> {
    // Read from buf.
    let mut s = String::new();
    read_buf.read_to_string(&mut s)?;
    // Parse into i32.
    let n = s.trim().parse::<i32>()?;
    // Add one.
    Ok(n + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn can_use_stdio() {
        let stdin = io::stdin();
        let mut read_buf = stdin.lock();
        let stdout = io::stdout();
        let mut write_buf = stdout.lock();

        let result = main(&mut read_buf, &mut write_buf);
        assert!(result.is_ok() || result.is_err());
    }
}
