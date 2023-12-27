use std::collections::HashMap;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

fn is_valid(line: &[Item], contiguous_counts_expected: &[u64]) -> bool {
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
    idx: usize,
    contiguous_counts: &[u64],
    cache: &mut HashMap<(Vec<Item>, Vec<u64>), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&(line.clone(), contiguous_counts.to_vec())) {
        return cached;
    }

    if idx >= line.len() {
        return is_valid(line, contiguous_counts) as u64;
    }

    if line[idx] != Item::Unknown {
        return backtrack_arrangements(line, idx + 1, contiguous_counts, cache);
    }

    let mut arrangements_count = 0;
    for &item in &[Item::Functional, Item::Broken] {
        line[idx] = item;
        arrangements_count += backtrack_arrangements(line, idx + 1, contiguous_counts, cache);
    }

    line[idx] = Item::Unknown;
    cache.insert(
        (line.clone(), contiguous_counts.to_vec()),
        arrangements_count,
    );

    arrangements_count
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, output) = parse_input(input).unwrap();
    let mut arrangements_count_total = 0;
    let mut cache = HashMap::new();
    for (mut line, contiguous_counts) in output {
        line.push(Item::Unknown);
        let mut repeating_line = line.repeat(5);
        repeating_line.pop();
        let repeating_contiguous_counts = contiguous_counts.repeat(5);
        arrangements_count_total += backtrack_arrangements(
            &mut repeating_line,
            0,
            &repeating_contiguous_counts,
            &mut cache,
        );
    }
    Ok(arrangements_count_total)
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
        525152
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
