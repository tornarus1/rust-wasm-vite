use std::env;

use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        return Err(anyhow!("Usage: calc <lhs> <rhs>")).context("incorrect number of args");
    }

    let lhs = args[1].parse::<i32>()?;
    let rhs = args[2].parse::<i32>()?;

    let result =
        calculator::div(lhs, rhs).with_context(|| format!("failed to divide {lhs} by {rhs}"))?;
    println!("{} / {} is {}", lhs, rhs, result);

    Ok(())
}
