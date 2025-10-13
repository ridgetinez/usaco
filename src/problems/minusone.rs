use std::io::{self, BufRead, Write};

use cp_macros::competitive_problem;

use crate::problems::util::parsing::parse_line;

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let specs: Vec<usize> = parse_line(&mut lines)?;
    let (n, m) = (specs[0], specs[1]);
    // "reverse-mario" stairs where:
    // - symbol_rules[i][i] is always 'D' for draw.
    // - symbol_rules[i][j] is defined where j <= i
    //  - if s[i][j] == 'L' then s[j][i] must be 'W'
    //  - if s[i][j] == 'W' then s[j][i] must be 'L'
    let mut symbol_rules: Vec<Vec<char>> = lines
        .by_ref()
        .take(n)
        .map(|line| {
            let mut rule_line = Vec::new();
            rule_line.extend(line.unwrap().chars());
            // Define defaults for the backwards relation which we'll populate
            // after reading all the forward relations.
            rule_line.extend("D".repeat(n - rule_line.len()).chars());
            rule_line
        })
        .collect();
    // define the reverse result
    (0..n).for_each(|i| {
        (i..n).for_each(|j| symbol_rules[i][j] = reverse_rule_result(symbol_rules[j][i]));
        // println!("{:?}", symbol_rules[i]);
    });

    lines
        .take(m)
        .map(|line| {
            let [mut li, mut ri] = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
            // li,ri are one indexed
            li -= 1;
            ri -= 1;

            let n_winning_symbols = symbol_rules[li]
                .iter()
                .zip(symbol_rules[ri].iter())
                .enumerate()
                .filter(|(_, (cl, cr))| cl == cr && **cl == 'L')
                .map(|(i, _)| i)
                .collect::<Vec<_>>()
                .len();
            // pick from winning, then pick from non-winning, repeat for other side (doubles)
            // OR
            // pick both the same winning (symmetrical, so no doubling)
            // OR
            // pick both from winning, (since not disjoint sets, other side would just repeat an already seen combination so no doubling)
            // ======================================
            // (wlen * (n-wlen)) * 2
            // + wlen
            // + (wlen * (wlen-1))*2
            // TODO(amartinez): Learn more about counting.
            n_winning_symbols * (n - n_winning_symbols) * 2
                + n_winning_symbols
                // check at least one winning symbol to prevent overflow
                + if n_winning_symbols > 0 {
                    n_winning_symbols * (n_winning_symbols - 1)
                } else {
                    0
                }
            // 2 1
            // 3 1
            // 1 2
            // 1 3
            // 2 2
            // 3 3
            // 2 3
            // 3 2
        })
        .for_each(|ncombinations| _ = writeln!(output, "{}", ncombinations));
    Ok(())
}

fn reverse_rule_result(result: char) -> char {
    match result {
        'D' => 'D',
        'W' => 'L',
        'L' => 'W',
        _ => panic!("unexpected result code"),
    }
}
