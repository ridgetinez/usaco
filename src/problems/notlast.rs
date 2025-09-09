use std::{
    collections::BTreeMap,
    io::{self, BufRead, Write},
};

use cp_macros::competitive_problem;

#[competitive_problem(input = "notlast.in", output = "notlast.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let _: usize = lines.next().unwrap()?.parse().unwrap();
    let milk_volumes: BTreeMap<String, usize> = lines
        .map(|maybe_line| {
            let line = maybe_line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].to_string(), parts[1].parse().unwrap())
        })
        .fold(BTreeMap::new(), |mut acc, (name, volume)| {
            acc.entry(name)
                .and_modify(|v| *v += volume)
                .or_insert(volume);
            acc
        });

    let all_cows_producing = milk_volumes.keys().count() == 7;

    let volume_mapping = milk_volumes
        .into_iter()
        .map(|(name, volume)| (volume, name))
        .fold(BTreeMap::new(), |mut acc, (volume, name)| {
            acc.entry(volume)
                // shoddy clone happens below that doesn't need to happen.
                .and_modify(|names: &mut Vec<String>| names.push(name.clone()))
                .or_insert(Vec::from([name]));
            acc
        });

    // println!("{:?}", volume_mapping);

    let maybe_second_last_producers = if all_cows_producing {
        volume_mapping.iter().nth(1)
    } else {
        volume_mapping.iter().nth(0)
    };
    _ = writeln!(
        output,
        "{}",
        if maybe_second_last_producers.is_none() || maybe_second_last_producers.unwrap().1.len() > 1
        {
            "Tie"
        } else {
            &maybe_second_last_producers.unwrap().1[0]
        }
    );

    Ok(())
}
