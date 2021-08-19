use std::io::{self, BufRead, Write};

#[allow(dead_code)]
fn main(read_buf: &mut impl BufRead, write_buf: &mut impl Write) -> io::Result<()> {
    // Read from buf.
    let mut s = String::new();
    read_buf.read_line(&mut s)?;
    // Parse into i32 and Add 1.
    let n = s.trim()
             .parse::<i32>()
             .map(|n| n + 1)
             .map_err(|_| io::ErrorKind::InvalidInput)?;
    // Write to buf.
    write_buf.write_all(n.to_string().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::main;
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
