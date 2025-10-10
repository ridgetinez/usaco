use std::io::{self, BufRead, Write};
use std::cmp::{max,min};
use crate::problems::util::parsing::parse_line;

use cp_macros::competitive_problem;

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let dimensions = parse_line::<usize>(&mut lines)?;
    // NOTE(amartinez): Opportunity to write a group function
    let mut logos: [([usize;2],char); 3] = dimensions
        .chunks_exact(2)
        .map(|chunk| chunk.into_iter().map(|x| x.clone()).collect::<Vec<usize>>().try_into().unwrap())
        .zip("ABC".chars())
        .collect::<Vec<_>>().try_into().unwrap();
    // sort descending of dimension length
    logos.sort_by_key(|logo| max(logo.0[0], logo.0[1]));
    logos.reverse();
    // println!("({},{}) | ({},{}) | ({},{})", l1, w1, l2, w2, l3, w3);
    println!("{:?}", logos);
    let billboard_dimension: usize = max(logos[0].0[0], logos[0].0[1]);

    // write it out with if statements non general so the printing is easier
    let mut remaining_billboard = (billboard_dimension, billboard_dimension - min(logos[0].0[0], logos[0].0[1]));

    // Check if we can stack on top of each other
    // Can be aligned against the length of the square
    // The sum of the other side (which is the min as we've found the global max) is equal to the billboard side length.
    if
        logos.iter().all(|logo| logo.0.contains(&billboard_dimension))
        && logos.iter().map(|logo| min(logo.0[0], logo.0[1])).sum::<usize>() == billboard_dimension
    {
        writeln!(output, "{}", billboard_dimension);
        logos.iter().for_each(|(logo,c)| (0..min(logo[0], logo[1])).for_each(|_| println!("{}", c.to_string().repeat(billboard_dimension))));
    }
    else if logos.iter().skip(1).all(|logo| logo.0.iter().find(|&&x| x == remaining_billboard.1).is_some())
    && logos.iter().skip(1).map(|logo| if logo.0[0] == remaining_billboard.1 { logo.0[1] } else { logo.0[0] }).sum::<usize>() == billboard_dimension {
        writeln!(output, "{}", billboard_dimension);
        let (first_logo,c) = logos[0];
        (0..min(first_logo[0], first_logo[1])).for_each(|_| println!("{}", c.to_string().repeat(billboard_dimension)));
        (0..remaining_billboard.1)
            .for_each(|_| {
                logos.iter().skip(1)
                    .map(|(logo,c)| (if logo[0] == remaining_billboard.1 { logo[1] } else { logo[0] },c))
                    .for_each(|(n,c)| print!("{}", c.to_string().repeat(n)));
                println!("")
            })

    } else {
        writeln!(output, "-1");
    }
    Ok(())
}
