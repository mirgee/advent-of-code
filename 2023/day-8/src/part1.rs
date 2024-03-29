use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, multispace1, newline},
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

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
                alpha1,
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(alpha1, tag(", "), alpha1),
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
    let mut current = "AAA";
    let mut num_steps = 0;
    for direction in directions.iter().cycle() {
        num_steps += 1;
        let (left, right) = map.get(current).unwrap();
        match direction {
            Direction::L => current = left,
            Direction::R => current = right,
        }
        if current == "ZZZ" {
            break;
        }
    }
    Ok(num_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_0() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
