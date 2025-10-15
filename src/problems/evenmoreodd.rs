use std::io::{self, BufRead, Write};

use cp_macros::competitive_problem;

use crate::problems::util::parsing::parse_line;

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let _ = lines.next();
    let ids: Vec<usize> = parse_line(&mut lines)?;
    let (evens, odds): (Vec<usize>, Vec<usize>) = ids.into_iter().partition(|n| n % 2 == 0);
    let (nevens, nodds) = (evens.len(), odds.len());
    if nevens >= nodds {
        // every odd number can be paired with an even number, the remaining even numbers
        // join the other evens as their sum is still an even number.
        _ = writeln!(
            output,
            "{}",
            (2 * nodds) + if nevens > nodds { 1 } else { 0 } // add an additional group if we still have an even left.
        );
    } else {
        // Crazy observation from post-analysis:
        // Instead of having a specific case for nodd > neven, we notice that 2 odd is equivalent
        // to 1 even which we can use to increase neven while decreasing nodd by 2. Do this
        // until it decomposes down to the neven >= nodd case which is simpler to think about.

        //
        // pair every even number with an odd number, then for the remaining odds,
        // it takes 3 odd numbers to make an (even, odd) group.
        let remaining_odds = nodds - nevens;
        // println!("{}", remaining_odds / 3);
        _ = writeln!(
            output,
            "{}",
            2 * nevens
                + match remaining_odds % 3 {
                    0 => 2 * (remaining_odds / 3),
                    // this last odd can't be put anywhere without breaking the even / odd chain,
                    // so we walk back the last odd group and put them together in some even branch.
                    1 => 2 * (remaining_odds / 3) - 1,
                    2 => 2 * (remaining_odds / 3) + 1,
                    _ => panic!("wtf"),
                }
        );
    }
    Ok(())
}
