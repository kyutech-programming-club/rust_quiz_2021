#![allow(dead_code)]
use std::io;
use anyhow::Result;

use crate::utils::sagoj0_::io_util;

fn logic(input: &str) -> Result<f64> {
    let r = input.parse::<f64>()?;
    let pi: f64 = 3.14;

    let volume = 4.0 / 3.0 * pi * r * r * r;
    Ok(volume)
}

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)?;
    Ok(())
}
