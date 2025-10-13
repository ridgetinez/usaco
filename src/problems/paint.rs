use std::{
    cmp::max,
    io::{self, BufRead, Write},
    mem::swap,
};

use cp_macros::competitive_problem;

use crate::problems::util::parsing::parse_line;

#[competitive_problem(input = "paint.in", output = "paint.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let mut ab: [usize; 2] = parse_line(&mut lines)?.try_into().unwrap();
    let mut cd: [usize; 2] = parse_line(&mut lines)?.try_into().unwrap();

    // The first index value should begin to the left of the right index value.
    if ab[0] > ab[1] {
        ab.swap(0, 1);
    }
    if cd[0] > cd[1] {
        cd.swap(0, 1)
    }

    // ab should contain the range that has the left-most index
    if cd[0] < ab[0] {
        swap(&mut cd, &mut ab);
    }

    if has_intersection(&ab, &cd) {
        _ = writeln!(output, "{}", max(ab[1], cd[1]) - ab[0]);
    } else {
        _ = writeln!(output, "{}", ab[1] - ab[0] + cd[1] - cd[0]);
    }
    Ok(())
}

// `ab` is guaranteed to have the start of the range less than the start of the `cd` range
fn has_intersection(ab: &[usize; 2], cd: &[usize; 2]) -> bool {
    ab[1] > cd[0]
}
