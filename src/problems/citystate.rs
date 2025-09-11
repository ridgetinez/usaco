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
    let (n_special_state_pairs, _) = lines.fold(
        (0, HashMap::<String, HashMap<String, usize>>::new()),
        |(found_count, mut acc), line| {
            let [city, state_code] = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string()) // check if we can remove this allocation (something about use-after-free)
                .collect::<Vec<String>>()
                .try_into()
                .unwrap();
            (
                found_count
                    + n_special_state_pairs_made(&mut acc, state_code, city[..2].to_string()),
                acc,
            )
        },
    );
    _ = writeln!(output, "{}", n_special_state_pairs);
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
    state_to_city_codes: &mut HashMap<String, HashMap<String, usize>>,
    state_code: String,
    city_code: String,
) -> usize {
    // don't count any from our own state if there's matches
    // e.g. AABEY AA
    //      AALLY AA
    let a = state_to_city_codes
        .get(&city_code)
        .map(|m| m.get(&state_code).map(|count| *count).unwrap_or(0))
        .unwrap_or(0);
    let b = if state_code == city_code {
        state_to_city_codes
            .get(&state_code)
            .map(|m| m.get(&city_code).map(|count| *count).unwrap_or(0))
            .unwrap_or(0)
    } else {
        0
    };
    // println!("{} - {}", a, b);
    let npairs_made = a - b;
    *state_to_city_codes
        .entry(state_code)
        .or_default()
        .entry(city_code)
        .or_default() += 1;
    npairs_made
}
