use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
    str::FromStr,
};

use cp_macros::competitive_problem;

fn parse_line<T: FromStr + Default>(
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> io::Result<Vec<T>> {
    Ok(lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap_or_default())
        .collect::<Vec<T>>())
}

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let [_, target]: [i64; 2] = parse_line::<i64>(&mut lines)?.try_into().unwrap();
    let xs: Vec<i64> = parse_line(&mut lines)?;
    let mut seen = HashMap::<i64, usize>::new();
    for (i, x) in xs.iter().enumerate() {
        if seen.contains_key(&(target - x)) {
            _ = writeln!(output, "{} {}", i + 1, seen.get(&(target - x)).unwrap());
            return Ok(());
        } else {
            seen.insert(*x, i + 1);
        }
    }
    _ = writeln!(output, "IMPOSSIBLE");
    Ok(())
}
