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

// Assumed weights are sorted
fn calculate_instability(weights: &[usize]) -> usize {
    weights
        .chunks(2)
        .map(|xs: &[usize]| xs[0].abs_diff(xs[1]))
        .sum()
}

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let _n = lines.next().unwrap()?;
    let mut weights = parse_line::<usize>(&mut lines)?;
    weights.sort();

    let min_instability = (0..weights.len())
        .flat_map(|i| (i + 1..weights.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let mut v = Vec::new();
            v.extend_from_slice(&weights[0..i]);
            v.extend_from_slice(&weights[i + 1..j]);
            v.extend_from_slice(&weights[j + 1..]);
            calculate_instability(&v)
        })
        .min()
        .unwrap();

    output.write_all(min_instability.to_string().as_bytes())?;
    Ok(())
}
