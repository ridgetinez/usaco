use std::{
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
    let n: u64 = lines.next().unwrap()?.parse().unwrap();
    let mut costs = parse_line::<u64>(&mut lines)?;
    costs.sort();
    costs
        .iter()
        .enumerate()
        .map(|(i, cost)| ((n - (i as u64)) * cost, (n - (i as u64))))
        .max()
        .map(|(max_earnings, n_students)| {
            writeln!(output, "{} {}", max_earnings, max_earnings / n_students)
        });
    Ok(())
}
