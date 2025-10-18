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
        let mut swap_index = i;
        // TODO(amartinez): Can use rposition to find the index to swap.
        for j in (0..i).rev() {
            // Check that we're not swapping identical heights, otherwise
            // take the left most _different from heights[i]_ height.
            if heights[i] != heights[j] && heights[j] >= heights[swap_index] {
                swap_index = j
            }
        }
        if swap_index != i {
            // println!("swapping {} and {}", i, swap_index);
            heights.swap(i, swap_index);
            nswaps += 1;
            // println!("{:?}", heights);
        }
    }
    _ = writeln!(output, "{}", nswaps);
    Ok(())
}
