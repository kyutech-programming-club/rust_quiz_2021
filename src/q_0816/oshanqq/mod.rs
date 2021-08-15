use std::io::{self, Write};

#[allow(dead_code)]
fn output_hello<W: Write>(output: &mut W) -> io::Result<()> {
  writeln!(output, "Hello, World!")
}

#[test]
pub fn test_hello() {
  let mut buffer = vec![];
  let result = output_hello(&mut buffer);
  assert!(result.is_ok());
  assert_eq!("Hello, World!\n".as_bytes(), buffer);
}