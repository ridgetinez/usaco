use std::io::{self, BufRead, Write};

use cp_macros::competitive_problem;

#[competitive_problem(input = "breedflip.in", output = "breedflip.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    _ = lines.next();
    let original = lines.next().unwrap()?;
    let mutated = lines.next().unwrap()?;
    let mut nflips = 0;
    let mut ndifferent = 0;

    // Commit the flip when we've subsequence of letters that should be flipped is terminated.
    for (co, cm) in original.chars().zip(mutated.chars()) {
        if co == cm && ndifferent > 0 {
            nflips += 1;
            ndifferent = 0;
        } else if co != cm {
            ndifferent += 1
        }
    }
    // We ended on a subsequence that's yet to be flipped, commit the last one.
    if ndifferent > 0 {
        nflips += 1;
    }

    writeln!(output, "{}", nflips);
    Ok(())
}
