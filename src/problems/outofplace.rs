use std::{
    io::{self, BufRead, Write},
    mem::swap,
};

use cp_macros::competitive_problem;

#[competitive_problem(input = "outofplace.in", output = "outofplace.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    _ = lines.next();
    let mut heights: Vec<usize> = lines
        .map(|maybe_line| maybe_line.unwrap().parse().unwrap())
        .collect();
    // Build the solution right to left, finding the left-most maximum inversion to swap each round.
    let mut nswaps = 0;
    for i in (0..heights.len()).rev() {
        if let Some(swap_index) = (0..i)
            .rev()
            .max_by_key(|&j| heights[j])
            .filter(|&j| heights[i] < heights[j])
        {
            // println!("swapping {} and {}", i, swap_index);
            heights.swap(i, swap_index);
            nswaps += 1;
            // println!("{:?}", heights);
        }
    }
    _ = writeln!(output, "{}", nswaps);
    Ok(())
}
