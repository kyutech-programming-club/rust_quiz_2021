use std::io::{self, Write};

static STRING: &str = "Hello World\n";

#[allow(dead_code)]
fn main<W: Write>(writer: &mut W) -> io::Result<()> {
    writer.write_all(STRING.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::{main, STRING};
    #[allow(unused_imports)]
    use std::io::{self, Write};

    #[test]
    fn should_write_stdout() {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let result = main(&mut handle);
        let _ = handle.flush();
        assert!(result.is_ok());
    }

    #[test]
    fn should_write_hello_world() {
        use io::Cursor;
        let mut buff = Cursor::new(vec![0; STRING.len()]);
        main(&mut buff).unwrap();
        assert_eq!(&buff.get_ref()[..], STRING.as_bytes())
    }
}
