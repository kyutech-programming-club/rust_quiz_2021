#![allow(dead_code)]

use super::super::q_0904::sagoj0_::my_rand_interface;
use rand::{thread_rng, RngCore};

fn randf_interface(rng: &mut impl RngCore) -> f64 {
    my_rand_interface(10, rng) as f64 / 10.
}

fn randf() -> f64 {
    let mut rng = thread_rng();
    randf_interface(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::rngs::mock::StepRng;
    use rstest::rstest;

    #[rstest]
    #[case(5, 0.5)]
    #[case(12, 0.2)]
    #[case(100, 0.)]
    fn 正_乱数を生成する(#[case] random: u64, #[case] expected: f64) {
        let mut rng = StepRng::new(random, 1);
        let result = randf_interface(&mut rng);

        assert_relative_eq!(result, expected);
    }
}
