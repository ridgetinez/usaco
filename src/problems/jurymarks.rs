use std::{
    collections::HashSet,
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
    let _: Vec<usize> = parse_line(&mut lines)?; // parse `k n` but since our collections are sized, we don't need this.
    let jury_scores: Vec<i32> = parse_line(&mut lines)?;
    let observations: HashSet<i32> = parse_line::<i32>(&mut lines)?.into_iter().collect();
    let first_observation = observations.iter().next().copied().unwrap();
    let k = jury_scores.len();
    let count = (1..k + 1)
        .map(|pos| generate_intermediate_score_set(&jury_scores, k, first_observation, pos))
        .filter(|scores| observations.is_subset(&scores.clone().into_iter().collect()))
        .map(|scores| scores[0] - jury_scores[0])
        .collect::<HashSet<i32>>()
        .len();

    _ = writeln!(output, "{}", count);
    Ok(())
}

fn generate_intermediate_score_set(
    scores: &Vec<i32>,
    k: usize,
    observed_score: i32,
    pos: usize,
) -> Vec<i32> {
    let prefix: Vec<_> = (1..pos)
        .rev()
        .scan(observed_score, |intermediate, i| {
            *intermediate -= scores[i];
            Some(*intermediate)
        })
        .collect();
    let suffix: Vec<_> = (pos..k)
        .scan(observed_score, |intermediate, i| {
            *intermediate += scores[i];
            Some(*intermediate)
        })
        .collect();
    let mut all_scores = prefix.clone();
    all_scores.extend([observed_score].iter());
    all_scores.extend(suffix.iter());
    // println!("pos = {}, k = {} :: {:?}", pos, k, all_scores);
    all_scores
}
