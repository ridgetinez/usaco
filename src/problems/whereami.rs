use std::{
    collections::HashSet,
    io::{self, BufRead, Write},
};

use cp_macros::competitive_problem;

#[competitive_problem(input = "whereami.in", output = "whereami.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let colours = lines.next().unwrap()?;
    let unique_window_sz = (1..n + 1)
        .filter(|&window_sz| {
            let unique_windows = colours
                .chars()
                .collect::<Vec<char>>()
                .as_slice()
                .windows(window_sz)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<HashSet<String>>();
            // println!("{} {:?}", window_sz, unique_subsequences);
            // n+1-window_sz implies all windows are unique
            unique_windows.len() == n + 1 - window_sz
        })
        .next()
        .unwrap();
    _ = writeln!(output, "{}", unique_window_sz);
    Ok(())
}
