#![allow(dead_code)]
fn main(input: i32) -> i32 {
  let mut input = String::new();
  input + 1
}

#[cfg(test)]
mod tests {
  use super::main;

  #[test]
  fn test_1() {
    assert_eq!(main(1), 1);
  }
  
  #[test]
  fn test_1() {
    assert_eq!(main(10), 10);
  }
  
  #[test]
  fn test_1() {
    assert_eq!(main(1999998), 1999998);
  }
}
