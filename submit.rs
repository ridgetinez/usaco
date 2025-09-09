pub mod yearofthecow {
    use std::{
        collections::HashMap, io::{self, BufRead, Write},
        ops::Rem,
    };
    const ANIMALS: [&str; 12] = [
        "Ox",
        "Tiger",
        "Rabbit",
        "Dragon",
        "Snake",
        "Horse",
        "Goat",
        "Monkey",
        "Rooster",
        "Dog",
        "Pig",
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
        let mut shift = direction;
        while (starting_index + shift).rem_euclid(ANIMALS.len() as i64)
            != destination_index
        {
            shift += direction;
        }
        shift
    }
    fn dfs(
        animal_map: &HashMap<String, String>,
        ages: &mut HashMap<String, i64>,
        tree: &HashMap<String, Vec<(i64, String)>>,
        start: String,
    ) {
        let starting_age_delta: i64 = ages.get(&start).unwrap().clone();
        for (direction, child) in tree.get(&start).unwrap_or(&Vec::new()) {
            ages.insert(
                child.clone(),
                starting_age_delta
                    + age_difference(
                        start.clone(),
                        child.clone(),
                        *direction,
                        animal_map,
                    ),
            );
            dfs(animal_map, ages, tree, child.clone())
        }
    }
    fn alternative_solve(
        input: Box<dyn BufRead>,
        mut output: Box<dyn Write>,
    ) -> io::Result<()> {
        let mut lines = input.lines();
        let _: usize = lines.next().unwrap()?.parse().unwrap();
        let mut ages: HashMap<String, i64> = HashMap::from([("Bessie".to_string(), 0)]);
        let mut animals: HashMap<String, String> = HashMap::from([
            ("Bessie".to_string(), "Ox".to_string()),
        ]);
        lines
            .for_each(|maybe_line| {
                let line = maybe_line.unwrap();
                let tokens: Vec<&str> = line.split_whitespace().collect();
                let child = tokens[0].to_string();
                let parent = tokens.last().unwrap().to_string();
                animals.insert(child.clone(), tokens[4].to_string());
                let direction = if tokens[3] == "next" { 1 } else { -1 };
                let starting_age_delta: i64 = ages.get(&parent).unwrap().clone();
                ages.insert(
                    child.clone(),
                    starting_age_delta
                        + age_difference(parent, child, direction, &animals),
                );
            });
        _ = output.write_fmt(format_args!("{0}\n", ages.get("Elsie").unwrap().abs()));
        Ok(())
    }
    fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
        alternative_solve(input, output)
    }
    pub fn run_problem() {
        use std::{fs::File, io::{self, BufRead, BufReader, Write}};
        let input_source = "stdin";
        let output_source = "stdout";
        let mut reader: Box<dyn BufRead> = Box::new(io::stdin().lock());
        if input_source != "stdin" {
            let f = File::open(input_source).unwrap();
            reader = Box::new(BufReader::new(f));
        }
        let mut writer: Box<dyn Write> = Box::new(io::stdout());
        if output_source != "stdout" {
            let f = File::create(output_source).unwrap();
            writer = Box::new(f);
        }
        solve(reader, writer).unwrap();
    }
}

fn main() {
    yearofthecow::run_problem();
}

