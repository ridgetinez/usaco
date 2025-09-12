pub mod citystate {
    use std::{collections::HashMap, io::{self, BufRead, Write}};
    fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
        let mut lines = input.lines();
        let _: usize = lines.next().unwrap()?.parse().unwrap();
        let mut pairs: Vec<(String, String)> = Vec::new();
        let mut occurrences: HashMap<(String, String), usize> = HashMap::new();
        for line in lines.map(|maybe_line| maybe_line.unwrap()) {
            let [city, state_code] = line
                .split(' ')
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
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
            .sum::<usize>() / 2;
        _ = output.write_fmt(format_args!("{0}\n", special_pair_count));
        Ok(())
    }
    pub fn run_problem() {
        use std::{fs::File, io::{self, BufRead, BufReader, Write}};
        let input_source = "citystate.in";
        let output_source = "citystate.out";
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
    fn n_special_state_pairs_made(
        state_to_city_codes: &HashMap<(String, String), usize>,
        state_code: String,
        city_code: String,
    ) -> usize {
        if state_code != city_code {
            let found = state_to_city_codes
                .get(&(city_code, state_code))
                .unwrap_or(&0)
                .clone();
            found
        } else {
            0
        }
    }
}

fn main() {
    citystate::run_problem();
}

