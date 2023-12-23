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
    let mut current: Vec<(&str, u64, bool)> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|pos| (*pos, 0, false))
        .collect();
    // println!("{:?}", current);
    while !current.iter().all(|(_, _, did_finish)| did_finish == &true) {
        let direction = directions_cycle.next().unwrap();
        current = current
            .iter()
            // .skip_while(|(_, _, did_finish)| did_finish == &true)
            .map(|(pos, steps, did_finish)| {
                if did_finish == &true {
                    return (*pos, *steps, *did_finish);
                }
                let steps = steps + 1;
                let next_pos = match direction {
                    Direction::L => map.get(pos).unwrap().0,
                    Direction::R => map.get(pos).unwrap().1,
                };
                let did_finish = next_pos.ends_with('Z');
                (next_pos, steps, did_finish)
            })
            .collect();
        // println!("{:?}", current);
    }
    let num_steps = lcm(&current
        .iter()
        .map(|(_, steps, _)| *steps)
        .collect::<Vec<u64>>());
    Ok(num_steps)
}

fn lcm(nums: &[u64]) -> u64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }

    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
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
