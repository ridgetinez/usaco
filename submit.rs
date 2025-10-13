pub mod util {
    pub mod parsing {
        pub fn parse_line<T: std::str::FromStr + Default>(
            lines: &mut impl Iterator<Item = std::io::Result<String>>,
        ) -> std::io::Result<Vec<T>> {
            Ok(
                lines
                    .next()
                    .unwrap()?
                    .split_whitespace()
                    .map(|s| s.parse().unwrap_or_default())
                    .collect::<Vec<T>>(),
            )
        }
    }
}
pub mod breedflip {
    use std::io::{self, BufRead, Write};
    fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
        let mut lines = input.lines();
        _ = lines.next();
        let original = lines.next().unwrap()?;
        let mutated = lines.next().unwrap()?;
        let mut nflips = 0;
        let mut ndifferent = 0;
        for (co, cm) in original.chars().zip(mutated.chars()) {
            if co == cm && ndifferent > 0 {
                nflips += 1;
                ndifferent = 0;
            } else if co != cm {
                ndifferent += 1
            }
        }
        if ndifferent > 0 {
            nflips += 1;
        }
        output.write_fmt(format_args!("{0}\n", nflips));
        Ok(())
    }
    pub fn run_problem() {
        use std::{fs::File, io::{self, BufRead, BufReader, Write}};
        let input_source = "breedflip.in";
        let output_source = "breedflip.out";
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
    breedflip::run_problem();
}

