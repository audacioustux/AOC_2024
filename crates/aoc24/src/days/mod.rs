use std::{fmt::Display, fs, path::Path, time::Instant};

use anyhow::{Context, Result, anyhow};
use winnow::{PResult, Parser as _};

pub mod day01;

pub trait Day {
    type Input;

    fn parser(input_string: &mut &str) -> PResult<Self::Input>;

    type Output1: Display;

    fn part_1(input: &Self::Input) -> Self::Output1;

    type Output2: Display;

    fn part_2(input: &Self::Input) -> Self::Output2;

    fn parse_file(path: impl AsRef<Path>) -> Result<Self::Input> {
        let input_string = fs::read_to_string(path).context("reading the input file")?;
        let input_string = input_string.trim();
        let input = Self::parser
            .parse(&input_string)
            .map_err(|e| anyhow!(e.to_string()))
            .context("running the parser")?;
        Ok(input)
    }

    fn run_day(path: impl AsRef<Path>) -> Result<()> {
        let input = Self::parse_file(path)?;

        let before1 = Instant::now();
        println!("Part 1: {}", Self::part_1(&input));
        println!("Part 1 took {:?}", before1.elapsed());

        let before2 = Instant::now();
        println!("Part 2: {}", Self::part_2(&input));
        println!("Part 2 took {:?}", before2.elapsed());

        Ok(())
    }
}
