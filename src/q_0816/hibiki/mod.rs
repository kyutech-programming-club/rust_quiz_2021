#![allow(dead_code)]
use std::io::{self, Write};

fn main<W: Write>(w: &mut W) -> io::Result<()> {
  writeln!(w, "Hello, World!")
}

#[cfg(test)]
mod tests {
  use super::main;

  #[test]
  fn test_hello_world() {
      let mut buf = vec![];
      let result = main(&mut buf);
      assert!(result.is_ok());
      assert_eq!(buf, b"Hello, World!\n");
  }
}