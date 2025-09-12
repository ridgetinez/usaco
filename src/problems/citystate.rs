use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
};

use cp_macros::competitive_problem;

#[competitive_problem(input = "citystate.in", output = "citystate.out")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let _: usize = lines.next().unwrap()?.parse().unwrap();
    // Maintaining a count + map is probably more readable with a for loop.
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut occurrences: HashMap<(String, String), usize> = HashMap::new();
    for line in lines.map(|maybe_line| maybe_line.unwrap()) {
        let (city, state_code) = line.split_once(' ').unwrap();
        let city_code = city[..2].to_string();
        *occurrences
            .entry((state_code.to_string(), city_code.to_string()))
            .or_default() += 1;
        pairs.push((state_code.to_string(), city_code));
    }

    let special_pair_count = pairs
        .into_iter()
        .map(|(state_code, city_code)| {
            n_special_state_pairs_made(&occurrences, state_code, city_code)
        })
        .sum::<usize>()
        / 2;

    _ = writeln!(output, "{}", special_pair_count);
    Ok(())
}

// FLINT MI
//      X
// MIAMI FL
//
// CITY (non-unique) STATE (non-unique)
// STATE-CODE -> first two letters of capital city
// MI -> FL
//
// MIAMI FL
// m[city-two-letters][state-code] gives the number of pairs this line will make
fn n_special_state_pairs_made(
    state_to_city_codes: &HashMap<(String, String), usize>,
    state_code: String,
    city_code: String,
) -> usize {
    // don't count any from our own state if there's matches
    // e.g. AABEY AA
    //      AALLY AA
    if state_code != city_code {
        // print!("city = {} state = {}", city_code, state_code);
        let found = state_to_city_codes
            .get(&(city_code, state_code))
            .copied()
            .unwrap_or(0);
        // print!(" {}\n", found);
        found
    } else {
        0
    }
}
