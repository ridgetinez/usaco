use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
    ops::Rem,
};

use cp_macros::competitive_problem;

const ANIMALS: [&str; 12] = [
    "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster", "Dog", "Pig",
    "Rat",
];

fn age_difference(
    parent: String,
    child: String,
    direction: i64,
    animal_map: &HashMap<String, String>,
) -> i64 {
    let starting_index = ANIMALS
        .iter()
        .position(|&s| s == animal_map.get(&parent).unwrap())
        .unwrap() as i64;
    let destination_index = ANIMALS
        .iter()
        .position(|&s| s == animal_map.get(&child).unwrap())
        .unwrap() as i64;

    // Must change by at least one year, and handles when the two have the same lunar animal.
    // let mut year = starting_index + direction;
    let mut shift = direction;
    // println!(
    //     "start = ({},{}), end = ({},{})",
    //     parent, year, child, destination_index
    // );
    while (starting_index + shift).rem_euclid(ANIMALS.len() as i64) != destination_index {
        // println!("{}", year);
        // year += direction;
        shift += direction;
    }
    shift

    // (n + pi) % n - ci
    // int change = r.prev ? -1 : 1;
    // -1
    //
    // +change because it has to be at least 1 year off
    // int year = birthYears[r.relative] + change;
    // year = -1
    //
    // while (mod(year, ZODIAC.size()) != r.year) { year += change; }
    // -1 % 12 != 3 { year += -1 }
    //
    // birthYears[r.name] = year;

    // if direction < 0 {
    //     if pi > ci {
    //         (ci as i64) - (pi as i64)
    //     } else {
    //         -((pi + ANIMALS.len() - ci) as i64)
    //     }
    // } else {
    //     if pi >= ci {
    //         (ci + ANIMALS.len() - pi) as i64
    //     } else {
    //         (ci as i64) - (pi as i64)
    //     }
    // }
}

fn dfs(
    animal_map: &HashMap<String, String>,
    ages: &mut HashMap<String, i64>,
    tree: &HashMap<String, Vec<(i64, String)>>,
    start: String,
) {
    // println!("'{}'", start);
    let starting_age_delta: i64 = ages.get(&start).unwrap().clone();

    for (direction, child) in tree.get(&start).unwrap_or(&Vec::new()) {
        ages.insert(
            child.clone(),
            starting_age_delta
                + age_difference(start.clone(), child.clone(), *direction, animal_map),
        );
        dfs(animal_map, ages, tree, child.clone())
    }
}

fn alternative_solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    let mut lines = input.lines();
    let _: usize = lines.next().unwrap()?.parse().unwrap();
    let mut ages: HashMap<String, i64> = HashMap::from([("Bessie".to_string(), 0)]);
    let mut animals: HashMap<String, String> =
        HashMap::from([("Bessie".to_string(), "Ox".to_string())]);
    lines.for_each(|maybe_line| {
        let line = maybe_line.unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let child = tokens[0].to_string();
        let parent = tokens.last().unwrap().to_string();
        animals.insert(child.clone(), tokens[4].to_string());
        let direction = if tokens[3] == "next" { 1 } else { -1 };
        let starting_age_delta: i64 = ages.get(&parent).unwrap().clone();
        ages.insert(
            child.clone(),
            starting_age_delta + age_difference(parent, child, direction, &animals),
        );
    });

    // println!("{:?}", ages);
    // println!("{:?}", animals);

    _ = writeln!(output, "{}", ages.get("Elsie").unwrap().abs());
    Ok(())
}

#[competitive_problem(input = "stdin", output = "stdout")]
fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
    alternative_solve(input, output)
    // let mut lines = input.lines();
    // let _: usize = lines.next().unwrap()?.parse().unwrap();
    // let mut differ_ages: HashMap<String, i64> = HashMap::from([("Bessie".to_string(), 0)]);
    // let mut animal_map: HashMap<String, String> =
    //     HashMap::from([("Bessie".to_string(), "Ox".to_string())]);
    // let mut m: HashMap<String, Vec<(i64, String)>> = HashMap::new();
    // lines.for_each(|maybe_line| {
    //     let mut line = maybe_line.unwrap();
    //     line = line.trim().to_string();
    //     let tokens: Vec<&str> = line.split_whitespace().collect();

    //     let (cow_name_fst, rest) = line.split_once(" born in ").unwrap();
    //     let (direction, rest) = rest.split_once(" ").unwrap();
    //     let (animal, rest) = rest.split_once(" ").unwrap();
    //     let (_, cow_name_snd) = rest.split_once("year from ").unwrap();
    //     animal_map.insert(cow_name_fst.to_string(), animal.to_string());
    //     m.entry(cow_name_snd.to_string()).or_default().push((
    //         if direction == "next" { 1 } else { -1 },
    //         cow_name_fst.to_string(),
    //     ));
    // });
    // // println!("{:?}", m);
    // // println!("{:?}", differ_ages);
    // // println!("{:?}", animal_map);
    // dfs(&animal_map, &mut differ_ages, &m, "Bessie".to_string());
    // // println!("{:?}", differ_ages);
    // _ = writeln!(output, "{}", differ_ages.get("Elsie").unwrap().abs());
    // Ok(())
}
