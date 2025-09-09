// 3
// 5 4 -1
// 4 3 2
//
// 5 4 -1
// 3 2  4
//
//
//
// (_3*4) (_4*3) (_2*3)
// -1      4       5

// 1. How many times will a[i]*b[i] be computed?
// (i+1) * (n-i) = (ni-i) = (n-1)i
// index 0 = 1 * (n) = n
// index 1 = 2 * (n-1) = 2n-2
// index 2 = (3) * (3-2) = 3n-6
// 2. Using this can you simplify the summation expression?
//
// 15 + (15+8) + (15+8-4) = 57
// 8 + (8-4) = 12
// -4 = -4
// =============================
// 65
//
// 12 12 6
// -1 4  5
//
//
//

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
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let mut xs = parse_line::<i64>(&mut lines)?
        .iter()
        .enumerate()
        .map(|(i, x)| x * (i + 1) as i64 * (n - i) as i64)
        .collect::<Vec<i64>>();
    let mut ys = parse_line::<i64>(&mut lines)?;
    xs.sort();
    ys.sort();
    ys.reverse();
    _ = writeln!(
        output,
        "{}",
        xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum::<i64>()
    );
    // writeln!(output, "{:?}", xs);
    // writeln!(output, "{:?}", ys);
    Ok(())
}
