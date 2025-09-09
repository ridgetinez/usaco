use std::{
    collections::BTreeSet,
    io::{self, BufRead, Write},
};

use cp_macros::competitive_problem;

fn board_lines(board: &Vec<Vec<char>>) -> Vec<String> {
    // horizontal
    let n = board.len();
    let mut res: Vec<String> = board.iter().map(|chars| String::from_iter(chars)).collect();
    // vertical
    res.extend((0..n).map(|j| (0..n).map(|i| board[i][j]).collect::<String>()));
    // diagonal
    res.push((0..n).map(|i| board[i][i]).collect::<String>());
    res.push((0..n).map(|i| board[i][n - i - 1]).collect::<String>());
    res
}

fn winner_count(board_lines: &Vec<String>, team_sz: usize) -> usize {
    board_lines
        .iter()
        .map(|line| BTreeSet::<char>::from_iter(line.chars()))
        .filter(|s| s.len() == team_sz)
        .collect::<BTreeSet<BTreeSet<char>>>() // I didn't have this before, which was duplicating same teams.
        .len()
}

#[competitive_problem(input = "tttt.in", output = "tttt.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let board: Vec<Vec<char>> = input
        .lines()
        .map(|maybe_s| maybe_s.unwrap().chars().collect())
        .collect();
    let board_lines = board_lines(&board);
    _ = writeln!(output, "{}", winner_count(&board_lines, 1));
    _ = writeln!(output, "{}", winner_count(&board_lines, 2));
    Ok(())
}
