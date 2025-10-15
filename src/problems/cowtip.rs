use std::io::{self, BufRead, Write};

use cp_macros::competitive_problem;

// use crate::problems::util::printing::print_grid;

#[competitive_problem(input = "cowtip.in", output = "cowtip.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let mut grid: Vec<Vec<bool>> = lines
        .map(|maybe_line| {
            maybe_line
                .unwrap()
                .chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("unexpected char"),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut ntips = 0;
    for i in (0..n).rev() {
        // check bottom right corner of the ixi square
        if grid[i][i] {
            invert(&mut grid, i, i);
            ntips += 1;
        }
        // check column and rows together
        // this is safe as rectangles ixj and jxi
        // don't intersect each other on column i and row i
        // which are the "lines" we care about on this iteration.
        for j in (0..i).rev() {
            // don't need to check if further subsequences
            // should be included as they will be caught by
            // this invert on [j][i]. Similarly with [i][j].
            if grid[j][i] {
                invert(&mut grid, j, i);
                ntips += 1;
            }
            if grid[i][j] {
                invert(&mut grid, i, j);
                ntips += 1;
            }
        }
    }

    _ = writeln!(output, "{}", ntips);
    Ok(())
}

fn invert(grid: &mut Vec<Vec<bool>>, di: usize, dj: usize) {
    for i in 0..(di + 1) {
        for j in 0..(dj + 1) {
            grid[i][j] = !grid[i][j];
        }
    }
    // print_grid(&grid);
    // println!("=========")
}
