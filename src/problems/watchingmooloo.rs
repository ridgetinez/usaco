use std::io::{self, BufRead, Write};

use cp_macros::competitive_problem;

use crate::problems::util::parsing::parse_line;

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let [n, k]: [u64; 2] = parse_line(&mut lines)?.try_into().unwrap();
    let days: Vec<u64> = parse_line(&mut lines)?;
    let mut min_cost = k + 1; // for taking the first day
    for i in 1..n {
        let day_delta = days[i as usize] - days[(i - 1) as usize];
        if day_delta > k {
            min_cost += k + 1;
        } else {
            min_cost += day_delta;
        }
    }
    _ = writeln!(output, "{}", min_cost);
    Ok(())
}
