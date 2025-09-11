pub mod citystate {
    use std::{collections::HashMap, io::{self, BufRead, Write}};
    fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
        let mut lines = input.lines();
        let _: usize = lines.next().unwrap()?.parse().unwrap();
        let (n_special_state_pairs, _) = lines
            .fold(
                (0, HashMap::<String, HashMap<String, usize>>::new()),
                |(found_count, mut acc), line| {
                    let [city, state_code] = line
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .try_into()
                        .unwrap();
                    (
                        found_count
                            + n_special_state_pairs_made(
                                &mut acc,
                                state_code,
                                city[..2].to_string(),
                            ),
                        acc,
                    )
                },
            );
        _ = output.write_fmt(format_args!("{0}\n", n_special_state_pairs));
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
        state_to_city_codes: &mut HashMap<String, HashMap<String, usize>>,
        state_code: String,
        city_code: String,
    ) -> usize {
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
        let npairs_made = a - b;
        *state_to_city_codes.entry(state_code).or_default().entry(city_code).or_default()
            += 1;
        npairs_made
    }
}

fn main() {
    citystate::run_problem();
}

