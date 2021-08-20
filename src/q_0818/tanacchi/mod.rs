use anyhow::Result;

#[allow(dead_code)]
fn main() -> Result<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    let n = s.trim().parse::<i32>()?.rem_euclid(2);
    println!("{}", n);
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        main();
    }
}
