#![allow(dead_code)]

fn interrupt_with_one(n: &mut usize) {
    let s = format!("{}", n);
    let len = s.len();

    let s = "1".repeat(len) + s.as_str() + "1".repeat(len).as_str();
    *n = s.parse::<usize>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(23, 112311)]
    #[case(123, 111123111)]
    fn 正_nの値を正しく変更する(#[case] input: usize, #[case] expected: usize) {
        let mut n = input;
        interrupt_with_one(&mut n);
        assert_eq!(n, expected);
    }
}
