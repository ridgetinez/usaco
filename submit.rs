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
    pub mod printing {
        pub fn print_grid<T: std::fmt::Debug>(grid: &Vec<Vec<T>>) {
            grid.iter()
                .for_each(|row| {
                    ::std::io::_print(format_args!("{0:?}\n", row));
                });
        }
    }
}
pub mod outofplace {
    use std::{
        io::{self, BufRead, Write},
        mem::swap,
    };
    fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
        let mut lines = input.lines();
        _ = lines.next();
        let mut heights: Vec<usize> = lines
            .map(|maybe_line| maybe_line.unwrap().parse().unwrap())
            .collect();
        let mut nswaps = 0;
        for i in (0..heights.len()).rev() {
            if let Some(swap_index) = (0..i)
                .rev()
                .max_by_key(|&j| heights[j])
                .filter(|&j| heights[i] < heights[j])
            {
                heights.swap(i, swap_index);
                nswaps += 1;
            }
        }
        _ = output.write_fmt(format_args!("{0}\n", nswaps));
        Ok(())
    }
    pub fn run_problem() {
        use std::{fs::File, io::{self, BufRead, BufReader, Write}};
        let input_source = "outofplace.in";
        let output_source = "outofplace.out";
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
    outofplace::run_problem();
}

