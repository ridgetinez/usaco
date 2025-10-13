use crate::problems::util::parsing::parse_line;
use std::cmp::{max, min};
use std::convert::TryInto;
use std::io::{self, BufRead, Write};
use std::mem::swap;

use cp_macros::competitive_problem;

fn minimum_side_length(logo: &[usize; 2]) -> usize {
    min(logo[0], logo[1])
}

fn is_stackable(billboard_dimension: usize, logos: &[[usize; 2]]) -> bool {
    logos.iter().all(|logo| logo.contains(&billboard_dimension))
        && logos.iter().map(minimum_side_length).sum::<usize>() == billboard_dimension
}

fn is_horizontal_arrangeable(
    billboard_dimension: usize,
    remaining_side: usize,
    logos: &[[usize; 2]],
) -> bool {
    // 1 6
    // heights
    false
}

struct Point {
    x: usize,
    y: usize,
    ch: char,
}

impl Point {
    fn print(self, output: &mut Box<dyn Write>) {
        (0..self.y).for_each(|_| _ = writeln!(output, "{}", self.ch.to_string().repeat(self.x)))
    }
}

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let d = parse_line::<usize>(&mut lines)?;
    let mut a = Point {
        x: d[0],
        y: d[1],
        ch: 'A',
    };
    let mut b = Point {
        x: d[2],
        y: d[3],
        ch: 'B',
    };
    let mut c = Point {
        x: d[4],
        y: d[5],
        ch: 'C',
    };

    // Put the longest dimension as the `x` field to reduce cases.
    // Geometrically, this is a rotation so that the width is the longest dimension.
    if a.y > a.x {
        swap(&mut a.x, &mut a.y)
    }
    if b.y > b.x {
        swap(&mut b.x, &mut b.y)
    }
    if c.y > c.x {
        swap(&mut c.x, &mut c.y)
    }

    // Now swap the logos such that `a` has the longest dimension across all logos.
    // This will be used as the "first" logo to place.
    if c.x > b.x {
        swap(&mut c, &mut b);
    }
    if b.x > a.x {
        swap(&mut b, &mut a);
    }

    // If all logos share there longest dimension, they can be stacked.
    if a.x == b.x && b.x == c.x {
        // If they form a square with their other dimension, this is a solution!
        if a.y + b.y + c.y == a.x {
            _ = writeln!(output, "{}", a.x);
            a.print(&mut output);
            b.print(&mut output);
            c.print(&mut output);
        } else {
            _ = writeln!(output, "-1");
        }
        return Ok(());
    }

    // Otherwise try horizontally arranging `b` and `c` below `a`.
    let remaining_y = a.x - a.y;
    // We want the `x` to be the width, where remaining_y should be the height (or `y`) of
    // the two logos we have left to place.
    if b.x == remaining_y {
        swap(&mut b.x, &mut b.y)
    }
    if c.x == remaining_y {
        swap(&mut c.x, &mut c.y)
    }
    if b.x + c.x == a.x && b.y == remaining_y && c.y == remaining_y {
        _ = writeln!(output, "{}", a.x);
        a.print(&mut output);
        (0..remaining_y).for_each(|_| {
            _ = write!(&mut output, "{}", b.ch.to_string().repeat(b.x));
            _ = write!(&mut output, "{}", c.ch.to_string().repeat(c.x));
            _ = writeln!(&mut output, "");
        });
        return Ok(());
    }

    // The above two cases are the only way to place three logos with the given conditions.
    _ = writeln!(output, "-1");
    Ok(())

    // // NOTE(amartinez): Opportunity to write a group function
    // let mut logos: [([usize; 2], char); 3] = dimensions
    //     .chunks_exact(2)
    //     .map(|chunk| {
    //         chunk
    //             .into_iter()
    //             .map(|x| x.clone())
    //             .collect::<Vec<usize>>()
    //             .try_into()
    //             .unwrap()
    //     })
    //     .zip("ABC".chars())
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .unwrap();
    // // sort descending of dimension length
    // logos.sort_by_key(|logo| max(logo.0[0], logo.0[1]));
    // logos.reverse();
    // // writeln!("({},{}) | ({},{}) | ({},{})", l1, w1, l2, w2, l3, w3);
    // // writeln!("{:?}", logos);
    // let billboard_dimension: usize = max(logos[0].0[0], logos[0].0[1]);
    // let logo_dimensions: Vec<_> = logos.iter().map(|logo| logo.0).collect();

    // // swap to ensure that the longer dimension is the second.
    // // do this for all logos if a swap was done
    // // this is like doing a rotation.

    // // write it out with if statements non general so the printing is easier
    // let mut remaining_billboard = (
    //     billboard_dimension,
    //     billboard_dimension - min(logos[0].0[0], logos[0].0[1]),
    // );

    // // Check if we can stack on top of each other
    // // Can be aligned against the length of the square
    // // The sum of the other side (which is the min as we've found the global max) is equal to the billboard side length.
    // if is_stackable(billboard_dimension, &logo_dimensions) {
    //     _ = writeln!(output, "{}", billboard_dimension);
    //     logos.iter().for_each(|(logo, c)| {
    //         (0..minimum_side_length(logo))
    //             .for_each(|_| _ = writeln!(output, "{}", c.to_string().repeat(billboard_dimension)))
    //     });
    // } else if logos.iter().skip(1).all(|logo| {
    //     logo.0
    //         .iter()
    //         .find(|&&x| x == remaining_billboard.1)
    //         .is_some()
    // }) && logos
    //     .iter()
    //     .skip(1)
    //     .map(|logo| {
    //         if logo.0[0] == remaining_billboard.1 {
    //             logo.0[1]
    //         } else {
    //             logo.0[0]
    //         }
    //     })
    //     .sum::<usize>()
    //     == billboard_dimension
    // {
    //     writeln!(output, "{}", billboard_dimension);
    //     let (first_logo, c) = logos[0];
    //     (0..min(first_logo[0], first_logo[1]))
    //         .for_each(|_| _ = writeln!(output, "{}", c.to_string().repeat(billboard_dimension)));
    //     (0..remaining_billboard.1).for_each(|_| {
    //         logos
    //             .iter()
    //             .skip(1)
    //             .map(|(logo, c)| {
    //                 (
    //                     if logo[0] == remaining_billboard.1 {
    //                         logo[1]
    //                     } else {
    //                         logo[0]
    //                     },
    //                     c,
    //                 )
    //             })
    //             .for_each(|(n, c)| _ = write!(output, "{}", c.to_string().repeat(n)));
    //         _ = writeln!(output, "");
    //     })
    // } else {
    //     _ = writeln!(output, "-1");
    // }
    // Ok(())
}
