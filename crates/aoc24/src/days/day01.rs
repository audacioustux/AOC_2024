use itertools::Itertools;
use winnow::{
    PResult, Parser,
    ascii::{digit1, newline, space1},
    combinator::{separated, separated_pair},
};

use crate::days::Day;

pub struct Day01;

pub struct Numbers {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse_line(input: &mut &str) -> PResult<(i32, i32)> {
    separated_pair(digit1.parse_to(), space1, digit1.parse_to()).parse_next(input)
}

impl Day for Day01 {
    type Input = Numbers;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let lines: Vec<_> = separated(0.., parse_line, newline).parse_next(input)?;
        let (left, right) = lines.into_iter().unzip();
        Ok(Numbers { left, right })
    }

    type Output1 = u32;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .left
            .iter()
            .sorted()
            .zip(input.right.iter().sorted())
            .map(|(l, r)| (l - r).unsigned_abs())
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let counts = input.right.iter().counts();
        input
            .left
            .iter()
            .map(|l| *l as usize * counts.get(l).unwrap_or(&0))
            .sum()
    }
}
