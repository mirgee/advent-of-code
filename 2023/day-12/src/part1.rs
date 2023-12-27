use std::{collections::HashMap, fmt::Display, ops::Range};

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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Broken => write!(f, "#"),
            Item::Functional => write!(f, "."),
            Item::Unknown => write!(f, "?"),
        }
    }
}

struct Printables<'a>(&'a [Item]);

impl Display for Printables<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.0.iter() {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
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

fn satisfiable(line: &[Item], start: usize, end: usize) -> bool {
    // Not if out of bounds
    // Note that we can't start the beginning because there is always ending of a contiguous group
    // right next to it
    if start < 1 || end >= line.len() {
        return false;
    }

    // Not if neighbored by a #
    if line[start - 1] == Item::Broken || (end + 1 < line.len() && line[end + 1] == Item::Broken) {
        return false;
    }

    // Not if overlaps a .
    if line[start..=end]
        .iter()
        .any(|&item| item == Item::Functional)
    {
        return false;
    }
    // Not if skips a group
    if line[..start].iter().any(|&item| item == Item::Broken) {
        return false;
    }
    return true;
}

fn backtrack_arrangements(
    line: &[Item],
    contiguous_counts: &[u64],
    cache: &mut HashMap<(Vec<Item>, Vec<u64>), u64>,
) -> u64 {
    let key = (line.to_vec(), contiguous_counts.to_vec());
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    if contiguous_counts.len() == 0 {
        if line.iter().contains(&Item::Broken) {
            return 0;
        } else {
            return 1;
        }
    }

    let group_size = contiguous_counts[0] as usize;
    let mut num_arrangements = 0;

    if group_size > line.len() {
        return 0;
    }

    for start in 0..=line.len() - group_size {
        let end = start + group_size - 1;
        if satisfiable(line, start, end) {
            // println!(
            //     "Found satisfiable range {}..={} on line {} with contiguous counts {:?}",
            //     start,
            //     end,
            //     Printables(line),
            //     contiguous_counts
            // );
            let res = backtrack_arrangements(&line[end + 1..], &contiguous_counts[1..], cache);
            // println!(
            //     "Found {} arrangements on line {} with contiguous counts {:?} and range {}..={}",
            //     res,
            //     Printables(line),
            //     contiguous_counts,
            //     start,
            //     end
            // );
            num_arrangements += res;
        }
    }
    cache.insert(key, num_arrangements);
    return num_arrangements;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, output) = parse_input(input).unwrap();
    let mut arrangements_count_total = 0;
    let mut cache = HashMap::new();
    for (mut line, contiguous_counts) in output {
        line.insert(0, Item::Functional);
        arrangements_count_total +=
            backtrack_arrangements(&mut line, &contiguous_counts, &mut cache);
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
        21
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
