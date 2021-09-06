#![allow(dead_code)]

fn fibo(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fibo(n - 1) + fibo(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    #[case(5, 5)]
    #[case(6, 8)]
    #[case(7, 13)]
    fn 正_フィボナッチ数を計算する(#[case] n: usize, #[case] expected: usize) {
        assert_eq!(fibo(n), expected);
    }
}
