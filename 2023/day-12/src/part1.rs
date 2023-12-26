use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete,
    character::complete::{line_ending, u64},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Item {
    Broken,
    Functional,
    Unknown,
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<Item>, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            many1(alt((
                complete::tag(".").map(|_| Item::Functional),
                complete::tag("#").map(|_| Item::Broken),
                complete::tag("?").map(|_| Item::Unknown),
            ))),
            complete::tag(" "),
            separated_list1(complete::tag(","), u64),
        ),
    )(input)
}

fn is_valid(line: &Vec<Item>, contiguous_counts_expected: &Vec<u64>) -> bool {
    let groups = line.into_iter().group_by(|key| key == &&Item::Broken);
    let contiguous_counts_actual = groups
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.count() as u64)
        .collect::<Vec<_>>();
    contiguous_counts_expected == &contiguous_counts_actual
}

fn backtrack_arrangements(
    line: &mut Vec<Item>,
    contiguous_counts: &Vec<u64>,
    arrangements_count: u64,
) -> u64 {
    let first_unknown_idx = line.iter().position(|&x| x == Item::Unknown);
    if let Some(idx) = first_unknown_idx {
        let mut arrangements_count = arrangements_count;
        for item in vec![Item::Functional, Item::Broken] {
            let prev_item = line[idx];
            line[idx] = item;
            arrangements_count =
                backtrack_arrangements(line, contiguous_counts, arrangements_count);
            line[idx] = prev_item;
        }
        arrangements_count
    } else {
        if is_valid(line, &contiguous_counts) {
            arrangements_count + 1
        } else {
            arrangements_count
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, output) = parse_input(input).unwrap();
    let mut arrangements_count = 0;
    for (mut line, contiguous_counts) in output {
        arrangements_count =
            backtrack_arrangements(&mut line, &contiguous_counts, arrangements_count);
    }
    Ok(arrangements_count)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        21
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("#.#.### 1,1,3", true)]
    #[case("##..### 1,1,3", false)]
    #[case("#...### 1,1,3", false)]
    #[case("....### 1,1,3", false)]
    #[test_log::test]
    fn test_valid(#[case] input: &str, #[case] output: bool) -> miette::Result<()> {
        let (line, contiguous_counts) = parse_input(input).unwrap().1[0].clone();
        assert_eq!(output, is_valid(&line, &contiguous_counts));
        Ok(())
    }
}
