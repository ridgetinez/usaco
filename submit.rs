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
pub mod evenmoreodd {
    use std::io::{self, BufRead, Write};
    use super::util::parsing::parse_line;
    fn solve(input: Box<dyn BufRead>, mut output: Box<dyn Write>) -> io::Result<()> {
        let mut lines = input.lines();
        let _ = lines.next();
        let ids: Vec<usize> = parse_line(&mut lines)?;
        let (evens, odds): (Vec<usize>, Vec<usize>) = ids
            .into_iter()
            .partition(|n| n % 2 == 0);
        let (nevens, nodds) = (evens.len(), odds.len());
        if nevens >= nodds {
            _ = output
                .write_fmt(
                    format_args!(
                        "{0}\n",
                        (2 * nodds) + if nevens > nodds { 1 } else { 0 },
                    ),
                );
        } else {
            let remaining_odds = nodds - nevens;
            _ = output
                .write_fmt(
                    format_args!(
                        "{0}\n",
                        2 * nevens
                            + match remaining_odds % 3 {
                                0 => 2 * (remaining_odds / 3),
                                1 => 2 * (remaining_odds / 3) - 1,
                                2 => 2 * (remaining_odds / 3) + 1,
                                _ => {
                                    ::core::panicking::panic_fmt(format_args!("wtf"));
                                }
                            },
                    ),
                );
        }
        Ok(())
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
    evenmoreodd::run_problem();
}

