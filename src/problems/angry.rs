use std::io::{self, BufRead, Write};

use cp_macros::competitive_problem;

fn num_exploded_bales(bale_positions: &Vec<u32>, starting_index: usize) -> usize {
    let mut seen = bale_positions.iter().map(|_| false).collect::<Vec<bool>>();
    let mut frontier = Vec::from([starting_index]);
    seen[starting_index] = true;
    let mut t = 1;
    // for each exploded bale at i
    // println!(
    //     "launching at position {} (index {})",
    //     bale_positions[starting_index], starting_index
    // );
    while frontier.len() > 0 {
        // println!("\t{:?} t={}", frontier, t);
        let mut next_frontier = Vec::new();
        frontier.iter().for_each(|i| {
            // find all unexploded within range [max(0,i-t), min(len,i+t)]
            let lower_bound =
                bale_positions.partition_point(|&x| x < bale_positions[*i].saturating_sub(t));
            let upper_bound = bale_positions.partition_point(|&x| {
                // println!("\t\tx = {}, cmp = {}", x, bale_positions[*i] + t);
                x <= bale_positions[*i] + t
            });
            // println!("\t[{},{})", lower_bound, upper_bound);

            (lower_bound..upper_bound).for_each(|i| {
                if !seen[i] {
                    // propagate to the next layer
                    next_frontier.push(i);
                    // mark them
                    seen[i] = true;
                }
            })
        });
        frontier = next_frontier;
        t += 1;
    }
    // finally: count how many were marked
    // why do we have an reference if we're moving the values?
    seen.into_iter().filter(|&x| x).count()
}

#[competitive_problem(input = "angry.in", output = "angry.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let mut bale_positions: Vec<u32> = lines.map(|l| l.unwrap().parse().unwrap()).collect();
    bale_positions.sort();
    // println!("{:?}", bale_positions);
    _ = writeln!(
        output,
        "{}",
        (0..n)
            .map(|i| num_exploded_bales(&bale_positions, i))
            .max()
            .unwrap()
    );
    Ok(())
}
