#![allow(dead_code)]

fn str_eql(s1: &str, s2: &str) -> bool {
    s1 == s2
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("hello", "hello")]
    #[case(" a b ", " a b ")]
    fn 正_文字列の等しさを判定する(#[case] s1: &str, #[case] s2: &str) {
        assert_eq!(str_eql(s1, s2), true);
    }
    #[rstest]
    #[case("hello", "hello1")]
    #[case(" a b ", "ab")]
    fn 異_文字列の等しさを判定する(#[case] s1: &str, #[case] s2: &str) {
        assert_eq!(str_eql(s1, s2), false);
    }
}
