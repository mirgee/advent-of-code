use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, line_ending, multispace1, newline},
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    L,
    R,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<&str, (&str, &str)>)> {
    let (input, directions) = many1(alt((
        char('L').map(|_| Direction::L),
        char('R').map(|_| Direction::R),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    tag(")"),
                ),
            ),
            line_ending,
        ),
        HashMap::new,
        |mut acc: HashMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;
    Ok((input, (directions, map)))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, (directions, map)) = parse_input(input).unwrap();
    let mut directions_cycle = directions.iter().cycle();
    let mut current: Vec<&str> = map.keys().filter(|k| k.ends_with('A')).cloned().collect();
    let mut num_steps = 0;
    while !current.iter().all(|pos| pos.ends_with('Z')) {
        let direction = directions_cycle.next().unwrap();
        current = current
            .par_iter()
            .map(|pos| match direction {
                Direction::L => map[pos].0,
                Direction::R => map[pos].1,
            })
            .collect();
        num_steps += 1;
    }
    Ok(num_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
