use std::io::Write;

#[allow(dead_code)]
fn hello(w: &mut dyn Write) {
  writeln!(w, "Hello World");
}

#[cfg(test)]
mod tests {
  use super::hello;

  #[test]
  fn test_hello() {
      let mut buf = vec![];
      hello(&mut buf);
      assert_eq!(buf, b"Hello World\n");
  }
}